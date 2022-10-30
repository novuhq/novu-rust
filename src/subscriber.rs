use serde::{Deserialize, Serialize};

use crate::{
    client::{Client, Response},
    error::NovuError,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriberPayload {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub avatar: Option<String>,
    pub subscriber_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscribersResponse {
    pub page: i32,
    pub total_count: i32,
    pub page_size: i32,
    pub data: Vec<String>,
}

pub struct Subscribers {
    client: Client,
}

impl Subscribers {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn list(&self, page: i32) -> Result<SubscribersResponse, NovuError> {
        let result: Response<SubscribersResponse> = self
            .client
            .get(format!("/subscribers/?page={}", page))
            .await?;

        match result {
            crate::client::Response::Success(data) => Ok(data.data),
            crate::client::Response::Error(err) => todo!("{:?}", err),
            crate::client::Response::Messages(err) => todo!("{:?}", err),
        }
    }

    // pub async fn identify(&self, data: SubscriberPayload) -> Result<reqwest::Response, NovuError> {
    //     let url = format!("{}/subscribers", self.backend_url);
    //     let response = self.client.post(&url).json(&data).send().await?;
    //     Ok(response)
    // }

    // pub async fn update(
    //     &self,
    //     subscriber_id: String,
    //     data: SubscriberPayload,
    // ) -> Result<reqwest::Response, NovuError> {
    //     let url = format!("{}/subscribers/{}", self.backend_url, subscriber_id);
    //     let response = self.client.put(&url).json(&data).send().await?;
    //     Ok(response)
    // }
}
