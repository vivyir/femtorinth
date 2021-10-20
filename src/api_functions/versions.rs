use crate::data_structures::{ModID, Version, VersionID, API_PREFIX};

/// This API call gets all the available versions of a mod, takes a `ModID` and returns a
/// `Vec<Version>` inside a `Result`.
pub fn version_list(mod_id: ModID) -> Result<Vec<Version>, crate::Error> {
    let request = format!("{}/api/v1/mod/{}/version", API_PREFIX, mod_id.0);
    Ok(ureq::get(request.as_str()).call()?.into_json()?)
}

/// This API call gets a full `Version` struct using a `VersionID`.
pub fn version_get(version_id: VersionID) -> Result<Version, crate::Error> {
    let request = format!("{}/api/v1/version/{}", API_PREFIX, version_id.0);
    Ok(ureq::get(request.as_str()).call()?.into_json()?)
}
