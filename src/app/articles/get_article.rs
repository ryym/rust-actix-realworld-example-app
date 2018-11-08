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
        let author = insert_user(&conn, "author")?;
        let article = insert_article(&conn, &slug, author.id)?;

        let tags = vec!["a".to_owned(), "b".to_owned()];
        db::articles::add_tags(&conn, article.id, tags.iter().cloned())?;

        let res = Mock { conn }.get_article(&slug, None)?;

        let expected = res::Article::new_builder()
            .author(res::Profile::from_user(author, false))
            .article(article, 0, tags)
            .build();
        assert_eq!(res, expected);

        Ok(())
    }

    #[test]
    fn with_login() -> Result<()> {
        let t = test::init()?;
        let conn = t.db_conn()?;
        let slug = "rust-is-so-fun".to_owned();

        let author = insert_user(&conn, "author")?;
        let article = insert_article(&conn, &slug, author.id)?;
        let user = insert_user(&conn, "user")?;

        db::articles::favorite(&conn, article.id, user.id)?;
        db::followers::insert(
            &conn,
            &mdl::NewFollower {
                user_id: author.id,
                follower_id: user.id,
            },
        )?;

        let res = Mock { conn }.get_article(&slug, Some(&user))?;

        let expected = res::Article::new_builder()
            .author(res::Profile::from_user(author, true))
            .article(article, 1, Vec::with_capacity(0))
            .favorited(true)
            .build();
        assert_eq!(res, expected);

        Ok(())
    }

    fn insert_user(conn: &db::Conn, name: &str) -> Result<mdl::User> {
        let author = db::users::insert(
            &conn,
            &mdl::NewUser {
                username: name.to_owned(),
                email: format!("{}@a.a", name),
                ..Default::default()
            },
            HashedPassword::dummy(),
        )?;
        Ok(author)
    }

    fn insert_article(conn: &db::Conn, slug: &str, author_id: i32) -> Result<mdl::Article> {
        let article = db::articles::insert(
            &conn,
            &mdl::NewArticle {
                author_id,
                slug: slug.to_owned(),
                ..Default::default()
            },
        )?;
        Ok(article)
    }
}
