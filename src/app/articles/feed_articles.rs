use diesel::prelude::*;
use std::cmp;

use super::build_article_list::CanBuildArticleList;
use super::res;
use db;
use hub::Hub;
use mdl::{Article, User};
use prelude::*;

#[derive(Debug, Deserialize)]
pub struct Params {
    limit: Option<u32>,
    offset: Option<u32>,
}

impl CanFeedArticles for Hub {}

pub trait CanFeedArticles: db::HaveDb + CanBuildArticleList {
    fn feed_articles(&self, user: &User, params: Params) -> Result<Vec<res::Article>> {
        self.use_db(|conn| {
            let author_ids = select_followed_authors(conn, user.id)?;
            let articles = select_articles(conn, &author_ids, params)?;
            self.build_article_list(conn, articles, Some(user))
        })
    }
}

fn select_followed_authors(conn: &db::Connection, user_id: i32) -> Result<Vec<i32>> {
    use schema::followers as flws;

    let ids = flws::table
        .filter(flws::follower_id.eq(user_id))
        .select(flws::user_id)
        .load(conn)?;
    Ok(ids)
}

fn select_articles(
    conn: &db::Connection,
    author_ids: &[i32],
    p: Params,
) -> Result<Vec<(Article, User)>> {
    use schema::{articles, users};

    let limit = cmp::min(p.limit.unwrap_or(20), 500) as i64;
    let offset = cmp::min(p.offset.unwrap_or(0), 500) as i64;

    let records = articles::table
        .inner_join(users::table)
        .filter(users::id.eq_any(author_ids))
        .limit(limit)
        .offset(offset)
        .load(conn)?;

    Ok(records)
}
