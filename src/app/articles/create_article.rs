use super::replace_tags::CanReplaceTags;
use super::res;
use super::slugify::CanSlugify;
use super::NewArticle;
use crate::db;
use crate::mdl;
use crate::prelude::*;

register_service!(CreateArticle);

pub trait CanCreateArticle {
    fn create_article(&self, author: mdl::User, article: NewArticle) -> Result<res::Article>;
}

pub trait CreateArticle: db::HaveConn + CanSlugify + CanReplaceTags {}
impl<T: CreateArticle> CanCreateArticle for T {
    fn create_article(&self, author: mdl::User, article: NewArticle) -> Result<res::Article> {
        let new_article = mdl::NewArticle {
            author_id: author.id,
            slug: self.slugify(&article.title),
            title: article.title,
            description: article.description,
            body: article.body,
        };
        let tag_list = article.tag_list;

        let article = db::articles::insert(self.conn(), &new_article)?;
        let tags = self.replace_tags(article.id, tag_list)?;

        let res = res::Article::new_builder()
            .author(res::Profile::from_user(author, false))
            .article(article, 0, tags)
            .build();

        Ok(res)
    }
}
