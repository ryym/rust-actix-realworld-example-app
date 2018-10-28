use chrono::NaiveDateTime;

use schema::{articles, favorite_articles};

#[derive(Debug, Queryable)]
pub struct Article {
    pub id: i32,
    pub author_id: i32,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "articles"]
pub struct NewArticle {
    pub author_id: i32,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
}

// XXX: One can create an ArticleChange with title but without slug.
// It should be avoided.
#[derive(Debug, AsChangeset)]
#[table_name = "articles"]
pub struct ArticleChange {
    pub slug: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub body: Option<String>,
}

#[derive(Debug, Insertable)]
#[table_name = "favorite_articles"]
pub struct NewFavoriteArticle {
    pub user_id: i32,
    pub article_id: i32,
}
