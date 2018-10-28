use diesel::prelude::*;
use std::cmp;

use super::build_article_list::CanBuildArticleList;
use super::res;
use db;
use hub::Hub;
use mdl::{Article, User};
use prelude::*;

impl CanListArticles for Hub {}

#[derive(Debug, Deserialize)]
pub struct Params {
    // TODO: Support tag parameter.
    // tag: Option<String>,
    author: Option<String>,
    favorited: Option<String>,
    limit: Option<u32>,
    offset: Option<u32>,
}

pub trait CanListArticles: db::HaveDb + CanBuildArticleList {
    fn list_articles(&self, params: Params, user: Option<&User>) -> Result<Vec<res::Article>> {
        self.use_db(|conn| {
            let articles = search_articles(conn, params)?;
            self.build_article_list(conn, articles, user)
        })
    }
}

fn search_articles(conn: &db::Connection, p: Params) -> Result<Vec<(Article, User)>> {
    use schema::{articles::dsl::*, users};

    let mut q = articles.inner_join(users::table).into_boxed();

    if let Some(ref author_name) = p.author {
        q = q.filter(users::username.eq(author_name));
    }

    if let Some(ref liker_name) = p.favorited {
        use schema::favorite_articles as favs;

        let favorited_ids = favs::table
            .inner_join(users::table)
            .filter(users::username.eq(liker_name))
            .select(favs::article_id)
            .load::<i32>(conn)?;

        q = q.filter(id.eq_any(favorited_ids));
    }

    let limit = cmp::min(p.limit.unwrap_or(20), 500) as i64;
    let offset = cmp::min(p.offset.unwrap_or(0), 500) as i64;

    let records = q
        .order(created_at.desc())
        .limit(limit)
        .offset(offset)
        .load::<(Article, User)>(conn)?;
    Ok(records)
}