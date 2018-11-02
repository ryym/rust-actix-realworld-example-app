use super::get_article::CanGetArticle;
use super::res;
use crate::db;
use crate::hub::Hub;
use crate::mdl::{Article, User};
use crate::prelude::*;

impl CanUnfavoriteArticle for Hub {}

pub trait CanUnfavoriteArticle: CanGetArticle {
    fn unfavorite_article(
        &self,
        conn: &db::Connection,
        user: &User,
        slug: &str,
    ) -> Result<res::Article> {
        use crate::schema::{articles, favorite_articles as fav_articles};
        use diesel::{self, prelude::*};

        let article = articles::table
            .filter(articles::slug.eq(slug))
            .get_result::<Article>(conn)?;

        diesel::delete(
            fav_articles::table
                .filter(fav_articles::user_id.eq(user.id))
                .filter(fav_articles::article_id.eq(article.id)),
        ).execute(conn)?;

        // XXX: This queries the article again.
        self.get_article(conn, &article.slug, Some(user))
    }
}
