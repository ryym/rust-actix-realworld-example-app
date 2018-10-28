use super::res;
use super::slugify::CanSlugify;
use super::NewArticle;
use db;
use hub::Hub;
use mdl;
use prelude::*;

impl CreateArticle for Hub {}

pub trait CanCreateArticle {
    fn create_article(&self, author: mdl::User, article: NewArticle) -> Result<res::Article>;
}

pub trait CreateArticle: db::HaveDb + CanSlugify {}
impl<T: CreateArticle> CanCreateArticle for T {
    fn create_article(&self, author: mdl::User, article: NewArticle) -> Result<res::Article> {
        let new_article = mdl::NewArticle {
            author_id: author.id,
            slug: self.slugify(&article.title),
            title: article.title,
            description: article.description,
            body: article.body,
        };

        let article = self.use_db(|conn| {
            // TODO: register tags.
            insert_article(conn, new_article)
        })?;

        Ok(res::Article {
            slug: article.slug,
            title: article.title,
            description: article.description,
            body: article.body,
            tag_list: Vec::new(),
            created_at: res::DateTimeStr(article.created_at),
            updated_at: res::DateTimeStr(article.updated_at),
            favorited: false,
            favorites_count: 0,
            author: res::Profile::from_user(author, false),
        })
    }
}

fn insert_article(conn: &db::Connection, article: mdl::NewArticle) -> Result<mdl::Article> {
    use diesel::{self, prelude::*};
    use schema::articles;

    let article = diesel::insert_into(articles::table)
        .values(&article)
        .get_result(conn)?;
    Ok(article)
}
