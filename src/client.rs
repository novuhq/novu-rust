use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::error::NovuError;

#[derive(Deserialize, Debug)]
pub struct ApiError {
    #[serde(rename = "statusCode")]
    pub status_code: i32,
    pub message: String,
    pub error: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ApiErrorWithMessages {
    #[serde(rename = "statusCode")]
    pub status_code: i32,
    pub message: Vec<String>,
    pub error: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Response<T> {
    Success(T),
    Error(ApiError),
    Messages(ApiErrorWithMessages),
}
pub struct Client {
    api_url: String,
    client: reqwest::Client,
}

impl Client {
    pub fn new(
        api_key: impl ToString,
        backend_url: Option<impl ToString>,
    ) -> Result<Self, NovuError> {
        Ok(Self {
            api_url: Self::build_backend_url(&backend_url),
            client: Self::build_client(&api_key)?,
        })
    }

    pub async fn post<T: DeserializeOwned>(
        &self,
        endpoint: &str,
        data: &impl Serialize,
    ) -> Result<Response<T>, NovuError> {
        let res = self
            .client
            .post(self.get_url(endpoint))
            .json(&data)
            .send()
            .await;

        match res {
            Ok(response) => Ok(response.json::<Response<T>>().await?),
            Err(err) => Err(NovuError::HttpError(err)),
        }
    }

    fn get_url(&self, endpoint: impl ToString) -> String {
        format!("{}{}", self.api_url, endpoint.to_string())
    }

    fn build_backend_url(backend_url: &Option<impl ToString>) -> String {
        const NOVU_VERSION: &str = "v1";

        if let Some(backend_url) = backend_url {
            let backend_url = &backend_url.to_string();

            if backend_url.contains("novu.co/v") {
                return backend_url.to_string();
            }

            return format!("{}/{}", backend_url, NOVU_VERSION);
        }

        format!(
            "https://api.novu.co/{NOVU_VERSION}",
            NOVU_VERSION = NOVU_VERSION
        )
    }

    fn build_client(api_key: &impl ToString) -> Result<reqwest::Client, NovuError> {
        let mut default_headers = reqwest::header::HeaderMap::new();

        default_headers.insert(
            "Authorization",
            reqwest::header::HeaderValue::from_str(&format!("ApiKey {}", &api_key.to_string()))?,
        );

        let client = reqwest::Client::builder()
            .default_headers(default_headers)
            .build();

        match client {
            Ok(data) => Ok(data),
            Err(_) => Err(NovuError::BuildError("client".to_string())),
        }
    }
}
