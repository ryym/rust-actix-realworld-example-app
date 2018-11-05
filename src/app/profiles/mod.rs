mod add_follower;
mod find_profile;
mod remove_follower;

use actix_web::{Json, Path, State};

use self::add_follower::CanAddFollower;
use self::find_profile::CanFindProfile;
use self::remove_follower::CanRemoveFollower;
use super::res::{Profile, ProfileResponse};
use crate::auth::Auth;
use crate::hub::Store;
use crate::prelude::*;

#[derive(Debug, Deserialize)]
pub struct ProfilePath {
    username: String,
}

pub fn get<S>(
    (store, path, auth): (State<impl Store<Svc = S>>, Path<ProfilePath>, Option<Auth>),
) -> Result<Json<ProfileResponse>>
where
    S: CanFindProfile,
{
    let hub = store.service()?;
    let current_user = auth.map(|a| a.user);
    let profile = hub.find_profile(&path.username, current_user.as_ref())?;
    Ok(Json(ProfileResponse { profile }))
}

pub fn follow<S>(
    (store, path, auth): (State<impl Store<Svc = S>>, Path<ProfilePath>, Auth),
) -> Result<Json<ProfileResponse>>
where
    S: CanAddFollower,
{
    let svc = store.service()?;
    let profile = svc.add_follower(&path.username, auth.user.id)?;
    Ok(Json(ProfileResponse { profile }))
}

pub fn unfollow<S>(
    (store, path, auth): (State<impl Store<Svc = S>>, Path<ProfilePath>, Auth),
) -> Result<Json<ProfileResponse>>
where
    S: CanRemoveFollower,
{
    let svc = store.service()?;
    let profile = svc.remove_follower(&path.username, auth.user.id)?;
    Ok(Json(ProfileResponse { profile }))
}
