//! Misc functions to request raw HAFAS journey details.

use std::{collections::VecDeque, time::Duration};

use hafas_wrap::{hafas_profiles, methods::JourneyDetailsRequest};
use progress_bar::{Color, Style};
use serde_json::Value;
use sqlx::{Connection, PgConnection};
use tokio::time::Instant;

use crate::{parse_raw_jids, util::logging::LogLevel};

/// This function is called by the cli sub-command `data request_raw`.
/// 
/// It creates the [`hafas_wrap::Client`], [`connect`](crate::util::database::connect)s to the database, and requests raw data chunk-wise.
/// # Panics
/// Use of unstable function [`query_and_request`].
pub async fn request(
    from: &usize,
    to: &usize,
    chunk_size: &usize,
    continue_at_max: bool,
    update: bool,
    parse: bool,
    log_level: &LogLevel,
) {
    let client = hafas_wrap::Client::new();
    let mut conn = crate::util::database::connect().await;
    progress_bar::init_progress_bar(0);
    let start = if continue_at_max {
        sqlx::query!("SELECT MAX(jid) FROM lookup_data.raw_data")
            .fetch_one(&mut conn)
            .await
            .unwrap()
            .max
            .unwrap_or(0) as usize
            + 1
    } else {
        *from
    };

    progress_bar::set_progress_bar_max(*to);
    progress_bar::set_progress_bar_action("Fetching", Color::Blue, Style::Bold);

    for i in (start / chunk_size)..(to / chunk_size) {
        query_and_request(
            &mut conn,
            &client,
            update,
            parse,
            log_level,
            i * chunk_size,
            i * chunk_size + chunk_size,
        )
        .await;
    }
    query_and_request(
        &mut conn,
        &client,
        update,
        parse,
        log_level,
        to - ((to - start) % chunk_size),
        *to,
    )
    .await;

    conn.close();
    progress_bar::finalize_progress_bar();
}

/// This function checks a single chunk against the database and queries the missing entries.
/// # Panics
/// Use of unstable function [`make_request`].
async fn query_and_request(
    conn: &mut PgConnection,
    client: &hafas_wrap::Client,
    update: bool,
    parse: bool,
    log_level: &LogLevel,
    from: usize,
    to: usize,
) {
    let mut request_jids = Vec::new();
    for jid in from..to {
        if update || !jid_exists(conn, jid).await {
            if log_level <= &LogLevel::Trace {
                progress_bar::print_progress_bar_info(
                    "Check",
                    format!("{jid} does not exist, requesting").as_str(),
                    Color::LightGreen,
                    Style::Dim,
                );
            }
            request_jids.push(jid);
        } else if log_level <= &LogLevel::Trace {
            progress_bar::print_progress_bar_info(
                "Check",
                format!("{jid} exists, not requesting").as_str(),
                Color::LightGreen,
                Style::Dim,
            );
        }
    }

    if !request_jids.is_empty() {
        let jids_len = request_jids.len();
        let times = make_request(client, conn, request_jids, parse).await;
        if log_level <= &LogLevel::Info {
            progress_bar::print_progress_bar_info(
                "Check",
                format!(
                    "{}-{}, {} requested. req:{}ms db:{}ms",
                    from,
                    to,
                    jids_len,
                    times.0.as_millis(),
                    times.1.as_millis()
                )
                .as_str(),
                Color::Green,
                Style::Normal,
            );
        }
    }
    progress_bar::set_progress_bar_progression(to);
}

/// This function requests a single chunk from HAFAS and inserts it into the database.
/// # Panics
/// It uses the unstable function [`make_raw_request`], which panics on any error.
/// 
/// Also, function uses [`unwrap`](core::option::Option::unwrap) to pop the front of an array in a loop. 
/// This only panics if HAFAS has returned less responses than requests made (which should not happen).
async fn make_request(
    client: &hafas_wrap::Client,
    conn: &mut sqlx::PgConnection,
    jids: Vec<usize>,
    parse: bool,
) -> (Duration, Duration) {
    let req_time = Instant::now();

    let raw_res = make_raw_request(client, &jids).await;

    let mut res = get_request_bodys(&raw_res);

    let req_elapsed = req_time.elapsed();
    let db_time = Instant::now();

    //Iterating over all jids, pop'ing the first element in the Vec each round.
    for jid in jids {
        database_insert(&mut *conn, jid, res.front().unwrap()).await;

        if parse {
            parse_raw_jids::parse_entry(jid as i32, res.pop_front().unwrap(), conn).await;
        } else {
            res.pop_front();
        }
    }

    (db_time.elapsed(), req_elapsed)
}

/// This function requests a single chunk from HAFAS ans returns the result.
/// # Panics
/// It panics on any error, as this is a cli program that has no real error handling in place.
pub async fn make_raw_request(client: &hafas_wrap::Client, jids: &[usize]) -> Value {
    serde_json::from_str(
        &client
            .request_raw(
                &hafas_profiles::DB,
                jids.iter()
                    .map(|i| {
                        JourneyDetailsRequest {
                            jid: format!("1|{i}|0|80|-1"),
                            ..Default::default()
                        }
                        .into()
                    })
                    .collect(),
            )
            .await
            .unwrap(),
    )
    .unwrap()
}

/// Tries to get the bodys of a request response.
/// # Panics
/// if the response body does not exist or the request returned an error.
pub fn get_request_bodys(res: &Value) -> VecDeque<Value> {
    assert!(
        !(res.get("err") != Some(&Value::String(String::from("OK")))),
        "{} {}",
        res.get("err").unwrap().as_str().unwrap_or_default(),
        res.get("errTxt")
            .unwrap_or(&Value::Null)
            .as_str()
            .unwrap_or_default()
    );

    res.get("svcResL")
        .unwrap_or(&Value::Array(vec![]))
        .as_array()
        .unwrap()
        .iter()
        .map(|r| r.get("res").unwrap().clone())
        .collect::<VecDeque<Value>>()
}

/// Inserts a given `jid` and `value` into the `raw_data` table on the database.
async fn database_insert(conn: &mut sqlx::PgConnection, jid: usize, value: &Value) {
    sqlx::query!(
        "INSERT INTO lookup_data.raw_data VALUES ($1,$2) ON CONFLICT DO NOTHING",
        jid as i32,
        value,
    )
    .execute(conn)
    .await
    .unwrap();
}

/// Checks if a given `jid` exists on the `raw_data` table on the database.
async fn jid_exists(conn: &mut sqlx::PgConnection, jid: usize) -> bool {
    sqlx::query!(
        "SELECT COUNT(*) FROM lookup_data.raw_data WHERE jid = $1",
        jid as i32
    )
    .fetch_one(conn)
    .await
    .unwrap()
    .count
    .unwrap()
        == 1
}
