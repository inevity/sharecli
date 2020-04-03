extern crate reqwest;
extern crate serde_json;
mod apifail;

pub use apifail::*;
use serde_json::value::Value as JsonValue;

#[derive(Deserialize, Debug, PartialEq)]
pub struct ApiSuccess<ResultType> {
    pub result: ResultType,
    pub result_info: Option<JsonValue>,
    pub messages: JsonValue,
    pub errors: Vec<ApiError>,
}
pub type ApiResponse<ResultType> = Result<ApiSuccess<ResultType>, ApiFailure>;

pub fn map_api_response<ResultType: ApiResult> (
    resp: reqwest::blocking::Response,
    ) -> ApiResponse<ResultType> {
    let status = resp.status();
    if status == reqwest::StatusCode::OK {
        let parsed: Result<ApiSuccess<ResultType>, reqwest::Error> = resp.json();
        match parsed {
            Ok(api_resp) => Ok(api_resp),
            Err(e) => Err(ApiFailure::Invalid(e)),
        }
    } else {
        let parsed: Result<ApiErrors, reqwest::Error> = resp.json();
        let errors = parsed.unwrap_or_default();
        Err(ApiFailure::Error(status, errors))
    }
}
/// Some endpoints return nothing. That's OK.

impl ApiResult for () {}

