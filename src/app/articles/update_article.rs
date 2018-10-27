use super::get_article::CanGetArticle;
use super::res;
use super::slugify::CanSlugify;
use super::ArticleChange;
use db;
use hub::Hub;
use mdl::{self, Article, User};
use prelude::*;

impl CanUpdateArticle for Hub {}

pub trait CanUpdateArticle: db::HaveDb + CanSlugify + CanGetArticle {
    fn update_article(
        &self,
        user: &User,
        slug: &str,
        change: ArticleChange,
    ) -> Result<res::Article> {
        let article = self.use_db(|conn| {
            use diesel::{self, prelude::*};
            use schema::articles;

            let article = articles::table
                .filter(articles::slug.eq(slug))
                .get_result::<Article>(conn)?;

            if article.author_id != user.id {
                return Err(ErrorKind::Auth.into());
            }

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

            Ok(article)
        })?;

        // XXX: This queries the article again.
        self.get_article(&article.slug, Some(user))
    }
}
