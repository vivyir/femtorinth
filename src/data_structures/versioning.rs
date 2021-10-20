use crate::data_structures::ModReleaseType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{ModID, UserID, VersionID};

/// A struct containing the hashes for a file in a mod's specific version.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct VersionFile {
    /// (K, V) = (Hashing algorithm, Hash in hexadecimal)
    pub hashes: HashMap<String, String>,
    /// Direct URL to the file for download
    pub url: String,
    /// The filename for this file
    pub filename: String,
}

/// A full struct representing a version of the mod.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Version {
    /// The ID of the version, encoded as a base62 string
    pub id: VersionID,
    /// The ID of the mod this version is for
    pub mod_id: ModID,
    /// The ID of the author who published this version
    pub author_id: UserID,
    /// Whether this version is the featured version for this mod
    pub featured: bool,
    /// The name of this version
    pub name: String,
    /// The version number for the mod itself, will ideally follow semver but it's not required to
    /// follow semver
    pub version_number: String,
    /// The changelog for this version of the mod, if one exists that is
    pub changelog: Option<String>,
    /// Deprecated: A link to the changelog for this version of the mod
    pub changelog_url: Option<String>,
    /// The date that this version was published, once again in RFC 3339 representation
    pub date_published: String,
    /// The number of downloads this specific version has
    pub downloads: usize,
    /// The type of the release - `Alpha`, `Beta`, or `Release`
    pub version_type: ModReleaseType,
    /// A vector of lists available for download for this version, check out the documentation for
    /// the `VersionFile` struct
    pub files: Vec<VersionFile>,
    /// A list of specific version IDs of mods that this mod depends on, they can
    /// be downloaded using only the version ID which makes dependency management
    /// extremely easy.
    pub dependencies: Vec<VersionID>,
    /// A vector of versions of Minecraft that this specific version of the mod supports
    pub game_versions: Vec<String>,
    /// Mod loaders supported by this mod version
    pub loaders: Vec<String>,
}

impl Version {
    /// Get the version id from a version struct
    pub fn get_version_id(&self) -> VersionID {
        self.id.clone()
    }
}
