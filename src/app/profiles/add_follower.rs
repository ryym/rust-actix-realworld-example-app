use diesel::{self, prelude::*};

use super::{find_user, Profile};
use crate::db;
use crate::hub::Hub;
use crate::mdl::{NewFollower, User};
use crate::prelude::*;

impl CanAddFollower for Hub {}

pub trait CanAddFollower {
    fn add_follower(&self, conn: &db::Conn, username: &str, follower_id: i32) -> Result<Profile> {
        let user = find_user(conn, username)?;

        if user.id == follower_id {
            let msg = "You cannot follow yourself".to_owned();
            return Err(ErrorKind::Validation(vec![msg]).into());
        }

        insert_follower(conn, &user, follower_id)?;
        Ok(Profile::from_user(user, true))
    }
}

fn insert_follower(conn: &db::Conn, user: &User, follower_id: i32) -> Result<()> {
    use crate::schema::followers as flws;

    diesel::insert_into(flws::table)
        .values(&NewFollower {
            user_id: user.id,
            follower_id,
        }).on_conflict((flws::user_id, flws::follower_id))
        .do_nothing()
        .execute(conn)?;

    Ok(())
}
