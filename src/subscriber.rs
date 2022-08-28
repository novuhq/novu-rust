use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriberPayload {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub avatar: Option<String>,
    pub subscriber_id: Option<String>,
}

pub struct Subscribers {
    client: reqwest::Client,
    backend_url: String,
}

impl Subscribers {
    pub fn new(backend_url: String, client: reqwest::Client) -> Self {
        Self {
            client,
            backend_url,
        }
    }

    pub async fn list(&self, page: i32) -> Result<reqwest::Response, Box<dyn Error>> {
        let url = format!("{}/subscribers/?page={}", self.backend_url, page);
        let response = self.client.get(&url).send().await?;
        Ok(response)
    }

    pub async fn identify(
        &self,
        data: SubscriberPayload,
    ) -> Result<reqwest::Response, Box<dyn Error>> {
        let url = format!("{}/subscribers", self.backend_url);
        let response = self.client.post(&url).json(&data).send().await?;
        Ok(response)
    }

    pub async fn update(
        &self,
        subscriber_id: String,
        data: SubscriberPayload,
    ) -> Result<reqwest::Response, Box<dyn Error>> {
        let url = format!("{}/subscribers/{}", self.backend_url, subscriber_id);
        let response = self.client.put(&url).json(&data).send().await?;
        Ok(response)
    }
}
