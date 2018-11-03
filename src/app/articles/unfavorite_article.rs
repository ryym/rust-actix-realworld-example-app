use super::get_article::CanGetArticle;
use super::res;
use crate::db;
use crate::mdl::{Article, User};
use crate::prelude::*;

add_hub_trait!(UnfavoriteArticle);

pub trait CanUnfavoriteArticle {
    fn unfavorite_article(&self, user: &User, slug: &str) -> Result<res::Article>;
}

pub trait UnfavoriteArticle: db::HaveConn + CanGetArticle {}
impl<T: UnfavoriteArticle> CanUnfavoriteArticle for T {
    fn unfavorite_article(&self, user: &User, slug: &str) -> Result<res::Article> {
        use crate::schema::{articles, favorite_articles as fav_articles};
        use diesel::{self, prelude::*};

        let conn = self.conn();

        let article = articles::table
            .filter(articles::slug.eq(slug))
            .get_result::<Article>(conn)?;

        diesel::delete(
            fav_articles::table
                .filter(fav_articles::user_id.eq(user.id))
                .filter(fav_articles::article_id.eq(article.id)),
        ).execute(conn)?;

        // XXX: This queries the article again.
        self.get_article(&article.slug, Some(user))
    }
}
