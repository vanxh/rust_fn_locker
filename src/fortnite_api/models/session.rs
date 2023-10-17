use chrono;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Session {
    pub access_token: String,
    pub expires_in: u32,
    pub expires_at: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub account_id: String,
}

impl Session {
    pub fn is_expired(&self) -> bool {
        let now = chrono::Utc::now();
        let expires_at = chrono::DateTime::parse_from_rfc3339(&self.expires_at).unwrap();
        now > expires_at
    }
}
