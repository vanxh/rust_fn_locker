use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GetItemsResponse {
    pub result: bool,
    pub pages: u32,
    pub items: Vec<Item>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    id: String,
    #[serde(rename = "type")]
    item_type: ItemType,
    name: String,
    description: String,
    rarity: Rarity,
    series: Option<Series>,
    price: u32,
    added: Added,
    built_in_emote: Option<String>,
    copyrighted_audio: Option<bool>,
    upcoming: bool,
    reactive: bool,
    release_date: Option<String>,
    last_appearance: Option<String>,
    interest: f64,
    images: Images,
    video: Option<String>,
    audio: Option<String>,
    path: Option<String>,
    gameplay_tags: Option<Vec<String>>,
    api_tags: Option<Vec<String>>,
    battlepass: Option<BattlePass>,
    set: Option<Set>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ItemType {
    id: String,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Rarity {
    id: String,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Series {
    id: String,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Added {
    date: String,
    version: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Images {
    icon: Option<String>,
    featured: Option<String>,
    background: Option<String>,
    icon_background: String,
    full_background: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Set {
    id: String,
    name: String,
    part_of: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct BattlePass {
    season: u32,
    tier: i32,
    page: Option<u32>,
    #[serde(rename = "type")]
    battle_pass_type: String,
    #[serde(rename = "displayText")]
    display_text: DisplayText,
    #[serde(rename = "battlePassName")]
    battle_pass_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct DisplayText {
    chapter: String,
    season: String,
    #[serde(rename = "chapterSeason")]
    chapter_season: String,
}
