use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAthenaProfileResponse {
    #[serde(rename = "profileRevision")]
    pub profile_revision: u32,
    #[serde(rename = "profileId")]
    pub profile_id: String,
    #[serde(rename = "profileChangesBaseRevision")]
    pub profile_changes_base_revision: u32,
    #[serde(rename = "profileChanges")]
    pub profile_changes: Vec<ProfileChange>,
    #[serde(rename = "profileCommandRevision")]
    pub profile_command_revision: u32,
    #[serde(rename = "serverTime")]
    pub server_time: String,
    #[serde(rename = "responseVersion")]
    pub response_version: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProfileChange {
    #[serde(rename = "changeType")]
    pub change_type: String,
    #[serde(rename = "profile")]
    pub profile: Profile,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Profile {
    pub _id: String,
    pub created: String,
    pub updated: String,
    pub rvn: u32,
    #[serde(rename = "wipeNumber")]
    pub wipe_number: u32,
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "profileId")]
    pub profile_id: String,
    pub version: String,
    #[serde(rename = "stats")]
    pub stats: Stats,
    #[serde(rename = "commandRevision")]
    pub command_revision: u32,
    #[serde(rename = "items")]
    pub items: HashMap<String, Item>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Stats {
    #[serde(rename = "attributes")]
    pub attributes: StatAttributes,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StatAttributes {}

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    #[serde(rename = "templateId")]
    pub template_id: String,
    #[serde(rename = "quantity")]
    pub quantity: u32,
}
