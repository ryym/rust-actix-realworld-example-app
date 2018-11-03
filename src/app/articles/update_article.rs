use super::get_article::CanGetArticle;
use super::replace_tags::CanReplaceTags;
use super::res;
use super::slugify::CanSlugify;
use super::ArticleChange;
use crate::db;
use crate::error::ErrorKindAuth;
use crate::mdl::{self, Article, User};
use crate::prelude::*;

add_hub_trait!(UpdateArticle);

pub trait CanUpdateArticle {
    fn update_article(
        &self,
        user: &User,
        slug: &str,
        change: ArticleChange,
    ) -> Result<res::Article>;
}

pub trait UpdateArticle: db::HaveConn + CanSlugify + CanGetArticle + CanReplaceTags {}
impl<T: UpdateArticle> CanUpdateArticle for T {
    fn update_article(
        &self,
        user: &User,
        slug: &str,
        change: ArticleChange,
    ) -> Result<res::Article> {
        use crate::schema::articles;
        use diesel::{self, prelude::*};

        let conn = self.conn();

        let article = articles::table
            .filter(articles::slug.eq(slug))
            .get_result::<Article>(conn)?;

        if article.author_id != user.id {
            return Err(ErrorKindAuth::Forbidden.into());
        }

        let tag_list = change.tag_list.unwrap_or(Vec::with_capacity(0));
        self.replace_tags(article.id, tag_list)?;

        let change = mdl::ArticleChange {
            slug: change.title.as_ref().map(|t| self.slugify(t)),
            title: change.title,
            description: change.description,
            body: change.body,
        };

        db::may_update(
            diesel::update(articles::table.filter(articles::id.eq(article.id)))
                .set(&change)
                .execute(conn),
        )?;

        // XXX: This queries the article again.
        self.get_article(&article.slug, Some(user))
    }
}
