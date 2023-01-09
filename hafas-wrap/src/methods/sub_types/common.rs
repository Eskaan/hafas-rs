use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct Common {
    pub locL: Vec<LocL>,
    pub prodL: Vec<ProdL>,
    pub polyL: Vec<PolyL>,
    pub layerL: Vec<LayerL>,
    pub crdSysL: Vec<CrdSysL>,
    pub opL: Vec<OpL>,
    pub remL: Vec<RemL>,
    pub icoL: Vec<IcoL>,
    pub tcocL: Option<Vec<TcocL>>,
    pub himMsgEdgeL: Option<Vec<HimMsgEdgeL>>,
}

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct LocL {
    pub lid: Option<String>,
    pub r#type: Option<String>,
    pub name: String,
    pub icoX: Option<isize>,
    pub extId: String,
    pub state: Option<String>,
    pub crd: Crd,
    pub pCls: Option<isize>,
    /// Reference to prodL
    pub pRefL: Option<Vec<isize>>,
}

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Serialize, Debug)]
pub struct OptionalLocL {
    pub lid: Option<String>,
    pub r#type: Option<String>,
    pub name: Option<String>,
    pub icoX: Option<isize>,
    pub extId: Option<String>,
    pub state: Option<String>,
    pub crd: Option<Crd>,
    pub pCls: Option<isize>,
    /// Reference to prodL
    pub pRefL: Option<Vec<isize>>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug)]
pub struct Crd {
    pub x: isize,
    pub y: isize,
    pub z: Option<isize>,
    pub layerX: Option<isize>,
    pub crdSysX: Option<isize>,
}

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct ProdL {
    pub name: String,
    pub isize: Option<String>,
    pub icoX: isize,
    pub cls: isize,
    pub oprX: Option<isize>,
    pub prodCtx: Option<ProdCtx>,
    pub addName: Option<String>,
    pub nameS: Option<String>,
    pub matchId: Option<String>,
}

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProdCtx {
    pub name: String,
    pub num: Option<String>,
    pub matchId: Option<String>,
    pub catOut: Option<String>,
    pub catOutS: Option<String>,
    pub catOutL: String, // was opt
    pub catIn: Option<String>,
    pub catCode: String, //was opt
    pub admin: Option<String>,
    pub lineId: Option<String>,
    pub line: Option<String>,
    //pub cls: isize,
    //pub icoX: isize,
}

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct PolyL {
    pub delta: bool,
    pub dim: isize,
    pub crdEncYX: String,
    pub crdEncS: String,
    pub crdEncF: String,
    pub ppLocRefL: Vec<PpLocRefL>,
}

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct PpLocRefL {
    pub ppIdx: isize,
    pub locX: isize,
}

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct LayerL {
    pub id: String,
    pub name: String,
    pub index: isize,
    pub annoCnt: isize,
}

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct CrdSysL {
    pub id: String,
    pub index: isize,
    pub r#type: String,
}

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct OpL {
    pub name: String,
    pub icoX: isize,
}

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct RemL {
    pub r#type: String,
    pub code: String,
    pub icoX: isize,
    pub txtN: String,
    pub txtS: Option<String>,
    pub prio: Option<isize>,
    pub sIdx: Option<isize>,
}

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct IcoL {
    pub res: String,
    pub txt: Option<String>,
}

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct TcocL {
    pub c: String,
    pub r: Option<isize>,
}

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct HimMsgEdgeL {
    pub icoCrd: IcoCrd,
}

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct IcoCrd {
    pub x: String,
    pub y: String,
}
