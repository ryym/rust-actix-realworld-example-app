mod add_follower;
mod find_profile;
mod remove_follower;

use actix_web::{Json, Path, State};

use self::add_follower::CanAddFollower;
use self::find_profile::CanFindProfile;
use self::remove_follower::CanRemoveFollower;
use super::res::{Profile, ProfileResponse};
use crate::auth::Auth;
use crate::db;
use crate::mdl::User;
use crate::prelude::*;

fn find_user(conn: &db::Conn, username: &str) -> Result<User> {
    use crate::schema::users as u;
    use diesel::prelude::*;

    let user = u::table.filter(u::username.eq(username)).first(conn)?;
    Ok(user)
}

#[derive(Debug, Deserialize)]
pub struct ProfilePath {
    username: String,
}

pub fn get<S>(
    (hub, path, auth): (State<S>, Path<ProfilePath>, Option<Auth>),
) -> Result<Json<ProfileResponse>>
where
    S: db::HaveDb + CanFindProfile,
{
    let current_user = auth.map(|a| a.user);
    let profile =
        hub.use_db(|conn| hub.find_profile(conn, &path.username, current_user.as_ref()))?;
    Ok(Json(ProfileResponse { profile }))
}

pub fn follow<S>(
    (hub, path, auth): (State<S>, Path<ProfilePath>, Auth),
) -> Result<Json<ProfileResponse>>
where
    S: db::HaveDb + CanAddFollower,
{
    let profile = hub.use_db(|conn| hub.add_follower(conn, &path.username, auth.user.id))?;
    Ok(Json(ProfileResponse { profile }))
}

pub fn unfollow<S>(
    (hub, path, auth): (State<S>, Path<ProfilePath>, Auth),
) -> Result<Json<ProfileResponse>>
where
    S: db::HaveDb + CanRemoveFollower,
{
    let profile = hub.use_db(|conn| hub.remove_follower(conn, &path.username, auth.user.id))?;
    Ok(Json(ProfileResponse { profile }))
}
