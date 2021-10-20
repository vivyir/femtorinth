use crate::data_structures::{DonationLink, License, ModID, ModSideRequirement, ModStatus};
use serde::{Deserialize, Serialize};

use super::VersionID;

/// The most useful struct, probably, represents a specific mod in its entirety.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Mod {
    /// The ID of the mod, encoded as a base62 string
    pub id: ModID,
    /// The slug of a mod, used for vanity URLs
    pub slug: String,
    /// The id of the team that has ownership of this mod
    pub team: String,
    /// The title or name of the mod
    pub title: String,
    /// A short description of the mod
    pub description: String,
    /// A long form description of the mod
    pub body: String,
    /// Deprecated, the link to the long description of the mod
    pub body_url: Option<String>,
    /// The date at which the mod was first published, represented in RFC 3339
    pub published: String,
    /// The date at which the mod was updated, represented in RFC 3339
    pub updated: String,
    /// The status of the mod, see `ModStatus`' documentation
    pub status: ModStatus,
    /// The license of the mod, see `License`'s documentation
    pub license: License,
    /// Client-side requirement for the mod
    pub client_side: ModSideRequirement,
    /// Server-side requirement for the mod
    pub server_side: ModSideRequirement,
    /// The total number of downloads the mod has
    pub downloads: usize,
    /// A vector of the categories the mod is in
    pub categories: Vec<String>,
    /// A vector of IDs for versions of the mod
    pub versions: Vec<VersionID>,
    /// The URL for the icon of the mod, if it exists
    pub icon_url: Option<String>,
    /// An optional link to where you can submit bugs or issues with the mod
    pub issues_url: Option<String>,
    /// An optional link to the source code for the mod
    pub source_url: Option<String>,
    /// An optional link to the mod's wiki page or other relevant information
    pub wiki_url: Option<String>,
    /// An optional link to the mod's discord
    pub discord_url: Option<String>,
    /// An optional vector of all donation links the mod has
    pub donation_urls: Vec<DonationLink>,
}
