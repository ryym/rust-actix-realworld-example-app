use super::password::CanHashPassword;
use crate::db;
use crate::hub::Hub;
use crate::mdl::{CredentialChange, User, UserChange};
use crate::prelude::*;

impl UpdateUser for Hub {}

pub struct UserChanges {
    pub user: UserChange,
    pub new_password: Option<String>,
}

pub trait CanUpdateUser {
    fn update_user(
        &self,
        conn: &db::Connection,
        current: User,
        change: UserChanges,
    ) -> Result<User>;
}

pub trait UpdateUser: CanHashPassword {}
impl<T: UpdateUser> CanUpdateUser for T {
    fn update_user(
        &self,
        conn: &db::Connection,
        current: User,
        change: UserChanges,
    ) -> Result<User> {
        use crate::schema::{
            credentials::dsl::*,
            users::{self, dsl::*},
        };
        use diesel::prelude::*;

        let given = change.user;
        let user_change = UserChange {
            username: if_changed(given.username, &current.username),
            email: if_changed(given.email, &current.email),
            bio: if_changed(given.bio, &current.bio),
            image: if_changed(given.image, &current.image),
        };

        let cred_change = CredentialChange {
            password_hash: match change.new_password {
                Some(ref pass) => Some(self.hash_password(pass)?),
                None => None,
            },
        };

        conn.transaction(|| {
            db::may_update(
                diesel::update(credentials.filter(user_id.eq(current.id)))
                    .set(cred_change)
                    .execute(conn),
            )?;

            let user = db::may_update(
                diesel::update(users.filter(users::id.eq(current.id)))
                    .set(user_change)
                    .get_result(conn),
            )?.unwrap_or(current);

            Ok(user)
        })
    }
}

fn if_changed<T: PartialEq>(new: Option<T>, old: &T) -> Option<T> {
    new.and_then(|new| if new != *old { Some(new) } else { None })
}
