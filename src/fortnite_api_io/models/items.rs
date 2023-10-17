use serde::de::{self, Deserializer};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GetItemsResponse {
    pub result: bool,
    pub pages: u16,
    pub items: Vec<Item>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    pub id: String,
    #[serde(rename = "type")]
    pub item_type: ItemType,
    pub name: String,
    pub rarity: Rarity,
    pub series: Box<Option<Series>>,
    pub price: u16,
    #[serde(rename = "images")]
    pub icon: Images,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
    Exotic,
}

impl<'de> Deserialize<'de> for Rarity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let helper: RarityHelper = Deserialize::deserialize(deserializer)?;
        match helper.id.as_str() {
            "Common" => Ok(Rarity::Common),
            "Uncommon" => Ok(Rarity::Uncommon),
            "Rare" => Ok(Rarity::Rare),
            "Epic" => Ok(Rarity::Epic),
            "Legendary" => Ok(Rarity::Legendary),
            "Transcendent" => Ok(Rarity::Exotic),
            "unattainable" => Ok(Rarity::Common),
            _ => Err(de::Error::unknown_variant(
                &helper.id,
                &["Common", "Uncommon", "Rare", "Epic", "Legendary"],
            )),
        }
    }
}

#[derive(Deserialize)]
struct RarityHelper {
    id: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ItemType {
    Backpack,
    Outfit,
    Contrail,
    Emote,
    Glider,
    LoadingScreen,
    Music,
    Pet,
    Pickaxe,
    Spray,
    Toy,
    Banner,
    Emoji,
    Wrap,
    Style,
    Bundle,
    BattleBus,
    ItemAccess,
}

impl<'de> Deserialize<'de> for ItemType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let helper: ItemTypeHelper = Deserialize::deserialize(deserializer)?;
        match helper.id.as_str() {
            "backpack" => Ok(ItemType::Backpack),
            "outfit" => Ok(ItemType::Outfit),
            "contrail" => Ok(ItemType::Contrail),
            "emote" => Ok(ItemType::Emote),
            "glider" => Ok(ItemType::Glider),
            "loadingscreen" => Ok(ItemType::LoadingScreen),
            "music" => Ok(ItemType::Music),
            "pet" => Ok(ItemType::Pet),
            "pickaxe" => Ok(ItemType::Pickaxe),
            "spray" => Ok(ItemType::Spray),
            "toy" => Ok(ItemType::Toy),
            "bannertoken" => Ok(ItemType::Banner),
            "emoji" => Ok(ItemType::Emoji),
            "wrap" => Ok(ItemType::Wrap),
            "cosmeticvariant" => Ok(ItemType::Style),
            "bundle" => Ok(ItemType::Bundle),
            "battlebus" => Ok(ItemType::BattleBus),
            "itemaccess" => Ok(ItemType::ItemAccess),
            _ => Err(de::Error::unknown_variant(
                &helper.id,
                &[
                    "backpack",
                    "outfit",
                    "contrail",
                    "emote",
                    "glider",
                    "loadingscreen",
                    "music",
                    "pet",
                    "pickaxe",
                    "spray",
                    "toy",
                    "bannertoken",
                    "emoji",
                    "wrap",
                    "cosmeticvariant",
                    "bundle",
                    "battlebus",
                    "itemaccess",
                ],
            )),
        }
    }
}

#[derive(Deserialize)]
struct ItemTypeHelper {
    id: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Series {
    PlatformSeries,      // Gaming Legends
    ColumbusSeries,      // Star Wars
    MarvelSeries,        // Marvel
    CreatorCollabSeries, // Icon Series
    LavaSeries,          // Lava Legends
    CUBESeries,          // Cube Series
    FrozenSeries,        // Frozen Legends
    SlurpSeries,         // Slurp Legends
    ShadowSeries,        // Shadow Series
    DCUSeries,           // DC
}

impl<'de> Deserialize<'de> for Series {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let helper: SeriesHelper = Deserialize::deserialize(deserializer)?;
        match helper.id.as_str() {
            "PlatformSeries" => Ok(Series::PlatformSeries),
            "ColumbusSeries" => Ok(Series::ColumbusSeries),
            "MarvelSeries" => Ok(Series::MarvelSeries),
            "CreatorCollabSeries" => Ok(Series::CreatorCollabSeries),
            "LavaSeries" => Ok(Series::LavaSeries),
            "CUBESeries" => Ok(Series::CUBESeries),
            "FrozenSeries" => Ok(Series::FrozenSeries),
            "SlurpSeries" => Ok(Series::SlurpSeries),
            "ShadowSeries" => Ok(Series::ShadowSeries),
            "DCUSeries" => Ok(Series::DCUSeries),
            _ => Err(de::Error::unknown_variant(
                &helper.id,
                &[
                    "PlatformSeries",
                    "ColumbusSeries",
                    "MarvelSeries",
                    "CreatorCollabSeries",
                    "LavaSeries",
                    "CUBESeries",
                    "FrozenSeries",
                    "SlurpSeries",
                    "ShadowSeries",
                    "DCUSeries",
                ],
            )),
        }
    }
}

#[derive(Deserialize)]
struct SeriesHelper {
    id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Added {
    date: String,
    version: String,
}

#[derive(Serialize, Debug)]
pub struct Images {
    icon: Option<String>,
}

impl<'de> Deserialize<'de> for Images {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let helper: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Some(map) = helper.as_object() {
            if let Some(icon) = map.get("icon") {
                if let Some(icon_str) = icon.as_str() {
                    return Ok(Images {
                        icon: Some(icon_str.to_string()),
                    });
                }
            }
        }
        Ok(Images { icon: None })
    }
}
