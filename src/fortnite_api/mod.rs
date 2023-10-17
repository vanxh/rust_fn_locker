mod models;

use models::auth_token::GetAccessTokenResponse;
use models::device_authorization::GetDeviceAuthorizationResponse;
use models::session::Session;
use serde_json;
use std::fs::{self, File};
use std::io::{self, Write};

pub struct FortniteAPI {
    pub session: Option<Session>,
}

impl FortniteAPI {
    pub fn new() -> Self {
        if let Ok(data) = fs::read_to_string("session.json") {
            if let Ok(session) = serde_json::from_str::<Session>(&data) {
                if !session.is_expired() {
                    return FortniteAPI {
                        session: Some(session),
                    };
                }
            }
        }

        FortniteAPI { session: None }
    }

    pub fn save_session(&self) -> io::Result<()> {
        if let Some(session) = &self.session {
            let data = serde_json::to_string(session)?;
            let mut file = File::create("session.json")?;
            file.write_all(data.as_bytes())?;
        }
        Ok(())
    }

    pub async fn get_auth_token(
        &mut self,
        token: &str,
        body: &str,
        login: bool,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let url = "https://account-public-service-prod.ol.epicgames.com/account/api/oauth/token";

        let client = reqwest::Client::new();

        let resp = client
            .post(url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Authorization", format!("basic {}", token))
            .body(body.to_string())
            .send()
            .await?
            .json::<GetAccessTokenResponse>()
            .await?;

        if login {
            let session = Session {
                access_token: resp.access_token.clone(),
                expires_in: resp.expires_in,
                expires_at: resp.expires_at.clone(),
                display_name: resp.display_name.clone().ok_or("No display name")?,
                account_id: resp.account_id.clone().ok_or("No account ID")?,
            };
            self.session = Some(session);
            self.save_session()?;
        }

        Ok(resp.access_token)
    }

    pub async fn get_device_authorization(
        &self,
        token: &str,
        body: &str,
    ) -> Result<GetDeviceAuthorizationResponse, Box<dyn std::error::Error>> {
        let url = "https://account-public-service-prod.ol.epicgames.com/account/api/oauth/deviceAuthorization";

        let client = reqwest::Client::new();

        let resp = client
            .post(url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .bearer_auth(token)
            .body(body.to_string())
            .send()
            .await?
            .json::<GetDeviceAuthorizationResponse>()
            .await?;

        Ok(resp)
    }
}
