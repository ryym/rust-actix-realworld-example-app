use super::get_article::CanGetArticle;
use super::res;
use crate::db;
use crate::mdl::{Article, User};
use crate::prelude::*;

register_service!(UnfavoriteArticle);

pub trait CanUnfavoriteArticle {
    fn unfavorite_article(&self, user: &User, slug: &str) -> Result<res::Article>;
}

pub trait UnfavoriteArticle: db::HaveConn + CanGetArticle {}
impl<T: UnfavoriteArticle> CanUnfavoriteArticle for T {
    fn unfavorite_article(&self, user: &User, slug: &str) -> Result<res::Article> {
        use crate::schema::articles;
        use diesel::prelude::*;

        let article = articles::table
            .filter(articles::slug.eq(slug))
            .get_result::<Article>(self.conn())?;

        db::articles::unfavorite(self.conn(), article.id, user.id)?;

        // XXX: This queries the article again.
        self.get_article(&article.slug, Some(user))
    }
}
