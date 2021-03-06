use diesel::prelude::*;
use std::cmp;

use super::build_article_list::CanBuildArticleList;
use super::res;
use crate::db;
use crate::mdl::{Article, User};
use crate::prelude::*;

#[derive(Debug, Deserialize)]
pub struct Params {
    limit: Option<u32>,
    offset: Option<u32>,
}

register_service!(FeedArticles);

pub trait CanFeedArticles {
    fn feed_articles(&self, user: &User, params: Params) -> Result<Vec<res::Article>>;
}

pub trait FeedArticles: db::HaveConn + CanBuildArticleList {}
impl<T: FeedArticles> CanFeedArticles for T {
    fn feed_articles(&self, user: &User, params: Params) -> Result<Vec<res::Article>> {
        let author_ids = select_followed_authors(self.conn(), user.id)?;
        let articles = select_articles(self.conn(), &author_ids, params)?;
        self.build_article_list(articles, Some(user))
    }
}

fn select_followed_authors(conn: &db::Conn, user_id: i32) -> Result<Vec<i32>> {
    use crate::schema::followers as flws;

    let ids = flws::table
        .filter(flws::follower_id.eq(user_id))
        .select(flws::user_id)
        .load(conn)?;
    Ok(ids)
}

fn select_articles(conn: &db::Conn, author_ids: &[i32], p: Params) -> Result<Vec<(Article, User)>> {
    use crate::schema::{articles, users};

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
