use super::get_article::CanGetArticle;
use super::res;
use crate::db;
use crate::mdl::{Article, User};
use crate::prelude::*;

register_service!(FavoriteArticle);

pub trait CanFavoriteArticle {
    fn favorite_article(&self, user: &User, slug: &str) -> Result<res::Article>;
}

pub trait FavoriteArticle: db::HaveConn + CanGetArticle {}
impl<T: FavoriteArticle> CanFavoriteArticle for T {
    fn favorite_article(&self, user: &User, slug: &str) -> Result<res::Article> {
        use crate::schema::articles;
        use diesel::prelude::*;

        let article = articles::table
            .filter(articles::slug.eq(slug))
            .get_result::<Article>(self.conn())?;

        db::articles::favorite(self.conn(), article.id, user.id)?;

        // XXX: This queries the article again.
        self.get_article(&article.slug, Some(user))
    }
}
