use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GetDeviceAuthorizationResponse {
    pub user_code: String,
    pub device_code: String,
    pub verification_uri: String,
    pub verification_uri_complete: String,
    pub prompt: String,
    pub expires_in: u32,
    pub interval: u32,
    pub client_id: String,
}
