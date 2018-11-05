use super::Profile;
use crate::db;
use crate::mdl::NewFollower;
use crate::prelude::*;

register_service!(CanAddFollower);

pub trait CanAddFollower: db::HaveConn {
    fn add_follower(&self, username: &str, follower_id: i32) -> Result<Profile> {
        let user = db::users::find_by_name(self.conn(), username)?;

        if user.id == follower_id {
            let msg = "You cannot follow yourself".to_owned();
            return Err(ErrorKind::Validation(vec![msg]).into());
        }

        db::followers::insert(
            self.conn(),
            &NewFollower {
                user_id: user.id,
                follower_id,
            },
        )?;

        Ok(Profile::from_user(user, true))
    }
}
