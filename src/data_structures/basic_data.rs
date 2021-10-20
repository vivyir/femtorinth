use serde::{Deserialize, Serialize};
use thiserror::Error;

/// The modrinth api prefix
pub const API_PREFIX: &str = "https://api.modrinth.com";

/// A simple tuple struct to represent a modrinth Mod ID
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ModID(pub String);

/// A simple tuple struct to represent a modrinth Version ID
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct VersionID(pub String);

/// A simple tuple struct to represent a modrinth User ID
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserID(pub String);

/// Convenience enum to control sorting while searching
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum SearchSorting {
    Relevance,
    Downloads,
    Updated,
    Newest,
}

/// The main error enum used by the whole femtorinth crate.
///
/// This enum is the error enum returned by all the functions in femtorinth that return a result
/// alongside the actual value, click on the colored `(Error)` in each variant to go to the docs of
/// that specific error.
#[derive(Debug, Error)]
#[allow(clippy::large_enum_variant)] // ive got no clue how to actually fix this so im disabling it for now
pub enum Error {
    /// This has a `std::io::Error` inside it.
    #[error("got io error: ({0})")]
    IoError(#[from] std::io::Error),
    /// This one has a `ureq::Error` from the ureq crate.
    #[error("got network error: ({0})")]
    NetError(#[from] ureq::Error),
}

/// Hosts, this enum is literally pointless right now, don't concern yourself
/// with anything relating to this.
#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Hosts {
    /// Modrinth?!@#$?@#?%#$?^ 😳😳😳😳
    Modrinth,
}

/// Requirement of the mod on client-side/server-side, convenience enum.
#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum ModSideRequirement {
    Required,
    Optional,
    Unsupported,
}

/// The mod's status, quite self-explanatory.
#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum ModStatus {
    Approved,
    Rejected,
    Draft,
    Unlisted,
    Processing,
    Unknown,
}

/// Release stage of the mod.
#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum ModReleaseType {
    Alpha,
    Beta,
    Release,
}

/// The bigger way to represent a license.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct License {
    /// The license id of a mod a.k.a the shorter form of a license
    pub id: String,
    /// The long form of a license
    pub name: String,
    /// URL to the license
    pub url: String,
}

/// Donation link for a mod.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DonationLink {
    /// The short name for the donation platform
    pub id: String,
    /// The long name for the donation platform
    pub platform: String,
    /// URL for donating
    pub url: String,
}

/// A user's role on modrinth, have a look at the `User` struct.
#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    Developer,
    Moderator,
    Admin,
}

/// A user struct representing a modrinth user.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    /// The user's modrinth id
    pub id: UserID,
    /// The user's github id; only visible to the user themselves
    pub github_id: Option<u64>,
    /// The user's username
    pub username: String,
    /// The user's display name
    pub name: String,
    /// The user's email; only visible to the user themselves
    pub email: Option<String>,
    /// The user's avatar url; uses github's icons
    pub avatar_url: Option<String>,
    /// A description of the user
    pub bio: String,
    /// The time at which the user was created, RFC 3339 formatted
    pub created: String,
    /// The user's role, have a look at `UserRole`'s documentation
    pub role: UserRole,
}
