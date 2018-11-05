use crate::mdl::NewFollower;
use crate::prelude::*;
use crate::schema::followers as flws;
use db::Conn;
use diesel::prelude::*;

pub fn insert(conn: &Conn, new_follower: &NewFollower) -> Result<()> {
    diesel::insert_into(flws::table)
        .values(new_follower)
        .on_conflict((flws::user_id, flws::follower_id))
        .do_nothing()
        .execute(conn)?;

    Ok(())
}

pub fn delete(conn: &Conn, user_id: i32, follower_id: i32) -> Result<()> {
    diesel::delete(
        flws::table
            .filter(flws::user_id.eq(user_id))
            .filter(flws::follower_id.eq(follower_id)),
    ).execute(conn)?;

    Ok(())
}
