use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetAccessTokenResponse {
    pub access_token: String,
    pub expires_in: u32,
    pub expires_at: String,
    pub token_type: String,
    pub client_id: String,
    pub internal_client: bool,
    pub client_service: Option<String>,
    pub product_ids: Option<String>,
    pub application_id: Option<String>,
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    pub account_id: Option<String>,
}
