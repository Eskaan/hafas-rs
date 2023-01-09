//! Functions for the cli sub-command `data parse`

use progress_bar::{Color, Style};
use serde_json::Value;
use sqlx::{
    postgres::{PgHasArrayType, PgTypeInfo},
    Connection, PgConnection,
};
use time::{Date, Instant, Time};

use crate::util::logging::LogLevel;

/// Database type `operation_dates`.
#[derive(sqlx::Type, Debug)]
#[sqlx(type_name = "operation_dates")]
pub struct OpDays {
    pub dates: Vec<Date>,
    pub from_loc: i32,
    pub to_loc: i32,
    pub info: Option<String>,
}

impl PgHasArrayType for OpDays {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        PgTypeInfo::with_name("_operation_dates")
    }
}

/// Database type `scheduled_stop`
#[derive(sqlx::Type)]
#[sqlx(type_name = "scheduled_stop")]
pub struct ScheduledStop {
    eva: i32,
    scheduled_arrival: Option<Time>,
    scheduled_departure: Option<Time>,
}

impl PgHasArrayType for ScheduledStop {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        PgTypeInfo::with_name("_scheduled_stop")
    }
}

/// This function is called by the cli sub-command `data parse`.
///
/// It [`connect`](crate::util::database::connect)s to the database and parses all data from the `raw_data` table into usable format.
/// Then this usable data gets inserted back into the tables `trips`, `operators`, train_types` and `locations`.
/// The operation is done in chunks of `chink_size` to not overload the local RAM. It is recommended to try a few different sizes.
pub async fn parse(from: Option<&i32>, to: Option<&i32>, chunk_size: &i32, log_level: &LogLevel) {
    let mut conn = crate::util::database::connect().await;
    progress_bar::init_progress_bar(0);

    let start = *from.unwrap_or(&0);
    let end = if let Some(&val) = to {
        val
    } else {
        let end = sqlx::query!("SELECT MAX(jid) FROM lookup_data.raw_data")
            .fetch_one(&mut conn)
            .await
            .unwrap()
            .max
            .unwrap_or(0);
        progress_bar::print_progress_bar_info(
            "Fetched",
            format!("end position {end}").as_str(),
            Color::LightGreen,
            Style::Dim,
        );
        end
    };

    progress_bar::set_progress_bar_max((end - start) as usize);
    progress_bar::set_progress_bar_action("Parsing", Color::Blue, Style::Bold);

    for i in (start / chunk_size)..(end / chunk_size) {
        parse_chunk(
            &mut conn,
            i * chunk_size,
            i * chunk_size + chunk_size - 1,
            log_level,
        )
        .await;
    }
    parse_chunk(
        &mut conn,
        end - ((end - start) % chunk_size),
        end,
        log_level,
    )
    .await;
    conn.close();
}

/// Queries, parses a single chunk and inserts it back into the database.
/// 
/// See [`parse`] for more details.
async fn parse_chunk(conn: &mut PgConnection, start: i32, end: i32, log_level: &LogLevel) {
    let instant = Instant::now();
    let query = sqlx::query!(
        "SELECT * FROM lookup_data.raw_data WHERE jid >= $1 AND jid <= $2",
        start,
        end
    )
    .fetch_all(&mut *conn)
    .await
    .unwrap();
    if log_level <= &LogLevel::Debug {
        progress_bar::print_progress_bar_info(
            "Fetch",
            format!("done in {}ms", instant.elapsed().whole_milliseconds()).as_str(),
            Color::LightGreen,
            Style::Dim,
        );
    }

    let instant = Instant::now();

    for record in query {
        parse_entry(record.jid, record.raw, conn).await;
    }

    if log_level <= &LogLevel::Debug {
        progress_bar::print_progress_bar_info(
            "Parse",
            format!("done in {}ms", instant.elapsed().whole_milliseconds()).as_str(),
            Color::Green,
            Style::Bold,
        );
    }
}

