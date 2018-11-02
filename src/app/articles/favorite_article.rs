use super::get_article::CanGetArticle;
use super::res;
use crate::db;
use crate::hub::Hub;
use crate::mdl::{Article, NewFavoriteArticle, User};
use crate::prelude::*;

impl CanFavoriteArticle for Hub {}

pub trait CanFavoriteArticle: CanGetArticle {
    fn favorite_article(
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

        let new_favorite = NewFavoriteArticle {
            user_id: user.id,
            article_id: article.id,
        };
        diesel::insert_into(fav_articles::table)
            .values(&new_favorite)
            .on_conflict((fav_articles::user_id, fav_articles::article_id))
            .do_nothing()
            .execute(conn)?;

        // XXX: This queries the article again.
        self.get_article(conn, &article.slug, Some(user))
    }
}
