#![doc = include_str!("../../README.md")]
#![doc = include_str!("../README.md")]

use clap::{arg, command, value_parser};
use util::logging::LogLevel;

mod compare_raw_data;
mod count_location_trips;
mod create_heatmap_diagram;
mod parse_raw_jids;
pub(crate) mod request_raw_jids;
pub mod util;

/// The main method of the cli program.
///
/// This method parses the system-given command arguments using the `clap`
/// crate and calls the corresponding method.
/// 
/// # List of sub-commands:
/// All commands can be passed -v or -vv to make them more verbose.
/// You can add the `--help` argument to any subcommand to see details about the arguments.
/// If you do not want to compile the cli, refer to the source code for arguments.
/// 
/// - `data`
///   - `request_raw`: This command requests raw schedule data over all jids in the HAFAS endpoint. 
///     Independenty of the `TO` argument, the command will panic if it reaches the last jid.
///     The ususal last jid is around 1.5 Million.
///     
///     It is recommended to leave all optional arguments to a default value to prevent a timeout.
///   - `parse`: This command parses the data from the `raw_data` table to a usable format and insertis it into the other tables. 
///     It can also be called automatically by adding `--parse` to the arguments of `request_raw`.
///   - `parse_heatmap`: This command is a command that should be called before using the `create_heatmap` feature. 
///     It counts together all of the recorded train trips into their own table for faster access.
///     As near and local trafic can obfuscate the ending image, I recommend setting `-o 'ICE'` as filter.
///   - `check`: This command checks if data from the HAFAS endpoint differs from the current data. 
///     This check is only done for a single jid. A difference might hint at a schedule change.
/// - `create_heatmap` Creates a horizontal bar diagram of the most used stations in the lookup table. 
///   It can be filtered by cat_code, cat_out and search limit. For currently unkown reasons, anything over 11 Bars will mess up the station names.
/// - `migrate`: Creates all neccessary infrastructure on the remote database.
#[tokio::main]
async fn main() {
    // Setup clap with subcommands
    let mut cmd = command!("Hafas Scraping Database Tools")
        .arg(arg!(-v --verbose... "Verbose output. Specify twice for trace mode"))
        .arg(arg!(-q --quiet "Only print on error and warnings"))
        .subcommands(&[
            command!("data")
                .about("Lookup and schedule data management")
                .subcommands(&[
                    command!("request_raw")
                        .about("Request raw schedule data")
                        .args(&[
                            arg!(<TO> "Stop at jid").value_parser(value_parser!(usize)),
                            arg!([FROM] "Begin with jid, defaults to 0")
                                .value_parser(value_parser!(usize)),
                            arg!(-u --update "Also request existing entries"),
                            arg!(-c --continue "Continue at current highes jid"),
                            arg!(-p --parse "Synchronously parse using `data parse`"),
                            arg!(-s --"chunk-size" <SIZE> "Chunk size of requests. Defaults to 100").value_parser(value_parser!(usize)),
                        ]),
                        command!("parse")
                            .about("Parse raw data to schedule and stop data.")
                            .args(&[
                                arg!([FROM] "Begin with jid, defaults to 0").value_parser(value_parser!(i32)),
                                arg!([TO] "Stop at jid").value_parser(value_parser!(i32)),
                                arg!(-s --"chunk-size" <SIZE> "Chunk size of requests. Defaults to 100").value_parser(value_parser!(i32)),
                        ]),
                        command!("parse_heatmap")
                            .about("Parse trip data to heatmap counts."),
                        command!("check")
                        .about("Check if a single jid is different to the remote data.").arg(arg!(<jid> "jid to check on").value_parser(value_parser!(usize))),
                ]),
                command!("create_heatmap")
                .about("Create heatmap diagram from parsed data.")
                .args(&[
                    arg!([FILE] "Output file to use. Defaults to ./heatmap.svg"),
                    arg!(-m --max <SIZE> "Maximum entries to display").value_parser(value_parser!(usize)),
                    arg!(-c --"filter-cat-code" <CAT_CODE> "Filter results by cat code").value_parser(value_parser!(u8)),
                    arg!(-o --"filter-cat-out" <CAT_OUT> "Filter results by cat out (f.e. RE, ICE)"),
                    arg!(-s --"limit-search" <SIZE> "Maximum search results to query").value_parser(value_parser!(i64)),
                ]),
            //command!("status").about("Query database status"),
            command!("migrate").about("Migrate/create database"),
        ]);

    let matches = cmd.clone().get_matches();

    let log_level = if matches.get_flag("quiet") {
        LogLevel::Warn
    } else {
        LogLevel::from(2 - matches.get_count("verbose"))
    };

    match matches.subcommand() {
        Some(("data", sub_match)) => match sub_match.subcommand() {
            Some(("request_raw", sub_match)) => {
                request_raw_jids::request(
                    sub_match.get_one("FROM").unwrap_or(&0),
                    sub_match.get_one("TO").unwrap(),
                    sub_match.get_one("chunk-size").unwrap_or(&100),
                    sub_match.get_flag("continue"),
                    sub_match.get_flag("update"),
                    sub_match.get_flag("parse"),
                    &log_level,
                )
                .await;
            }
            Some(("parse", sub_match)) => {
                parse_raw_jids::parse(
                    sub_match.get_one("FROM"),
                    sub_match.get_one("TO"),
                    sub_match.get_one("chunk-size").unwrap_or(&100),
                    &log_level,
                )
                .await;
            }
            Some(("parse_heatmap", _)) => {
                count_location_trips::parse_count().await;
            }
            Some(("check", sub_match)) => {
                compare_raw_data::compare_print(*sub_match.get_one("jid").unwrap(), &log_level)
                    .await;
            }
            _ => cmd.print_help().unwrap(),
        },
        Some(("create_heatmap", sub_match)) => {
            create_heatmap_diagram::create_heatmap(
                sub_match.get_one("FILE"),
                sub_match
                    .get_one("filter-cat-out")
                    .unwrap_or(&String::from(".*")),
                sub_match.get_one("filter-cat-code").unwrap_or(&8),
                sub_match.get_one("limit-search").unwrap_or(&i64::MAX),
                sub_match.get_one("max").unwrap_or(&usize::MAX),
            )
            .await;
        }
        Some(("status", _)) => {}
        Some(("migrate", _)) => sqlx::migrate!()
            .run(&mut crate::util::database::connect().await)
            .await
            .unwrap(),
        _ => cmd.print_help().unwrap(),
    };
}
