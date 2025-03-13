use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ApiResponse<T> {
    pub response: T,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "uuid")]
    pub uuid: String,

    #[serde(rename = "shortUuid")]
    pub short_uuid: String,

    #[serde(rename = "status")]
    pub status: String,

    #[serde(rename = "expireAt")]
    pub expire_at: String,

    #[serde(rename = "subscriptionUrl")]
    pub subscription_url: String,
}