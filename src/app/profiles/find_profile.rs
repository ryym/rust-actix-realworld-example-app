use diesel::prelude::*;

use super::Profile;
use crate::db;
use crate::mdl::User;
use crate::prelude::*;

register_service!(CanFindProfile);

pub trait CanFindProfile: db::HaveConn {
    fn find_profile(&self, username: &str, current: Option<&User>) -> Result<Profile> {
        let user = db::users::find_by_name(self.conn(), username)?;
        let following = match current {
            None => false,
            Some(current) => is_follower(self.conn(), user.id, current.id)?,
        };

        Ok(Profile::from_user(user, following))
    }
}

fn is_follower(conn: &db::Conn, user_id: i32, follower_id: i32) -> Result<bool> {
    use crate::schema::followers as fl;

    let id = fl::table
        .filter(fl::user_id.eq(user_id))
        .filter(fl::follower_id.eq(follower_id))
        .select(fl::id)
        .first::<i32>(conn)
        .optional()?;

    Ok(id.is_some())
}
