use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GetDeviceAuthorizationResponse {
    pub verification_uri_complete: String,
    pub device_code: String,
}
