#![doc = include_str!("../README.md")]

pub mod hafas_profiles;
pub mod methods;
pub mod util;

use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;
use thiserror::Error;

use crate::hafas_profiles::HafasProfile;

/// The request error is the error returned from the two [request](Client::request()) functions.
#[derive(Error, Debug)]
pub enum RequestError {
    #[error("Could not serialize/deserialize request")]
    JsonError(#[from] serde_json::Error),
    #[error("Could not deserialize request")]
    DeserializeError(&'static str),
    #[error("Request failed")]
    SendError(#[from] reqwest::Error),
    #[error("Request failed with internal error: $0, $1")]
    InternalError(String, String),
}

/// The client is a simple struct that manages requests to HAFAS. I only contains a [reqwest] web 
/// client to make requests.
///
/// # Usage
/// To make a request, you will need three things: A [`Client`], a [`HafasProfile`] 
/// and a request object of the [`RawHafasRequest`] type. You then call 
/// `client.request::<_, ResponseType>(profile, request).await`, 
/// making a call to HAFAS and parsing the result.
///
/// There are also more specialized methods like request_raw, if you don't want to specify the 
/// response type and just want to get the raw response String, altrough this method has the 
/// advantage of fewer memory allocations and thus higher execution speed, it is rarely used and 
/// not really recommended practice.
#[derive(Debug)]
pub struct Client {
    /// A reqwest client used to make web requests.
    reqwest_client: reqwest::Client,
}
impl Client {
    #[must_use]
    pub fn new() -> Self {
        Self {
            reqwest_client: reqwest::Client::default(),
        }
    }

    /// Serializes, requests and deserializes a request with the given profile and client. See [`Client`] for more information.
    ///
    /// # Errors
    /// - [`SendError`](RequestError::SendError) from [`reqwest::Error`] when the web request fails.
    /// - [`JsonError`](RequestError::JsonError) from [`serde_json::Error`] when serializing/deserializing fails.
    /// - [`InternalError`](RequestError::InternalError) when the response contains an error message. Usually means the request was wrongly formulated.
    /// - [`DeserializeError`](RequestError::DeserializeError) when the request contains an otherwise invalid body.
    /// 
    /// # Example
    /// This example queries the JourneyDetails for the train with jid `1|221760|0|80|-1`.
    /// ```rust
    /// # use hafas_wrap::Client;
    /// # use hafas_wrap::methods;
    /// # use hafas_wrap::hafas_profiles;
    /// # async {
    /// let client = Client::new();
    /// let profile = &hafas_profiles::DB;
    /// let request = methods::JourneyDetailsRequest {
    ///     jid: String::from("1|221760|0|80|-1"),
    ///     ..Default::default()
    /// };
    /// let response: &methods::JourneyDetailsResponse = &client.request(profile, vec![request.into()]).await.unwrap()[0];
    /// # };
    /// ```
    pub async fn request<I: Serialize + Sized, O: DeserializeOwned>(
        &self,
        profile: &HafasProfile,
        requests: Vec<RawHafasRequest<I>>,
    ) -> Result<Vec<O>, RequestError> {
        let res_text = self.request_raw(profile, requests).await?;
        let res = serde_json::from_str::<Value>(&res_text)?;

        // Catch error because HAFAS does not give http error codes.
        if res.get("err") != Some(&Value::String(String::from("OK"))) {
            return Err(RequestError::InternalError(
                res.get("err")
                    .unwrap()
                    .as_str()
                    .unwrap_or_default()
                    .to_string(),
                res.get("errTxt")
                    .unwrap_or(&Value::Null)
                    .as_str()
                    .unwrap_or_default()
                    .to_string(),
            ));
        }

        // Get request body's from response, discarding
        res.get("svcResL")
            .ok_or(RequestError::DeserializeError(
                "Response not an Object or svcResL does not exist in it.",
            ))?
            .as_array()
            .ok_or(RequestError::DeserializeError("svcResL is not an array."))?
            .iter()
            .map(|r| {
                Ok(serde_json::from_value(
                    r.get("res")
                        .ok_or(RequestError::DeserializeError(
                            "field res does not exist. This is most likely a request error.",
                        ))?
                        .clone(),
                )?)
            })
            .collect::<Result<Vec<O>, RequestError>>()
    }

    /// Serializes and requests a request, returning the raw request body.
    /// # Errors
    /// - [`SendError`](RequestError::SendError) from [`reqwest::Error`] when the web request fails.
    /// - [`JsonError`](RequestError::JsonError) from [`serde_json::Error`] when serializing fails.
    /// 
    /// # Example
    /// This example queries the JourneyDetails for the train with jid `1|221760|0|80|-1`.
    /// ```rust
    /// # use hafas_wrap::Client;
    /// # use hafas_wrap::methods;
    /// # use hafas_wrap::hafas_profiles;
    /// # async {
    /// let client = Client::new();
    /// let profile = &hafas_profiles::DB;
    /// let request = methods::JourneyDetailsRequest {
    ///     jid: String::from("1|221760|0|80|-1"),
    ///     ..Default::default()
    /// };
    /// let response: String = client.request_raw(profile, vec![request.into()]).await.unwrap();
    /// # };
    /// ```
    pub async fn request_raw<I: Serialize + Sized>(
        &self,
        profile: &HafasProfile,
        requests: Vec<RawHafasRequest<I>>,
    ) -> Result<String, RequestError> {
        //Add requests to config
        let mut req_values = profile.config.clone();
        req_values["svcReqL"] = serde_json::to_value(&requests).unwrap();
        let req_string = serde_json::to_string(&req_values)?;

        let checksum = match &profile.secret {
            Some(secret) => util::hash_md5(&(req_string.clone() + secret)),
            None => String::new(),
        };

        let response = self
            .reqwest_client
            .post(profile.url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .query(&[("checksum", &checksum)])
            .body(req_string)
            .send()
            .await?;

        Ok(response.text().await?)
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

/// The RawHafasRequest consists of both the request method and a generic request object that has 
/// to implement Serialize. It is used to query HAFAS in [request()](Client::request()).
#[derive(Serialize, Debug)]
pub struct RawHafasRequest<I: Serialize + Sized> {
    pub meth: &'static str,
    pub req: I,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        hafas_profiles::DB,
        methods::{JourneyMatchRequest, JourneyMatchResponse},
    };

    #[test]
    fn journey_match_bulk() {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            let client = Client::new();
            let req: &Vec<JourneyMatchResponse> = &client
                .request(
                    &DB,
                    vec![
                        RawHafasRequest {
                            meth: "JourneyMatch",
                            req: JourneyMatchRequest {
                                date: String::from("20221122"),
                                input: String::from("RE 4734"),
                                ..JourneyMatchRequest::default()
                            },
                        },
                        JourneyMatchRequest {
                            date: String::from("20221122"),
                            input: String::from("RE 4735"),
                            ..JourneyMatchRequest::default()
                        }
                        .into(),
                    ],
                )
                .await
                .unwrap();
            dbg!(&req);
        });
    }
}
