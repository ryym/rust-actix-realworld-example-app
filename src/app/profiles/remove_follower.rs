use super::Profile;
use crate::db;
use crate::prelude::*;

register_service!(CanRemoveFollower);

pub trait CanRemoveFollower: db::HaveConn {
    fn remove_follower(&self, username: &str, follower_id: i32) -> Result<Profile> {
        let user = db::users::find_by_name(self.conn(), username)?;
        db::followers::delete(self.conn(), user.id, follower_id)?;
        Ok(Profile::from_user(user, false))
    }
}
