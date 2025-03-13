use reqwest::{header, Client, Error};
use serde_json::json;
use crate::models::{ApiResponse, User};

pub struct ApiClient {
    client: Client,
    base_url: String,
}

impl ApiClient {
    pub fn new(base_url: &str, token: &str) -> Self {
        let mut headers = header::HeaderMap::new();
        let auth_value = format!("Bearer {}", token);

        headers.insert(header::AUTHORIZATION, auth_value.parse().unwrap());
        headers.insert(header::CONTENT_TYPE, "application/json".parse().unwrap());
        let client = Client::builder().default_headers(headers).build().expect("Failed to create client");
        ApiClient {
            client,
            base_url: base_url.to_string(),
        }
    }

    pub async fn create_user(&self, username: &String) -> Result<(), Error> {
        let url = format!("{}/users", self.base_url);
        let limit: i64 = 16106127360;
        let payload = json!({
            "username": username,
            "trafficLimitStrategy": "DAY",
            "trafficLimitBytes": limit, // 15 GB
            "expireAt": serde_json::Value::Null
        });

        let response = self.client.post(&url).json(&payload).send().await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(Error::from(response.error_for_status().unwrap_err()))
        }
    }

    pub async fn get_user(&self, user_id: &str) -> Result<User, Error> {
        let url = format!("{}/users/username/{}", self.base_url, user_id);

        let response = self.client.get(&url).send().await?;

        if response.status().is_success() {
            let user: ApiResponse<User> = response.json().await?;
            Ok(user.response)
        } else {
            Err(Error::from(response.error_for_status().unwrap_err()))
        }
    }

    pub async fn renewal(&self, uuid: &str, expire_at: &str) -> Result<(), Error> {
        let url = format!("{}/users/update", self.base_url);
        let payload = json!({
            "uuid": uuid,
            "expireAt": expire_at
        });
        let response = self.client.post(&url).json(&payload).send().await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(Error::from(response.error_for_status().unwrap_err()))
        }
    }

    pub async fn select_inbound(&self, uuid: &str, inbound_uuid: &str) -> Result<(), Error> {
        let url = format!("{}/users/update", self.base_url);
        let payload = json!({
            "uuid": uuid,
            "activeUserInbounds": [
                inbound_uuid
            ]
        });
        let response = self.client.post(&url).json(&payload).send().await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(Error::from(response.error_for_status().unwrap_err()))
        }
    }
}