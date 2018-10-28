use super::res;
use db;
use hub::Hub;
use mdl::{Article, User};
use prelude::*;

impl CanGetArticle for Hub {}

pub trait CanGetArticle: db::HaveDb {
    fn get_article(&self, slug: &str, current: Option<&User>) -> Result<res::Article> {
        self.use_db(|conn| {
            use diesel::prelude::*;
            use schema::{articles, favorite_articles as fav_articles, users as authors};

            let (article, author) = articles::table
                .inner_join(authors::table)
                .filter(articles::slug.eq(slug))
                .get_result::<(Article, User)>(conn)?;

            let favorites_count = fav_articles::table
                .filter(fav_articles::article_id.eq(article.id))
                .count()
                .get_result::<i64>(conn)?;

            let (favorited, author_followed) = match current {
                Some(current) => find_favorite_and_following(conn, article.id, author.id, current)?,
                None => (false, false),
            };

            Ok(res::Article {
                slug: article.slug,
                title: article.title,
                description: article.description,
                body: article.body,
                tag_list: Vec::new(),
                created_at: res::DateTimeStr(article.created_at),
                updated_at: res::DateTimeStr(article.updated_at),
                favorited,
                favorites_count,
                author: res::Profile::from_user(author, author_followed),
            })
        })
    }
}

fn find_favorite_and_following(
    conn: &db::Connection,
    article_id: i32,
    author_id: i32,
    user: &User,
) -> Result<(bool, bool)> {
    use diesel::prelude::*;
    use schema::{favorite_articles as fav_articles, followers, users};

    let (_, fav_id, follow_id) = users::table
        .left_join(
            fav_articles::table.on(fav_articles::user_id
                .eq(users::id)
                .and(fav_articles::article_id.eq(article_id))),
        ).left_join(
            followers::table.on(followers::follower_id
                .eq(users::id)
                .and(followers::user_id.eq(author_id))),
        ).filter(users::id.eq(user.id))
        .select((
            users::id,
            fav_articles::id.nullable(),
            followers::id.nullable(),
        )).get_result::<(i32, Option<i32>, Option<i32>)>(conn)?;

    Ok((fav_id.is_some(), follow_id.is_some()))
}