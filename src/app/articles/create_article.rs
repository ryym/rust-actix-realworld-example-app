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

        Ok(res::Article {
            slug: article.slug,
            title: article.title,
            description: article.description,
            body: article.body,
            tag_list: tags,
            created_at: res::DateTimeStr(article.created_at),
            updated_at: res::DateTimeStr(article.updated_at),
            favorited: false,
            favorites_count: 0,
            author: res::Profile::from_user(author, false),
        })
    }
}
