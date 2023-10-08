pub mod changes;
pub mod client;
pub mod consts;
pub mod error;
pub mod events;
pub mod subscriber;

use client::Client;
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
        let result = self.client.post("/events/trigger", &data).await?;

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
