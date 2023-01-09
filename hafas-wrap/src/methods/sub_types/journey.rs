use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct Journey {
    pub jid: String,
    pub date: Option<String>,
    pub prodX: isize,
    pub status: Option<String>,
    pub isRchbl: Option<bool>,
    pub stopL: Option<Vec<CommonStop>>,
    pub sDaysL: Vec<SDaysL>,
    pub polyG: Option<PolyG>,
    pub msgL: Option<Vec<MsgL>>,
    pub subscr: Option<String>,
    pub prodL: Option<Vec<ProdL>>,
    pub dTrnCmpSX: Option<TrnCmpSX>,
}

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct CommonStop {
    pub locX: isize,
    pub idx: Option<isize>,
    pub dCncl: Option<bool>,
    pub dProdX: Option<isize>,
    pub dInS: Option<bool>,
    pub dInR: Option<bool>,
    pub dTimeS: Option<String>,
    pub dTimeR: Option<String>,
    pub dPlatfS: Option<String>,
    pub dPlatfR: Option<String>,
    pub dProgType: Option<String>,
    pub r#type: Option<String>,
    pub dTZOffset: Option<isize>,
    pub dTrnCmpSX: Option<TrnCmpSX>,
    pub msgL: Option<Vec<MsgL>>,
    pub aCncl: Option<bool>,
    pub aProdX: Option<isize>,
    pub aOutR: Option<bool>,
    pub aTimeS: Option<String>,
    pub aTimeR: Option<String>,
    pub aPlatfS: Option<String>,
    pub aPlatfR: Option<String>,
    pub aProgType: Option<String>,
    pub aTZOffset: Option<isize>,
    pub aTrnCmpSX: Option<TrnCmpSX>,
    pub isAdd: Option<bool>,
}

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct SDaysL {
    pub sDaysR: Option<String>,
    pub sDaysI: Option<String>,
    pub sDaysB: String,
    pub fLocX: isize, //was opt
    pub tLocX: isize, //was opt
}

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct PolyG {
    pub polyXL: Vec<isize>,
    pub layerX: isize,
    pub crdSysX: isize,
}

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct MsgL {
    pub r#type: String,
    pub remX: isize,
    pub txtC: Option<TxtC>,
    pub prio: Option<isize>,
    pub fIdx: Option<isize>,
    pub tIdx: Option<isize>,
    pub tagL: Vec<String>,
}

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct ProdL {
    pub prodX: isize,
    pub fLocX: isize,
    pub tLocX: isize,
    pub fIdx: isize,
    pub tIdx: isize,
}

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct TxtC {
    pub r: isize,
    pub g: isize,
    pub b: isize,
}

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct TrnCmpSX {
    pub tcocX: Option<isize>,
    pub tcM: Option<isize>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct JourneyFilter {
    mode: JourneyFilterMode,
    r#type: JourneyFilterType,
    value: String,
}

#[allow(dead_code)]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub enum JourneyFilterMode {
    BIT,
    EXC,
    INC,
    UNDEF,
}

#[allow(non_camel_case_types, dead_code)]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub enum JourneyFilterType {
    ADM,
    ATTRF,
    ATTRJ,
    ATTRL,
    BC,
    CAT,
    COUCH,
    CTX_RECON,
    GROUP,
    INFOTEXTS,
    JID,
    LID,
    LINE,
    LINEID,
    META,
    NAME,
    NUM,
    OP,
    PID,
    PROD,
    ROUTE,
    SLEEP,
    STATIONS,
    UIC,
}
