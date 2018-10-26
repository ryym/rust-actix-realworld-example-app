mod add_follower;
mod find_profile;
mod remove_follower;

use actix_web::{Json, Path, State};

use self::add_follower::CanAddFollower;
use self::find_profile::CanFindProfile;
use self::remove_follower::CanRemoveFollower;
use super::res::{Profile, ProfileResponse};
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

pub fn unfollow<S>(
    (hub, path, auth): (State<S>, Path<ProfilePath>, Auth),
) -> Result<Json<ProfileResponse>>
where
    S: CanRemoveFollower,
{
    let profile = hub.remove_follower(&path.username, auth.user.id)?;
    Ok(Json(ProfileResponse { profile }))
}
