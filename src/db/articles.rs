use crate::prelude::*;
use crate::schema::articles;
use db::{may_update, Conn};
use diesel::prelude::*;
use mdl::{Article, ArticleChange, NewArticle};

pub fn insert(conn: &Conn, article: &NewArticle) -> Result<Article> {
    diesel::insert_into(articles::table)
        .values(article)
        .get_result(conn)
        .map_err(|e| e.into())
}

pub fn update(conn: &Conn, id: i32, change: &ArticleChange) -> Result<()> {
    may_update(
        diesel::update(articles::table.filter(articles::id.eq(id)))
            .set(change)
            .execute(conn),
    )?;
    Ok(())
}

pub fn delete(conn: &Conn, id: i32) -> Result<()> {
    diesel::delete(articles::table.filter(articles::id.eq(id))).execute(conn)?;
    Ok(())
}
