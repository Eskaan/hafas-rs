//! Functions for the cli sub-command `data parse_heatmap`

use progress_bar::{Color, Style};
use sqlx::PgConnection;

use crate::{parse_raw_jids::OpDays, util::database};

/// This function is called by the cli sub-command `data parse_heatmap`.
///
/// It queries all jids from the main `trips` table and parses them into the 
/// `location_counts` table for faster access in the `create_heatmap` sub-command.
pub async fn parse_count() {
    let mut conn = database::connect().await;
    let from = sqlx::query!("SELECT MIN(jid) FROM lookup_data.trips")
        .fetch_one(&mut conn)
        .await
        .expect("Start fetch failed")
        .min
        .unwrap();
    let to = sqlx::query!("SELECT MAX(jid) FROM lookup_data.trips")
        .fetch_one(&mut conn)
        .await
        .expect("Start fetch failed")
        .max
        .unwrap();
    let drop = sqlx::query!("DELETE FROM lookup_data.location_counts")
        .execute(&mut conn)
        .await
        .expect("Delete failed")
        .rows_affected();
    progress_bar::init_progress_bar(to as usize);
    progress_bar::print_progress_bar_info(
        "Dropped",
        format!("{drop} old entries").as_str(),
        Color::White,
        Style::Bold,
    );
    progress_bar::set_progress_bar_action("Parsing", Color::Blue, Style::Bold);

    for jid in from..to {
        let resp = sqlx::query!(
            r#"SELECT lookup_data.get_evas(stops),cat_code,cat_out,op_days as "op_days: Vec<OpDays>" FROM lookup_data.trips WHERE jid = $1"#,
            jid
        )
        .fetch_one(&mut conn)
        .await
        .expect("Fetch failed");

        for (i, eva) in resp.get_evas.unwrap().into_iter().enumerate() {
            insert_or_nothing(&mut conn, eva, resp.cat_code, &resp.cat_out).await;
            let count = resp
                .op_days
                .iter()
                .filter(|&days| days.from_loc <= i as i32 && days.to_loc >= i as i32)
                .fold(0, |acc, days| acc + days.dates.len());
            increment_count(&mut conn, eva, resp.cat_code, &resp.cat_out, count).await;
        }
        if jid % 10 == 0 {
            progress_bar::set_progress_bar_progression(jid as usize);
        }
    }
    progress_bar::finalize_progress_bar();
}

/// Tries to insert a new entry into the `location_counts` table, does nothing if the primary key already exists.
pub async fn insert_or_nothing(conn: &mut PgConnection, eva: i32, cat_code: i16, cat_out: &str) {
    sqlx::query!(
        "INSERT INTO lookup_data.location_counts VALUES ($1,$2,$3,$4,0) ON CONFLICT DO NOTHING",
        eva,
        cat_code,
        cat_out,
        get_eva_name(conn, eva).await
    )
    .execute(conn)
    .await
    .expect("Insert failed");
}

/// Increments the count of a specific entry in the `location_counts` table by `count`.
pub async fn increment_count(
    conn: &mut PgConnection,
    eva: i32,
    cat_code: i16,
    cat_out: &str,
    count: usize,
) {
    sqlx::query!(
        "UPDATE lookup_data.location_counts SET count = count + $4 WHERE eva = $1 AND cat_code = $2 AND cat_out = $3",
        eva,
        cat_code,
        cat_out,
        count as i32,
    )
    .execute(conn)
    .await
    .expect("Increment failed.");
}

/// Fetches the `name` for a `eva` from the `locations` table.
pub async fn get_eva_name(conn: &mut PgConnection, eva: i32) -> String {
    sqlx::query!("SELECT name FROM lookup_data.locations WHERE eva = $1", eva)
        .fetch_one(conn)
        .await
        .expect("Eva does not exist.")
        .name
}
