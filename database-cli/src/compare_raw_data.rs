//! Functions for the cli sub-command `data check`

use crate::{request_raw_jids, util::logging::LogLevel};

/// This function is called by the cli sub-command `data check`.
///
/// It checks a specific `jid` against the database and prints out if the results differ.
/// On verbose mode, it also prints the raw data String it compared. 
/// If the results differ, it prints both.
pub async fn compare_print(jid: usize, log_level: &LogLevel) {
    let remote = request_raw_jids::get_request_bodys(
        &request_raw_jids::make_raw_request(&hafas_wrap::Client::new(), &[jid]).await,
    )
    .get(0)
    .unwrap()
    .clone();
    let database = sqlx::query!(
        "SELECT raw FROM lookup_data.raw_data WHERE jid = $1",
        jid as i32
    )
    .fetch_one(&mut crate::util::database::connect().await)
    .await
    .unwrap()
    .raw;

    if remote.get("common") == database.get("common")
        && remote.get("journey") == database.get("journey")
    {
        println!("{jid} is the same.");
        if log_level <= &LogLevel::Debug {
            println!("Data: {remote}");
        }
    } else {
        println!("{jid} differs!");
        if log_level <= &LogLevel::Debug {
            println!("Remote: {remote}\nDatabase: {database}");
        }
    }
}
