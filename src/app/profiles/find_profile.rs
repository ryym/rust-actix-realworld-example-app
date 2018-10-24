use diesel::prelude::*;

use super::Profile;
use db;
use hub::Hub;
use mdl::User;
use prelude::*;

impl FindProfile for Hub {}

pub trait CanFindProfile {
    fn find_profile(&self, username: &str, current: Option<&User>) -> Result<Profile>;
}

pub trait FindProfile: db::HaveDb {}
impl<T: FindProfile> CanFindProfile for T {
    fn find_profile(&self, username: &str, current: Option<&User>) -> Result<Profile> {
        let (user, following) = self.use_db(|conn| {
            let user = find_user(conn, username)?;
            let following = match current {
                None => false,
                Some(current) => is_follower(conn, user.id, current.id)?,
            };

            Ok((user, following))
        })?;

        Ok(Profile {
            username: user.username,
            bio: user.bio,
            image: user.image,
            following,
        })
    }
}

fn find_user(conn: &db::Connection, username: &str) -> Result<User> {
    use schema::users as u;

    let user = u::table.filter(u::username.eq(username)).first(conn)?;

    Ok(user)
}

fn is_follower(conn: &db::Connection, user_id: i32, follower_id: i32) -> Result<bool> {
    use schema::followers as fl;

    let id = fl::table
        .filter(fl::user_id.eq(user_id))
        .filter(fl::follower_id.eq(follower_id))
        .select(fl::id)
        .first::<i32>(conn)
        .optional()?;

    Ok(id.is_some())
}
