//! A collection of different HAFAS profiles for different endpoints.
//! 
//! Althrough you can define your own, you can get some common HAFAS endpoint profiles here.
//! These can be used in the [request](crate::Client::request()) functions.


#[derive(Debug, Clone)]
pub struct HafasProfile {
    pub url: &'static str,
    pub secret: Option<String>,
    pub config: serde_json::Value,
}

use crate::util::decode;
use serde_json::json;

lazy_static::lazy_static! {
    /// Germany - Deutsche Bahn
    pub static ref DB: HafasProfile = HafasProfile {
        url: "https://reiseauskunft.bahn.de/bin/mgate.exe",
        secret: Some(decode("IeesB8a/ctdu457TpvVasltMN3tgSw3qRUBj6nJiY+4=").unwrap()),
        config: json!({"client":{"id":"DB","v":"20100000","type":"IPH","name":"DB Navigator"},"ext":"DB.R22.04.a","lang":"de","ver":"1.18","auth":{"aid":"n91dB8Z77MLdoR0K","type":"AID"}}),
    };
    /// Swizerland - Schweizerische Bundesbahn
    pub static ref SBB: HafasProfile = HafasProfile {
        url: "http://fahrplan.sbb.ch/bin/mgate.exe",
        secret: None,
        config: json!({"auth":{"aid":"hf7mcf9bv3nv8g5f","type":"AID"},"client":{"id":"DBZUGRADARNETZ","type":"AND","v":""},"ext":"DBNETZZUGRADAR.2","formatted":false,"lang":"deu","ver":"1.18"}),
    };
    /// Austria - Österreichische Bundesbahn
    pub static ref OEBB: HafasProfile = HafasProfile {
        url: "https://fahrplan.oebb.at/bin/mgate.exe",
        secret: None,
        config: json!({"client":{"os":"iOS 12.4","id":"OEBB","v":"6020300","type":"IPH","name":"oebbADHOC"},"lang":"de","ver":"1.18","auth":{"aid":"OWDL4fE4ixNiPBBm","type":"AID"}}),
    };
    /// Netherlands
    pub static ref SNCB: HafasProfile = HafasProfile {
        url: "http://www.belgianrail.be/jp/sncb-nmbs-routeplanner/mgate.exe",
        secret: None,
        config: json!({"client":{"os":"iOS 12.4","id":"SNCB","v":"4030200","type":"IPH","name":"sncb"},"lang":"de","ver":"1.18","auth":{"aid":"sncb-mobi","type":"AID"}}),
    };
    /// Poland
    pub static ref PKP: HafasProfile = HafasProfile {
        url: "https://mobil.rozklad-pkp.pl:8019/bin/mgate.exe",
        secret: None,
        config: json!({"client":{"id":"HAFAS","type":"AND"},"lang":"en","ver":"1.18","auth":{"aid":"DrxJYtYZQpEBCtcb","type":"AID"}}),
    };
    pub static ref BVG: HafasProfile = HafasProfile {
        url: "https://bvg-apps.hafas.de/bin/mgate.exe",
        secret: None,
        config: json!({"client":{"os":"iOS 12.4","id":"BVG","v":"6021600","type":"IPH","name":"Fahrinfo"},"lang":"de","ver":"1.18","auth":{"aid":"Mz0YdF9Fgx0Mb9","type":"AID"}}),
    };
    pub static ref HVV: HafasProfile = HafasProfile {
        url: "https://hvv-app.hafas.de/bin/mgate.exe",
        secret: Some(decode("QSbzFgPlbKddv4YaZlKmMawExDseXAlqVyTLJSlKEjo=").unwrap()),
        config: json!({"client":{"os":"iOS 12.4","id":"HVV","v":"4020100","type":"IPH","name":"HVVPROD_ADHOC"},"lang":"de","ext":"HVV.1","ver":"1.18","auth":{"aid":"andcXUmC9Mq6hjrwDIGd2l3oiaMrTUzyH","type":"aid"}}),
    };
    pub static ref RMV: HafasProfile = HafasProfile {
        url: "https://www.rmv.de/auskunft/bin/jp/mgate.exe",
        secret: None,
        config: json!({"client":{"id":"RMV","l":"vs_webapp","type":"WEB","name":"webapp"},"ext":"RMV.1","lang":"de","ver":"1.18","auth":{"aid":"x0k4ZR33ICN9CWmj","type":"AID"}}),
    };
    pub static ref AVV: HafasProfile = HafasProfile {
        url: "https://auskunft.avv.de/bin/mgate.exe",
        secret: None,
        config: json!({"client":{"id":"HAFAS","type":"WEB","name":"Test-Client","v":"100"},"lang":"deu","ver":"1.18","auth":{"type":"AID","aid":"4vV1AcH3N511icH"}}),
    };
    pub static ref VAO: HafasProfile = HafasProfile {
        url: "http://app.verkehrsauskunft.at/bin/mgate.exe",
        secret: None,
        config: json!({"client":{"id":"VAO","type":"IPH"},"ver":"1.18","lang":"deu","ext":"VAO.10","auth":{"type":"USER","user":"mobile","pw":"87a6f8ZbnBih32","aid":"hf7mcf9bv3nv8g5f"}}),
    };
    pub static ref VBN: HafasProfile = HafasProfile {
        url: "https://fahrplaner.vbn.de/bin/mgate.exe",
        secret: Some(decode("awanINIF6bWi0KUjJTq78Dvvi1x0BkRB/MfBl0fS8Uw=").unwrap()),
        config: json!({"client":{"os":"iOS 13.4.1","id":"VBN","name":"vbn","type":"IPH","v":"6000000"},"ver":"1.18","lang":"de","auth":{"aid":"kaoxIXLn03zCr2KR","type":"AID"}}),
    };
    pub static ref DBNETZ: HafasProfile = HafasProfile {
        url: "https://db-livemaps.hafas.de/bin/mgate.exe",
        secret: None,
        config: json!({"client":{"id":"DBZUGRADARNETZ","v":"0.1.0","type":"WEB","name":"webapp"},"ext":"DBNETZZUGRADAR.2","ver":"1.18","auth":{"type":"AID","aid":"hf7mcf9bv3nv8g5f"},"lang":"deu"}),
    };
    pub static ref DBREGIO: HafasProfile = HafasProfile {
        url: "https://bnav.hafas.de/bin/mgate.exe",
        secret: Some(decode("8h+PHzsE8XcQ4mYhVBf7rj1FZ+bMLGmvVxkDg4biIYY=").unwrap()),
        config: json!({"client":{"id":"DB-REGIO-BNAV","v":"3000500","type":"IPH","name":"StreckenagentPROD-APPSTORE"},"lang":"de","ext":"DB.R19.12.a","ver":"1.18","auth":{"aid":"Xd91BNAVkuI6rr6z","type":"AID"}}),
    };
    pub static ref NAHSH: HafasProfile = HafasProfile {
        url: "https://nah.sh.hafas.de/bin/mgate.exe",
        secret: None,
        config: json!({"client":{"os":"iOS 12.4","id":"NAHSH","v":"5000100","type":"IPH","name":"NAHSHPROD-APPSTORE"},"lang":"de","ver":"1.18","auth":{"aid":"r0Ot9FLFNAFxijLW","type":"AID"}}),
    };
    pub static ref INSA: HafasProfile = HafasProfile {
        url: "https://reiseauskunft.insa.de/bin/mgate.exe",
        secret: None,
        config: json!({"client":{"os":"iOS 12.4.1","id":"NASA","v":"4020300","type":"IPH","name":"nasaPROD-APPSTORE"},"lang":"de","ver":"1.18","auth":{"aid":"nasa-apps","type":"AID"}}),
    };
    pub static ref ANACHB: HafasProfile = HafasProfile {
        url: "https://anachb.vor.at/bin/mgate.exe",
        secret: None,
        config: json!({"client":{"id":"VAO","type":"WEB","name":"webapp"},"ver":"1.18","lang":"deu","ext":"VAO.10","auth":{"type":"AID","aid":"wf7mcf9bv3nv8g5f"}}),
    };
    pub static ref SMARTRBL: HafasProfile = HafasProfile {
        url: "https://db-smartrbl.hafas.de/mct/mgate",
        secret: None,
        config: json!({"client":{"id":"HAFAS","name":"Test-Client","type":"WEB","v":"1.0.0"},"lang":"deu","ver":"1.18","auth":{"aid":"izfpmpj8tnh6acye","type":"AID"}}),
    };
}
