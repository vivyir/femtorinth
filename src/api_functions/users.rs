use crate::data_structures::{User, UserID, API_PREFIX};

/// This API call gets the full details about a user while taking a `UserID` parameter and
/// returning a `User` struct.
pub fn user_get(user_id: UserID) -> Result<User, crate::Error> {
    let request = format!("{}/api/v1/user/{}", API_PREFIX, user_id.0);
    Ok(ureq::get(request.as_str()).call()?.into_json()?)
}
