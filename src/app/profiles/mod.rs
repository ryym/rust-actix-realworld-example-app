mod add_follower;
mod find_profile;

use actix_web::{Json, Path, State};

use self::add_follower::CanAddFollower;
use self::find_profile::CanFindProfile;
use auth::Auth;
use db;
use mdl::User;
use prelude::*;

fn find_user(conn: &db::Connection, username: &str) -> Result<User> {
    use diesel::prelude::*;
    use schema::users as u;

    let user = u::table.filter(u::username.eq(username)).first(conn)?;
    Ok(user)
}

#[derive(Debug, Serialize)]
pub struct Profile {
    username: String,
    bio: Option<String>,
    image: Option<String>,
    following: bool,
}

impl Profile {
    pub fn from_user(user: User, following: bool) -> Profile {
        Profile {
            username: user.username,
            bio: user.bio,
            image: user.image,
            following,
        }
    }
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

pub fn follow<S>(
    (hub, path, auth): (State<S>, Path<ProfilePath>, Auth),
) -> Result<Json<ProfileResponse>>
where
    S: CanAddFollower,
{
    let profile = hub.add_follower(&path.username, auth.user.id)?;
    Ok(Json(ProfileResponse { profile }))
}
