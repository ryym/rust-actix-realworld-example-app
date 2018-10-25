use diesel;

use super::{find_user, Profile};
use db;
use hub::Hub;
use prelude::*;

impl RemoveFollower for Hub {}

pub trait CanRemoveFollower {
    fn remove_follower(&self, username: &str, follower_id: i32) -> Result<Profile>;
}

pub trait RemoveFollower: db::HaveDb {}
impl<T: RemoveFollower> CanRemoveFollower for T {
    fn remove_follower(&self, username: &str, follower_id: i32) -> Result<Profile> {
        let user = self.use_db(|conn| {
            let user = find_user(conn, username)?;
            delete_follower(conn, user.id, follower_id)?;
            Ok(user)
        })?;

        Ok(Profile::from_user(user, false))
    }
}

fn delete_follower(conn: &db::Connection, user_id: i32, follower_id: i32) -> Result<()> {
    use diesel::prelude::*;
    use schema::followers as fl;

    diesel::delete(
        fl::table
            .filter(fl::user_id.eq(user_id))
            .filter(fl::follower_id.eq(follower_id)),
    ).execute(conn)?;

    Ok(())
}
