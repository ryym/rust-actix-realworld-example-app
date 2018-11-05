use diesel::{self, prelude::*};

use super::Profile;
use crate::db;
use crate::mdl::{NewFollower, User};
use crate::prelude::*;

register_service!(CanAddFollower);

pub trait CanAddFollower: db::HaveConn {
    fn add_follower(&self, username: &str, follower_id: i32) -> Result<Profile> {
        let user = db::users::find_by_name(self.conn(), username)?;

        if user.id == follower_id {
            let msg = "You cannot follow yourself".to_owned();
            return Err(ErrorKind::Validation(vec![msg]).into());
        }

        insert_follower(self.conn(), &user, follower_id)?;
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
