use diesel::prelude::*;

use super::{find_user, Profile};
use crate::db;
use crate::hub::Hub;
use crate::mdl::User;
use crate::prelude::*;

impl FindProfile for Hub {}

pub trait CanFindProfile {
    fn find_profile(&self, username: &str, current: Option<&User>) -> Result<Profile>;
}

pub trait FindProfile: db::HaveDb {}
impl<T: FindProfile> CanFindProfile for T {
    fn find_profile(&self, username: &str, current: Option<&User>) -> Result<Profile> {
        let (user, following) = self.use_db(|conn| {
            let user = find_user(conn, username)?;
            let following = match current {
                None => false,
                Some(current) => is_follower(conn, user.id, current.id)?,
            };

            Ok((user, following))
        })?;

        Ok(Profile::from_user(user, following))
    }
}

fn is_follower(conn: &db::Connection, user_id: i32, follower_id: i32) -> Result<bool> {
    use crate::schema::followers as fl;

    let id = fl::table
        .filter(fl::user_id.eq(user_id))
        .filter(fl::follower_id.eq(follower_id))
        .select(fl::id)
        .first::<i32>(conn)
        .optional()?;

    Ok(id.is_some())
}
