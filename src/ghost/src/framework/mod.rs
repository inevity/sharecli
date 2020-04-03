pub mod apiclient; //import ,and can use exteranl
pub mod async_api;
pub mod auth;
pub mod endpoint;
pub mod mock;
mod reqwest_adaptors; // import and only use here
pub mod response;

//use crate::framework::{apiclient::ApiClient, auth::AuthClient, response::map_api_response};
use crate::framework::{apiclient::ApiClient, auth::AuthClient, response::map_api_response};
//when import we will can use the auth tarit

use reqwest_adaptors::match_reqwest_method;
// default imported??
use serde::Serialize;
use std::time::Duration;

#[derive(Serialize, Clone, Debug)]
pub enum OrderDirection {
    #[serde(rename = "asc")]
    Ascending,
    #[serde(rename = "desc")]
    Descending,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SearchMatch {
    ALl,
    Any,
}

#[derive(Debug)]
pub enum Environment {
    Production,
    Custom(url::Url),
}

impl<'a> From<&'a Environment> for url::Url {
    fn from(environment: &Environment) -> Self {
        match environment {
            Environment::Production => {
               // url::Url::parse("https://api.cloudflare.com/client/v4").unwrap()
                url::Url::parse("https://blog.approachai.com/ghost/api/v3/admin/").unwrap()
            }
            Environment::Custom(url) => url.clone(),
        }
    }
}

pub struct HttpApiClient {
    environment: Environment,
    credentials: auth::Credentials,
    http_client: reqwest::blocking::Client,
}
pub struct HttpApiClientConfig {
    pub http_timeout: Duration,
}
impl Default for HttpApiClientConfig {
    fn default() -> Self {
        HttpApiClientConfig {
            http_timeout: Duration::from_secs(30),
        }
    }
}

impl HttpApiClient {
    pub fn new(
        credentials: auth::Credentials,
        config: HttpApiClientConfig,
        environment: Environment,
    ) -> Result<HttpApiClient, failure::Error> {
        let http_client = reqwest::blocking::Client::builder()
            .timeout(config.http_timeout)
            .build()?;

        Ok(HttpApiClient {
            environment,
            credentials,
            http_client,
        })
    }
        
}

impl<'a> ApiClient for HttpApiClient {
    fn request<ResultType, QueryType, BodyType>(
        &self,
        endpoint: &dyn endpoint::Endpoint<ResultType, QueryType, BodyType>,
        ) -> response::ApiResponse<ResultType>
        where
            ResultType: response::ApiResult,
            QueryType: Serialize,
            BodyType: Serialize,
       {
           let mut request = self
               .http_client //reqwest blocking client
               .request( //setting req from endpoint set
                   match_reqwest_method(endpoint.method()),
                       endpoint.url(&self.environment),
               )
               .query(&endpoint.query()); //set query 
           if let Some(body) = endpoint.body() {
               request = request.body(serde_json::to_string(&body).unwrap());
               request = request.header(reqwest::header::CONTENT_TYPE, endpoint.content_type());
           }        

           request = request.auth(&self.credentials); //auth header setting who impl this auth
           // impl auth trait when run

           //send req 
           let response = request.send()?;
           map_api_response(response)

       }
}
