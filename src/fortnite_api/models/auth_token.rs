use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAccessTokenResponse {
    pub access_token: String,
}
