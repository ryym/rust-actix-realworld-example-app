use crate::db::{self, if_changed};
use crate::mdl::{User, UserChange};
use crate::password::CanHashPassword;
use crate::prelude::*;

register_service!(UpdateUser);

pub struct UserChanges {
    pub user: UserChange,
    pub new_password: Option<String>,
}

pub trait CanUpdateUser {
    fn update_user(&self, current: User, change: UserChanges) -> Result<User>;
}

pub trait UpdateUser: db::HaveConn + CanHashPassword {}
impl<T: UpdateUser> CanUpdateUser for T {
    fn update_user(&self, current: User, change: UserChanges) -> Result<User> {
        let given = change.user;
        let user_change = UserChange {
            username: if_changed(given.username, &current.username),
            email: if_changed(given.email, &current.email),
            bio: if_changed(given.bio, &current.bio),
            image: if_changed(given.image, &current.image),
        };

        let password_hash = match change.new_password {
            Some(ref pass) => Some(self.hash_password(pass)?),
            None => None,
        };
        let user = db::users::update(self.conn(), current.id, &user_change, password_hash)?;

        Ok(user.unwrap_or(current))
    }
}