/// Parses a single [`serde_json::Value`] into a usable format and inserts it into the database.
pub async fn parse_entry(jid: i32, raw: Value, conn: &mut PgConnection) {
    let parsed: hafas_wrap::methods::JourneyDetailsResponse = serde_json::from_value(raw).unwrap();

    for loc in &parsed.common.locL {
        sqlx::query!(
            "INSERT INTO lookup_data.locations VALUES ($1,$2,$3,$4,$5) ON CONFLICT DO NOTHING",
            loc.extId.parse::<i32>().unwrap(),
            loc.name,
            loc.crd.x as i32,
            loc.crd.y as i32,
            loc.crd.z.map(|v| v as i32)
        )
        .execute(&mut *conn)
        .await
        .unwrap();
    }

    for op in &parsed.common.opL {
        sqlx::query!(
            "INSERT INTO lookup_data.operators VALUES ($1) ON CONFLICT DO NOTHING",
            op.name
        )
        .execute(&mut *conn)
        .await
        .unwrap();
    }

    for prod in &parsed.common.prodL {
        match &prod.prodCtx {
            Some(ctx) => {
                sqlx::query!(
                    "INSERT INTO lookup_data.train_types VALUES ($1,$2) ON CONFLICT DO NOTHING",
                    ctx.catCode.parse::<i32>().unwrap(),
                    ctx.catOut.clone().unwrap_or(ctx.catOutL.clone()),
                )
                .execute(&mut *conn)
                .await
                .unwrap();
            }
            None => println!("No prodctx in {jid}"),
        }
    }

    let ctx = parsed.common.prodL[0].prodCtx.clone().unwrap();

    let op_days = parsed
        .journey
        .sDaysL
        .iter()
        .map(|days| OpDays {
            dates: parse_op_days(parse_date(&parsed.fpB), &days.sDaysB),
            from_loc: days.fLocX as i32,
            to_loc: days.tLocX as i32,
            info: days.sDaysR.clone(),
        })
        .collect::<Vec<OpDays>>();

    let stop_list = parsed
        .journey
        .stopL
        .unwrap()
        .iter()
        .map(|s| ScheduledStop {
            eva: parsed.common.locL[s.locX as usize]
                .extId
                .parse::<i32>()
                .unwrap(),
            scheduled_arrival: s.aTimeS.clone().map(|t| parse_time(&t)),
            scheduled_departure: s.dTimeS.clone().map(|t| parse_time(&t)),
        })
        .collect::<Vec<ScheduledStop>>();

    sqlx::query!("INSERT INTO lookup_data.trips (jid,op_days,cat_code,cat_out,name,route,id,admin,operator,stops) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10) ON CONFLICT DO NOTHING",
        jid,
        op_days as _,
        ctx.catCode.parse::<i16>().unwrap(),
        ctx.catOut.unwrap_or(ctx.catOutL),
        parsed.common.prodL[0].addName.clone().unwrap_or(parsed.common.prodL[0].name.clone()),
        ctx.line.or(ctx.lineId.or(ctx.matchId.or(parsed.common.prodL[0].matchId.clone().or(parsed.common.prodL[0].nameS.clone())))),
        ctx.num.unwrap().parse::<i32>().unwrap(),
        ctx.admin,
        parsed.common.prodL[0].oprX.map(|op| &parsed.common.opL.get(op as usize).unwrap().name),
        stop_list as _,
    )
    .execute(&mut *conn)
    .await
    .unwrap();

    progress_bar::inc_progress_bar();
}

/// Parses a date from HAFAS into a [`time::Date`].
fn parse_date(s: &str) -> Date {
    let format = time::format_description::parse("[year][month][day]").unwrap();
    Date::parse(s, &format).unwrap()
}

/// Parses a time from HAFAS into a [`time::Time`]
fn parse_time(s: &str) -> Time {
    let format = match s.len() {
        6 => time::format_description::parse("[hour][minute][second]").unwrap(),
        // If the trip goes over to the next day, a additional day gets added.
        8 => time::format_description::parse("[day][hour][minute][second]").unwrap(),
        _ => panic!("Input has invalid length"),
    };
    Time::parse(s, &format).unwrap()
}

/// Parses the obfuscated String of operation days from HAFAS into a Vec of [`time::Date`].
fn parse_op_days(start: Date, days: &str) -> Vec<Date> {
    days.chars()
        .fold(String::new(), |acc, v| acc + hex_to_binary(v))
        .chars()
        .enumerate()
        .filter(|(_, c)| c == &'1')
        .map(|(i, _)| start.saturating_add(time::Duration::days(i as i64)))
        .collect()
}

/// Parses a hexadecimal char into a four-digit binary String.
fn hex_to_binary(c: char) -> &'static str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    }
}
