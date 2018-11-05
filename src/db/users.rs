use crate::db::{may_update, Conn};
use crate::mdl::{CredentialChange, NewCredential, NewUser, User, UserChange};
use crate::password::HashedPassword;
use crate::prelude::*;
use crate::schema::{credentials, users};
use diesel::prelude::*;

pub fn insert(conn: &Conn, user: &NewUser, password: HashedPassword) -> Result<User> {
    conn.transaction(|| {
        let user = diesel::insert_into(users::table)
            .values(user)
            .get_result::<User>(conn)
            .context("register user")?;

        let cred = NewCredential {
            user_id: user.id,
            password_hash: password.into(),
        };
        diesel::insert_into(credentials::table)
            .values(cred)
            .execute(conn)
            .context("register credential")?;
        Ok(user)
    })
}

pub fn update(
    conn: &Conn,
    user_id: i32,
    user: &UserChange,
    new_password: Option<HashedPassword>,
) -> Result<Option<User>> {
    let cred = CredentialChange {
        password_hash: new_password.map(|p| p.into()),
    };
    conn.transaction(|| {
        may_update(
            diesel::update(credentials::table.filter(credentials::user_id.eq(user_id)))
                .set(cred)
                .execute(conn),
        )?;
        let user = may_update(
            diesel::update(users::table.filter(users::id.eq(user_id)))
                .set(user)
                .get_result(conn),
        )?;

        Ok(user)
    })
}

pub fn find_by_name(conn: &Conn, username: &str) -> Result<User> {
    let user = users::table
        .filter(users::username.eq(username))
        .first(conn)?;
    Ok(user)
}
