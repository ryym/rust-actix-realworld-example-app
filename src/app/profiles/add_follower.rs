use diesel::{self, prelude::*};

use super::{find_user, Profile};
use crate::db;
use crate::hub::Hub;
use crate::mdl::{NewFollower, User};
use crate::prelude::*;

impl AddFollower for Hub {}

pub trait CanAddFollower {
    fn add_follower(&self, username: &str, follower_id: i32) -> Result<Profile>;
}

pub trait AddFollower: db::HaveDb {}
impl<T: AddFollower> CanAddFollower for T {
    fn add_follower(&self, username: &str, follower_id: i32) -> Result<Profile> {
        let user = self.use_db(|conn| {
            let user = find_user(conn, username)?;
            insert_follower(conn, &user, follower_id)?;
            Ok(user)
        })?;

        Ok(Profile::from_user(user, true))
    }
}

// TODO: What if the user is already followed?
// TODO: Should not allow to follow oneself (case of user.id == follower_id)
fn insert_follower(conn: &db::Connection, user: &User, follower_id: i32) -> Result<()> {
    use crate::schema::followers;

    diesel::insert_into(followers::table)
        .values(&NewFollower {
            user_id: user.id,
            follower_id,
        }).execute(conn)?;

    Ok(())
}
