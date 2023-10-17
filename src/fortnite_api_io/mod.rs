mod models;
use models::items::{GetItemsResponse, Item};

pub struct FortniteApiIo {
    api_key: String,
    url: &'static str,
}

impl FortniteApiIo {
    pub fn new(api_key: String) -> FortniteApiIo {
        FortniteApiIo {
            api_key,
            url: "https://fortniteapi.io",
        }
    }

    pub async fn get_items(&self, url: &str) -> Result<Vec<Item>, Box<dyn std::error::Error>> {
        let url = format!("{}{}", self.url, url);

        let client = reqwest::Client::new();

        let resp = client
            .get(url)
            .header("Authorization", self.api_key.clone())
            .send()
            .await?
            .json::<GetItemsResponse>()
            .await?;

        Ok(resp.items)
    }
}
