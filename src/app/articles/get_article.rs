use super::res;
use crate::db;
use crate::hub::Hub;
use crate::mdl::{Article, User};
use crate::prelude::*;

impl CanGetArticle for Hub {}

pub trait CanGetArticle {
    fn get_article(
        &self,
        conn: &db::Connection,
        slug: &str,
        current: Option<&User>,
    ) -> Result<res::Article> {
        use crate::schema::{articles, favorite_articles as fav_articles, users as authors};
        use diesel::prelude::*;

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

        let tags = select_tags(conn, article.id)?;

        Ok(res::Article {
            slug: article.slug,
            title: article.title,
            description: article.description,
            body: article.body,
            tag_list: tags,
            created_at: res::DateTimeStr(article.created_at),
            updated_at: res::DateTimeStr(article.updated_at),
            favorited,
            favorites_count,
            author: res::Profile::from_user(author, author_followed),
        })
    }
}

fn find_favorite_and_following(
    conn: &db::Connection,
    article_id: i32,
    author_id: i32,
    user: &User,
) -> Result<(bool, bool)> {
    use crate::schema::{favorite_articles as fav_articles, followers, users};
    use diesel::prelude::*;

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

fn select_tags(conn: &db::Connection, article_id: i32) -> Result<Vec<String>> {
    use crate::schema::article_tags;
    use diesel::prelude::*;

    let tags = article_tags::table
        .filter(article_tags::article_id.eq(article_id))
        .select(article_tags::tag_name)
        .load(conn)?;

    Ok(tags)
}
