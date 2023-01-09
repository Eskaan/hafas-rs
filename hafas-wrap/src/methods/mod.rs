//! Predefined request and response structs for HAFAS requests.
//! 
//! These can be used in the [request](crate::Client::request()) functions.
mod journey_match;
pub mod sub_types;
pub use journey_match::*;
mod him_search;
pub use him_search::*;
mod journey_detail;
pub use journey_detail::*;


/// Creates a request struct with corresponding implementations
/// 
/// This macro creates a struct with a `From` implementation into a [`RawHafasRequest`](crate::RawHafasRequest)
/// and some needed deriviatives like [`Serialize`](serde::Serialize), [`Debug`] and [`Default`].
/// It also allows non_snake_case because most of HAFAS is written in camelCase.
///
/// Altrough this macro derives Debug, that is just to simplify the creation
/// of request objects. It it not safe to assume the default is a working
/// request.
/// 
/// # Example
/// ```rust
/// # use hafas_wrap::define_request;
/// define_request!(
///     "ExampleMethodName",
///     ExampleMethodRequest {
///         some_value: bool,
///         longer_value: String,
///     }
/// );
/// ```
#[macro_export]
macro_rules! define_request {
    // Create Request
    (
        $method_name:expr,
        $struct_name:ident {
            $($field_name:ident: $field_type:ty,)*
        }
    ) => {
        #[allow(non_snake_case)]
        #[serde_with::skip_serializing_none]
        #[derive(serde::Serialize, Debug, Default)]
        pub struct $struct_name {
            $(pub $field_name: $field_type,)*
        }

        impl From<$struct_name> for $crate::RawHafasRequest<$struct_name> {
            fn from(value: $struct_name) -> $crate::RawHafasRequest<$struct_name> {
                $crate::RawHafasRequest {
                    meth: $method_name,
                    req: value,
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{hafas_profiles::DB, RawHafasRequest};
    use serde::{de::DeserializeOwned, Serialize};
    use std::fmt::Debug;

    fn request<I: Serialize + Sized + Debug, O: DeserializeOwned>(r: RawHafasRequest<I>) {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            let _: Vec<O> = crate::Client::new().request(&DB, vec![r]).await.unwrap();
        });
    }

    #[test]
    fn journey_match_into() {
        request::<JourneyMatchRequest, JourneyMatchResponse>(
            JourneyMatchRequest {
                date: String::from("20221204"),
                input: String::from("RE 4747"),
                ..Default::default()
            }
            .into(),
        );
    }

    #[test]
    fn journey_details_into() {
        request::<JourneyDetailsRequest, JourneyDetailsResponse>(
            JourneyDetailsRequest {
                jid: String::from("1|221760|0|80|-1"),
                getPolyline: Some(false),
                ..Default::default()
            }
            .into(),
        );
    }
}
