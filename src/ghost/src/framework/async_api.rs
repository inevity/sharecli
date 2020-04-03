use crate::framework::{
    auth,
    auth::{AuthClient, Credentials},
    endpoint::Endpoint,
    reqwest_adaptors::match_reqwest_method,
    response::{ApiErrors, ApiFailure, ApiSuccess},
    response::{ApiResponse, ApiResult},
    Environment, HttpApiClientConfig,
};
use reqwest;
use serde::Serialize;

pub struct Client {
    environment: Environment,
    credentials: Credentials,
    http_client: reqwest::Client,
}

impl AuthClient for reqwest::RequestBuilder {
    fn auth(mut self, credentials: &Credentials) -> Self {
        for (k, v) in credentials.headers() {
            self = self.header(k, v);
        }
        self
    }
}

impl Client {
    pub fn new(
        credentials: auth::Credentials,
        config: HttpApiClientConfig,
        environment: Environment,
        ) -> Result<Client, failure::Error> {
        let http_client = reqwest::Client::builder()
            .timeout(config.http_timeout)
            .build()?;
        Ok(Client {
            environment,
            credentials,
            http_client,
        })
    }
    pub async fn request<ResultType, QueryType, BodyType>(
        &self,
        endpoint: &dyn Endpoint<ResultType, QueryType, BodyType>,
        ) -> ApiResponse<ResultType>
        where
            ResultType: ApiResult,
            QueryType: Serialize,
            BodyType: Serialize,
        {
            let mut request = self
                .http_client
                .request(
                    match_reqwest_method(endpoint.method()),
                    endpoint.url(&self.environment),
                    )
                .query(&endpoint.query());
            if let Some(body) = endpoint.body() {
                request = request.body(serde_json::to_string(&body).unwrap());
                request = request.header(reqwest::header::CONTENT_TYPE, endpoint.content_type());
            }
            request = request.auth(&self.credentials);
            let response = request.send().await?;
            map_api_response(response).await
    }
}

async fn map_api_response<ResultType: ApiResult>(
    resp: reqwest::Response,
    ) -> ApiResponse<ResultType> {
    let status = resp.status();
    if status == reqwest:: StatusCode::OK {
        let parsed: Result<ApiSuccess<ResultType>, reqwest::Error> = resp.json().await;
        match parsed {
            Ok(api_resp) => Ok(api_resp),
            Err(e) => Err(ApiFailure::Invalid(e)),
        } 
    } else {
            let parsed: Result<ApiErrors, reqwest::Error> = resp.json().await;
            let errors = parsed.unwrap_or_default();
            Err(ApiFailure::Error(status, errors))
        }
} 

