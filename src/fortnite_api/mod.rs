mod models;
use models::auth_token::GetAccessTokenResponse;
use models::device_authorization::GetDeviceAuthorizationResponse;

pub struct FortniteAPI;

impl FortniteAPI {
    pub fn new() -> FortniteAPI {
        FortniteAPI
    }

    pub async fn get_auth_token(
        &self,
        token: &str,
        body: &str,
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
