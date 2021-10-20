use crate::data_structures::{Hosts, ModID, ModSideRequirement};
use serde::{Deserialize, Serialize};

/// A Struct representing the results for a modrinth search api call.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ModSearchResults {
    /// A vector of `ModResult`s which represents all the results modrinth could find
    pub hits: Vec<ModResult>,
    /// Number of skipped results
    pub offset: usize,
    /// The limit to how many results were returned according to the limit you set
    /// (or the default limit of 10)
    pub limit: usize,
    /// Total results returned, including the ones displayed and not displayed
    pub total_hits: usize,
}

/// This is the struct returned by the search api call as a vector in the other `ModSearchResults`
/// struct.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ModResult {
    /// The mod id, prefixed with `local-`
    pub mod_id: String,
    //project_type: String,
    /// Vanity id for the mod
    pub slug: String,
    /// Username of the author of the mod
    pub author: String,
    /// The actual name of the mod itself
    pub title: String,
    /// A small description for the mod
    pub description: String,
    /// A vector representing the categories this mod is in
    pub categories: Vec<String>,
    /// A vector representing the minecraft versions which this mod supports
    pub versions: Vec<String>,
    /// Total number of downloads for this mod
    pub downloads: usize,
    /// Number of people who are currently following this mod
    pub follows: usize,
    /// URL for the mod's main page
    pub page_url: String,
    /// URL for the mod's icon
    pub icon_url: Option<String>,
    /// URL for the mod's author
    pub author_url: String,
    /// The date this mod was created, represented in the RFC 3339 format
    pub date_created: String,
    /// The date this mod was last modified, represented in the RFC 3339 format
    pub date_modified: String,
    /// The latest version of **minecraft** that this mod supports
    pub latest_version: String,
    /// The short string representation of this mod's license, for a longer representation you need
    /// to query the mod for a complete `Mod` struct, from there you can use the license field
    pub license: String,
    /// Client-side requirement for the mod
    pub client_side: ModSideRequirement,
    /// Server-side requirement for the mod
    pub server_side: ModSideRequirement,
    /// Hosts, this is literally pointless right now!
    pub host: Hosts,
}

#[allow(dead_code)]
impl ModResult {
    pub fn get_clean_id(&self) -> ModID {
        ModID(self.mod_id.replace("local-", ""))
    }
}
