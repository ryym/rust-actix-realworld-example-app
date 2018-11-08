use chrono::NaiveDateTime;
use serde::ser::{Serialize, Serializer};

use crate::mdl;

#[derive(Debug, Serialize)]
pub struct User {
    pub email: String,
    pub token: String,
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
}

impl User {
    pub fn from_model(token: String, user: mdl::User) -> User {
        User {
            token,
            username: user.username,
            email: user.email,
            bio: user.bio,
            image: user.image,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub user: User,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Profile {
    username: String,
    bio: Option<String>,
    image: Option<String>,
    following: bool,
}

impl Profile {
    pub fn from_user(user: mdl::User, following: bool) -> Profile {
        Profile {
            username: user.username,
            bio: user.bio,
            image: user.image,
            following,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ProfileResponse {
    pub profile: Profile,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Article {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub tag_list: Vec<String>,
    pub created_at: DateTimeStr,
    pub updated_at: DateTimeStr,
    pub favorited: bool,
    pub favorites_count: i64,
    pub author: Profile,
}

impl Article {
    pub fn new_builder() -> ArticleBuilder<(), (), ()> {
        ArticleBuilder::new()
    }
}

// https://keens.github.io/blog/2017/02/09/rustnochottoyarisuginabuilderpata_n/
pub struct ArticleBuilder<ArticleT, AuthorT, TagsT> {
    article: ArticleT,
    author: AuthorT,
    favorited: bool,
    favorites_count: Option<i64>,
    tag_list: TagsT,
}

impl ArticleBuilder<(), (), ()> {
    fn new() -> Self {
        ArticleBuilder {
            article: (),
            author: (),
            favorited: false,
            favorites_count: None,
            tag_list: (),
        }
    }
}

impl<ArticleT, AuthorT, TagsT> ArticleBuilder<ArticleT, AuthorT, TagsT> {
    pub fn article(
        self,
        article: mdl::Article,
        favorites_count: i64,
        tag_list: Vec<String>,
    ) -> ArticleBuilder<mdl::Article, AuthorT, Vec<String>> {
        ArticleBuilder {
            article,
            author: self.author,
            favorited: self.favorited,
            favorites_count: Some(favorites_count),
            tag_list,
        }
    }

    pub fn author(self, author: Profile) -> ArticleBuilder<ArticleT, Profile, TagsT> {
        ArticleBuilder {
            article: self.article,
            author,
            favorited: self.favorited,
            favorites_count: self.favorites_count,
            tag_list: self.tag_list,
        }
    }

    pub fn favorited(self, favorited: bool) -> ArticleBuilder<ArticleT, AuthorT, TagsT> {
        ArticleBuilder { favorited, ..self }
    }
}

impl ArticleBuilder<mdl::Article, Profile, Vec<String>> {
    pub fn build(self) -> Article {
        Article {
            slug: self.article.slug,
            title: self.article.title,
            description: self.article.description,
            body: self.article.body,
            tag_list: self.tag_list,
            created_at: DateTimeStr(self.article.created_at),
            updated_at: DateTimeStr(self.article.updated_at),
            favorited: self.favorited,
            favorites_count: self.favorites_count.unwrap(),
            author: self.author,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ArticleResponse {
    pub article: Article,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArticleListResponse {
    pub articles: Vec<Article>,
    pub articles_count: u32,
}

impl ArticleListResponse {
    pub fn new(articles: Vec<Article>) -> ArticleListResponse {
        Self {
            articles_count: articles.len() as u32,
            articles,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    id: i32,
    created_at: DateTimeStr,
    updated_at: DateTimeStr,
    body: String,
    author: Profile,
}

impl Comment {
    pub fn new(comment: mdl::Comment, author: Profile) -> Comment {
        Comment {
            id: comment.id,
            created_at: DateTimeStr(comment.created_at),
            updated_at: DateTimeStr(comment.updated_at),
            body: comment.body,
            author,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct CommentResponse {
    pub comment: Comment,
}

#[derive(Debug, Serialize)]
pub struct CommentListResponse {
    pub comments: Vec<Comment>,
}

#[derive(Debug, Serialize)]
pub struct TagListResponse {
    pub tags: Vec<String>,
}

/// Default serialization of datetime string.
#[derive(Debug, PartialEq)]
pub struct DateTimeStr(pub NaiveDateTime);

impl Serialize for DateTimeStr {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let s = self.0.format("%Y-%m-%dT%H:%M:%S.%3fZ");
        serializer.serialize_str(&s.to_string())
    }
}
