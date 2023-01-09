use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::sub_types::{journey, Common, Journey};
use crate::define_request;

define_request!(
    "JourneyMatch",
JourneyMatchRequest {
    jnyFltrL: Option<Vec<journey::JourneyFilter>>,
    date: String,
    dateB: Option<String>,
    dateE: Option<String>,
    extId: Option<String>,
    input: String,
    onlyCR: Option<bool>,
    onlyRT: Option<bool>,
    onlyTN: Option<bool>,
    time: Option<String>,
    timeE: Option<String>,
    timeB: Option<String>,
    tripId: Option<String>,
    useAeqi: Option<bool>,
});

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct JourneyMatchResponse {
    pub common: Common,
    pub jnyL: Vec<Journey>,
    pub fpB: String,
    pub fpE: String,
    pub planrtTS: String,
}
