use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::sub_types::{common, Common, Journey};
use crate::define_request;

define_request!(
    "JourneyDetails",
JourneyDetailsRequest {
      jid: String,
      getAltCoordinates: Option<bool>,
      getAnnotations: Option<bool>,
      getPasslist: Option<bool>,
      getPolyline: Option<bool>,
      getSimpleTrainComposition: Option<bool>,
      getTrainComposition: Option<bool>,
      aDate: Option<String>,
      aIdx: Option<isize>,
      aLoc: Option<common::OptionalLocL>,
      aTime: Option<String>,
      dDate: Option<String>,
      dIdx: Option<isize>,
      dLoc: Option<common::OptionalLocL>,
      dTime: Option<String>,
      date: Option<String>,
      name: Option<String>,
      polySplitting: Option<bool>,
    }
);

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct JourneyDetailsResponse {
    pub common: Common,
    pub journey: Journey,
    pub fpB: String,
    pub fpE: String,
    pub planrtTS: String,
}
