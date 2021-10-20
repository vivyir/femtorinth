use crate::data_structures::API_PREFIX;

/// This API call lists all the categories that Modrinth has
pub fn list_categories() -> Result<Vec<String>, crate::Error> {
    let request = format!("{}/api/v1/tag/category", API_PREFIX);
    Ok(ureq::get(request.as_str()).call()?.into_json()?)
}

/// This API call lists all the mod-loaders that Modrinth supports
pub fn list_loaders() -> Result<Vec<String>, crate::Error> {
    let request = format!("{}/api/v1/tag/loader", API_PREFIX);
    Ok(ureq::get(request.as_str()).call()?.into_json()?)
}

/// This API call lists all the game versions for minecraft that Modrinth supports
pub fn list_game_versions() -> Result<Vec<String>, crate::Error> {
    let request = format!("{}/api/v1/tag/game_version", API_PREFIX);
    Ok(ureq::get(request.as_str()).call()?.into_json()?)
}
