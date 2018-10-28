use super::get_article::CanGetArticle;
use super::res;
use crate::db;
use crate::hub::Hub;
use crate::mdl::{Article, User};
use crate::prelude::*;

impl CanUnfavoriteArticle for Hub {}

pub trait CanUnfavoriteArticle: db::HaveDb + CanGetArticle {
    fn unfavorite_article(&self, user: &User, slug: &str) -> Result<res::Article> {
        let article = self.use_db(|conn| {
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

            Ok(article)
        })?;

        // XXX: This queries the article again.
        self.get_article(&article.slug, Some(user))
    }
}
