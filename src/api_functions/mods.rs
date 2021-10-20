use crate::data_structures::{Mod, ModID, ModSearchResults, SearchSorting, API_PREFIX};

/// This API call searches Modrinth for your query with 2 additional optional parameters and
/// returns a `ModSearchResults` struct.
///
/// This function takes a `query`, an `Option<SearchSorting>` for how to sort the searches (by
/// default its sorted by relevance) and a `Option<usize>` as the limit which indicates how
/// many results to return at max.
pub fn search_mods(
    query: String,
    sorting: Option<SearchSorting>,
    limit: Option<usize>,
) -> Result<ModSearchResults, crate::Error> {
    let mut request = format!("{}/api/v1/mod?query={}", API_PREFIX, query);
    let sort: &'static str;

    if let Some(sorter) = sorting {
        sort = match sorter {
            SearchSorting::Relevance => "relevance",
            SearchSorting::Downloads => "downloads",
            SearchSorting::Updated => "updated",
            SearchSorting::Newest => "newest",
        };
    } else {
        sort = "relevance";
    }
    request = format!("{}&index={}", request, sort);

    if let Some(limit) = limit {
        request = format!("{}&limit={}", request, limit);
    }

    Ok(ureq::get(request.as_str()).call()?.into_json()?)
}

/// This API call gets the full details of a mod using its Mod ID, have a look at the `ModID`
/// struct.
pub fn mod_get(mod_id: ModID) -> Result<Mod, crate::Error> {
    let request = format!("{}/api/v1/mod/{}", API_PREFIX, mod_id.0);
    Ok(ureq::get(request.as_str()).call()?.into_json()?)
}
