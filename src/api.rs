use reqwest::{header, Client, Error};
use crate::models::User;

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

    /// GET-запрос для получения пользователей
    pub async fn get_users(&self) -> Result<Vec<User>, Error> {
        let url = format!("{}/users", self.base_url);
        let response = self.client
            .get(&url)
            .send()
            .await?
            .json::<Vec<User>>()
            .await?;

        Ok(response)
    }
}