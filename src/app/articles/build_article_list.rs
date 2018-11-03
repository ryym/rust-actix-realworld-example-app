use diesel::prelude::*;
use std::collections::{HashMap, HashSet};

use super::res;
use crate::db;
use crate::hub::Hub;
use crate::mdl::{Article, User};
use crate::prelude::*;

impl BuildArticleList for Hub {}

pub trait CanBuildArticleList {
    fn build_article_list(
        &self,
        articles: Vec<(Article, User)>,
        user: Option<&User>,
    ) -> Result<Vec<res::Article>>;
}

pub trait BuildArticleList: db::HaveConn {}

impl<T: BuildArticleList> CanBuildArticleList for T {
    fn build_article_list(
        &self,
        articles: Vec<(Article, User)>,
        user: Option<&User>,
    ) -> Result<Vec<res::Article>> {
        let (article_ids, author_ids): (Vec<_>, Vec<_>) =
            articles.iter().map(|(a, u)| (a.id, u.id)).unzip();

        let (favorites, followings, fav_counts) = match user {
            Some(user) => {
                let conn = self.conn();
                (
                    select_favorites(conn, user.id, &article_ids)?,
                    select_followings(conn, user.id, &author_ids)?,
                    select_favorite_counts(conn, &article_ids)?,
                )
            }
            None => (
                HashSet::with_capacity(0),
                HashSet::with_capacity(0),
                HashMap::with_capacity(0),
            ),
        };

        let results = articles
            .into_iter()
            .map(|(article, author)| {
                let following = followings.contains(&author.id);
                res::Article {
                    slug: article.slug,
                    title: article.title,
                    description: article.description,
                    body: article.body,
                    tag_list: Vec::new(),
                    created_at: res::DateTimeStr(article.created_at),
                    updated_at: res::DateTimeStr(article.updated_at),
                    favorited: favorites.contains(&article.id),
                    favorites_count: *fav_counts.get(&article.id).unwrap_or(&0),
                    author: res::Profile::from_user(author, following),
                }
            }).collect();

        Ok(results)
    }
}

fn select_favorites(conn: &db::Conn, user_id: i32, article_ids: &[i32]) -> Result<HashSet<i32>> {
    use crate::schema::favorite_articles as favs;

    let ids = favs::table
        .filter(favs::user_id.eq(user_id))
        .filter(favs::article_id.eq_any(article_ids))
        .select(favs::article_id)
        .load::<i32>(conn)?;

    Ok(ids.into_iter().collect())
}

fn select_followings(conn: &db::Conn, user_id: i32, author_ids: &[i32]) -> Result<HashSet<i32>> {
    use crate::schema::followers as flws;

    let ids = flws::table
        .filter(flws::user_id.eq_any(author_ids))
        .filter(flws::follower_id.eq(user_id))
        .select(flws::user_id)
        .load::<i32>(conn)?;

    Ok(ids.into_iter().collect())
}

fn select_favorite_counts(conn: &db::Conn, article_ids: &[i32]) -> Result<HashMap<i32, i64>> {
    use crate::schema::favorite_articles as favs;
    use diesel::{dsl::sql, sql_types::BigInt};

    // Unfortunately, currently diesel does not support `GROUP BY`.
    // https://github.com/diesel-rs/diesel/issues/210
    let fav_counts = favs::table
        .select((favs::article_id, sql::<BigInt>("count(*)")))
        .filter(favs::article_id.eq_any(article_ids))
        .filter(sql("TRUE GROUP BY article_id"))
        .load::<(i32, i64)>(conn)?;

    Ok(fav_counts.into_iter().collect())
}
