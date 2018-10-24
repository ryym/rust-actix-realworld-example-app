mod find_profile;

use actix_web::{Json, Path, State};

use self::find_profile::CanFindProfile;
use auth::Auth;
use prelude::*;

#[derive(Debug, Serialize)]
pub struct Profile {
    username: String,
    bio: Option<String>,
    image: Option<String>,
    following: bool,
}

#[derive(Debug, Serialize)]
pub struct ProfileResponse {
    profile: Profile,
}

#[derive(Debug, Deserialize)]
pub struct ProfilePath {
    username: String,
}

pub fn get_profile<S>(
    (hub, path, auth): (State<S>, Path<ProfilePath>, Option<Auth>),
) -> Result<Json<ProfileResponse>>
where
    S: CanFindProfile,
{
    let current_user = auth.map(|a| a.user);
    let profile = hub.find_profile(&path.username, current_user.as_ref())?;
    Ok(Json(ProfileResponse { profile }))
}
