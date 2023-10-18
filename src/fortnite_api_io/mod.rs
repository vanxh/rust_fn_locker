pub mod models;

use models::items::{GetItemsResponse, Item};

pub struct FortniteApiIo {
    api_key: String,
    url: &'static str,
    client: reqwest::Client,
}

impl FortniteApiIo {
    pub fn new(api_key: String) -> FortniteApiIo {
        FortniteApiIo {
            api_key,
            url: "https://fortniteapi.io",
            client: reqwest::Client::new(),
        }
    }

    pub async fn get_items(&self, url: &str) -> Result<Vec<Item>, Box<dyn std::error::Error>> {
        let url = format!("{}{}", self.url, url);

        let resp = self
            .client
            .get(url)
            .header("Authorization", &self.api_key)
            .send()
            .await?
            .json::<GetItemsResponse>()
            .await?;

        Ok(resp.items)
    }
}
