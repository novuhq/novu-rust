pub mod client;
pub mod consts;
pub mod environments;
pub mod error;
pub mod events;
pub mod subscriber;

use client::Client;
use environments::{ApiKey, CreateEnvironmentPayload, Environment, EnvironmentPayload};
use error::NovuError;
use events::{TriggerPayload, TriggerResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ChannelTypeEnum {
    EMAIL,
    SMS,
    DIRECT,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IAttachmentOptions {
    pub mime: String,
    pub file: Vec<u8>,
    pub name: Option<String>,
    pub channels: Option<Vec<ChannelTypeEnum>>,
}

pub struct Novu {
    client: Client,
}

impl Novu {
    pub fn new(api_key: impl ToString, api_url: Option<&str>) -> Result<Self, NovuError> {
        Ok(Self {
            client: Client::new(api_key, api_url)?,
        })
    }

    pub async fn trigger(self, data: TriggerPayload) -> Result<TriggerResponse, NovuError> {
        let result = self.client.post("/events/trigger", Some(&data)).await?;

        match result {
            client::Response::Success(data) => Ok(data.data),
            client::Response::Error(err) => match err.status_code {
                422 => Err(match err.message.as_str() {
                    "TEMPLATE_NOT_FOUND" => NovuError::TemplateNotFound(data.name.to_string()),
                    _ => NovuError::Unknown("".to_string()),
                }),
                401 => Err(NovuError::UnauthorizedError("/events/trigger".to_string())),
                400 => {
                    println!("{:?}", err);
                    todo!()
                }
                code => todo!("{}", code),
            },
            client::Response::Messages(err) => Err(NovuError::InvalidValues(
                "triggering".to_string(),
                format!("{:?}", err.message),
            )),
        }
    }

    pub async fn current_environment(&self) -> Result<Environment, NovuError> {
        let result = self.client.get("/environments/me").await?;
        match result {
            client::Response::Success(data) => Ok(data.data),
            client::Response::Error(err) => match err.status_code {
                401 => Err(NovuError::UnauthorizedError("/environments/me".to_string())),
                code => todo!("{}", code),
            },
            client::Response::Messages(err) => Err(NovuError::InvalidValues(
                "current_environment".to_string(),
                format!("{:?}", err.message),
            )),
        }
    }

    pub async fn get_environments(&self) -> Result<Vec<Environment>, NovuError> {
        let result = self.client.get("/environments").await?;
        match result {
            client::Response::Success(data) => Ok(data.data),
            client::Response::Error(err) => match err.status_code {
                401 => Err(NovuError::UnauthorizedError("/environments".to_string())),
                code => todo!("{}", code),
            },
            client::Response::Messages(err) => Err(NovuError::InvalidValues(
                "current_environment".to_string(),
                format!("{:?}", err.message),
            )),
        }
    }

    pub async fn create_environment(
        &self,
        data: EnvironmentPayload,
    ) -> Result<Environment, NovuError> {
        let result = self.client.post("/environments", Some(&data)).await?;
        match result {
            client::Response::Success(data) => Ok(data.data),
            client::Response::Error(err) => match err.status_code {
                401 => Err(NovuError::UnauthorizedError("/environments".to_string())),
                code => todo!("{}", code),
            },

            client::Response::Messages(err) => Err(NovuError::InvalidValues(
                "current_environment".to_string(),
                format!("{:?}", err.message),
            )),
        }
    }

    pub async fn update_environment(
        &self,
        environment_id: String,
        data: CreateEnvironmentPayload,
    ) -> Result<Environment, NovuError> {
        let result = self
            .client
            .put(&format!("/environments/{}", environment_id), &data)
            .await?;
        match result {
            client::Response::Success(data) => Ok(data.data),
            client::Response::Error(err) => match err.status_code {
                401 => Err(NovuError::UnauthorizedError(format!(
                    "/environments/{}",
                    environment_id
                ))),
                code => todo!("{}", code),
            },
            client::Response::Messages(err) => Err(NovuError::InvalidValues(
                "current_environment".to_string(),
                format!("{:?}", err.message),
            )),
        }
    }

    pub async fn get_environment_api_keys(&self) -> Result<ApiKey, NovuError> {
        let result = self.client.get("/environments/api-keys").await?;
        match result {
            client::Response::Success(data) => Ok(data.data),
            client::Response::Error(err) => match err.status_code {
                401 => Err(NovuError::UnauthorizedError(
                    "/environments/api-keys".to_string(),
                )),
                code => todo!("{}", code),
            },
            client::Response::Messages(err) => Err(NovuError::InvalidValues(
                "current_environment".to_string(),
                format!("{:?}", err.message),
            )),
        }
    }

    pub async fn regenerate_environment_api_keys(&self) -> Result<ApiKey, NovuError> {
        let result = self
            .client
            .post("/environments/api-keys/regenerate", None::<&()>)
            .await?;
        match result {
            client::Response::Success(data) => Ok(data.data),
            client::Response::Error(err) => match err.status_code {
                401 => Err(NovuError::UnauthorizedError(
                    "/environments/api-keys".to_string(),
                )),
                400 => {
                    println!("{:?}", err);
                    todo!()
                }
                code => todo!("{}", code),
            },
            client::Response::Messages(err) => Err(NovuError::InvalidValues(
                "current_environment".to_string(),
                format!("{:?}", err.message),
            )),
        }
    }
}

#[cfg(test)]
#[tokio::test]
async fn test_trigger() {
    let novu = Novu::new("", None).unwrap();
    let result = novu
        .trigger(TriggerPayload {
            name: "testing".to_string(),
            payload: std::collections::HashMap::new(),
            to: events::TriggerRecipientsType::Single(
                events::TriggerRecipientBuilder::new("test_subscriber_id")
                    .first_name("Test")
                    .last_name("testing")
                    .build(),
            ),
        })
        .await;

    assert!(result.is_err());
}

#[cfg(test)]
#[tokio::test]
async fn test_current_environment() {
    let novu = Novu::new("", None).unwrap();
    let curr_result = novu.current_environment().await;
    assert!(curr_result.is_err());
}

#[cfg(test)]
#[tokio::test]
async fn test_get_environments() {
    let novu = Novu::new("", None).unwrap();
    let result = novu.get_environments().await;
    assert!(result.is_err());
}

#[cfg(test)]
#[tokio::test]
async fn test_create_environment() {
    let novu = Novu::new("", None).unwrap();
    let create_result = novu
        .create_environment(environments::EnvironmentPayloadBuilder::new("test").build())
        .await;
    assert!(create_result.is_err());
}

#[cfg(test)]
#[tokio::test]
async fn test_update_environment() {
    let novu = Novu::new("", None).unwrap();
    let update_result = novu
        .update_environment(
            "test".to_string(),
            environments::CreateEnvironmentPayloadBuilder::new().build(),
        )
        .await;
    assert!(update_result.is_err());
}

#[cfg(test)]
#[tokio::test]
async fn test_get_environment_api_keys() {
    let novu = Novu::new("", None).unwrap();
    let api_keys_result = novu.get_environment_api_keys().await;
    assert!(api_keys_result.is_err());
}

#[cfg(test)]
#[tokio::test]
async fn test_regenerate_environment_api_keys() {
    let novu = Novu::new("", None).unwrap();
    let regenerate_api_keys_result = novu.regenerate_environment_api_keys().await;
    assert!(regenerate_api_keys_result.is_err());
}
