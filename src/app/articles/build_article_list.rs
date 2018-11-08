use diesel::prelude::*;
use std::collections::{HashMap, HashSet};

use super::res;
use crate::db;
use crate::mdl::{Article, User};
use crate::prelude::*;

register_service!(BuildArticleList);

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

        let conn = self.conn();
        let (followings, favorites, fav_counts) = match user {
            Some(user) => (
                db::followers::filter_followee_ids(conn, user.id, &author_ids)?.collect(),
                select_favorites(conn, user.id, &article_ids)?,
                select_favorite_counts(conn, &article_ids)?,
            ),
            None => (
                HashSet::with_capacity(0),
                HashSet::with_capacity(0),
                HashMap::with_capacity(0),
            ),
        };

        let mut tag_lists = select_tags(conn, &article_ids)?;

        let results = articles
            .into_iter()
            .map(|(article, author)| {
                let following = followings.contains(&author.id);
                let favorites_count = *fav_counts.get(&article.id).unwrap_or(&0);
                let tags = tag_lists.remove(&article.id).unwrap_or(vec![]);
                res::Article::new_builder()
                    .author(res::Profile::from_user(author, following))
                    .favorited(favorites.contains(&article.id))
                    .article(article, favorites_count, tags)
                    .build()
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

fn select_favorite_counts(conn: &db::Conn, article_ids: &[i32]) -> Result<HashMap<i32, i64>> {
    use crate::schema::favorite_articles as favs;
    use diesel::{dsl::sql, sql_types::BigInt};

    // Unfortunately, currently diesel does not support `GROUP BY`.
    // https://github.com/diesel-rs/diesel/issues/210
    let fav_counts = favs::table
        .select((favs::article_id, sql::<BigInt>("COUNT(*)")))
        .filter(favs::article_id.eq_any(article_ids))
        .filter(sql("TRUE GROUP BY article_id"))
        .load::<(i32, i64)>(conn)?;

    Ok(fav_counts.into_iter().collect())
}

fn select_tags(conn: &db::Conn, article_ids: &[i32]) -> Result<HashMap<i32, Vec<String>>> {
    use crate::schema::article_tags;

    let rows = article_tags::table
        .select((article_tags::article_id, article_tags::tag_name))
        .filter(article_tags::article_id.eq_any(article_ids))
        .load::<(i32, String)>(conn)?;

    let id_to_tags = rows.into_iter().fold(HashMap::new(), |mut map, row| {
        map.entry(row.0).or_insert(Vec::new()).push(row.1);
        map
    });

    Ok(id_to_tags)
}
