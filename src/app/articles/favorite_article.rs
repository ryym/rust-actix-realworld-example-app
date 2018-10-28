use super::get_article::CanGetArticle;
use super::res;
use db;
use hub::Hub;
use mdl::{Article, NewFavoriteArticle, User};
use prelude::*;

impl CanFavoriteArticle for Hub {}

pub trait CanFavoriteArticle: db::HaveDb + CanGetArticle {
    fn favorite_article(&self, user: &User, slug: &str) -> Result<res::Article> {
        let article = self.use_db(|conn| {
            use diesel::{self, prelude::*};
            use schema::{articles, favorite_articles as fav_articles};

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

            Ok(article)
        })?;

        // XXX: This queries the article again.
        self.get_article(&article.slug, Some(user))
    }
}
