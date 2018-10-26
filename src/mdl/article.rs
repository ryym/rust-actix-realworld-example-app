use chrono::NaiveDateTime;

use schema::articles;

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
