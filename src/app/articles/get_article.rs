use super::res;
use crate::db;
use crate::mdl::{Article, User};
use crate::prelude::*;

register_service!(GetArticle);

pub trait CanGetArticle {
    fn get_article(&self, slug: &str, current: Option<&User>) -> Result<res::Article>;
}

pub trait GetArticle: db::HaveConn {}
impl<T: GetArticle> CanGetArticle for T {
    fn get_article(&self, slug: &str, current: Option<&User>) -> Result<res::Article> {
        use crate::schema::{articles, favorite_articles as fav_articles, users as authors};
        use diesel::prelude::*;

        let conn = self.conn();

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

        let res = res::Article::new_builder()
            .author(res::Profile::from_user(author, author_followed))
            .article(article, favorites_count, tags)
            .favorited(favorited)
            .build();

        Ok(res)
    }
}

fn find_favorite_and_following(
    conn: &db::Conn,
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

fn select_tags(conn: &db::Conn, article_id: i32) -> Result<Vec<String>> {
    use crate::schema::article_tags;
    use diesel::prelude::*;

    let tags = article_tags::table
        .filter(article_tags::article_id.eq(article_id))
        .select(article_tags::tag_name)
        .load(conn)?;

    Ok(tags)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::password::HashedPassword;
    use crate::{app::res, db, mdl, test};

    struct Mock {
        conn: db::Conn,
    }
    impl_have_conn!(Mock(conn));
    impl GetArticle for Mock {}

    #[test]
    fn no_login_required() -> Result<()> {
        let t = test::init()?;
        let conn = t.db_conn()?;

        let slug = "rust-is-fun".to_owned();
        let author = db::users::insert(&conn, &mdl::NewUser::default(), HashedPassword::dummy())?;
        let article = db::articles::insert(
            &conn,
            &mdl::NewArticle {
                author_id: author.id,
                slug: slug.clone(),
                ..Default::default()
            },
        )?;

        let res = Mock { conn }.get_article(&slug, None)?;

        let expected = res::Article::new_builder()
            .author(res::Profile::from_user(author, false))
            .article(article, 0, Vec::with_capacity(0))
            .build();
        assert_eq!(res, expected);

        Ok(())
    }
}
