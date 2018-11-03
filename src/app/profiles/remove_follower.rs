use super::{find_user, Profile};
use crate::db;
use crate::prelude::*;

register_service!(CanRemoveFollower);

pub trait CanRemoveFollower: db::HaveConn {
    fn remove_follower(&self, username: &str, follower_id: i32) -> Result<Profile> {
        let user = find_user(self.conn(), username)?;
        delete_follower(self.conn(), user.id, follower_id)?;
        Ok(Profile::from_user(user, false))
    }
}

fn delete_follower(conn: &db::Conn, user_id: i32, follower_id: i32) -> Result<()> {
    use crate::schema::followers as fl;
    use diesel::prelude::*;

    diesel::delete(
        fl::table
            .filter(fl::user_id.eq(user_id))
            .filter(fl::follower_id.eq(follower_id)),
    ).execute(conn)?;

    Ok(())
}
