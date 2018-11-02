pub mod comments;

mod build_article_list;
mod create_article;
mod delete_article;
mod favorite_article;
mod feed_articles;
mod get_article;
mod list_articles;
mod replace_tags;
mod slugify;
mod unfavorite_article;
mod update_article;

pub(self) use super::res;

use actix_web::{Json, Path, Query, State};

use self::create_article::CanCreateArticle;
use self::delete_article::CanDeleteArticle;
use self::favorite_article::CanFavoriteArticle;
use self::feed_articles::{CanFeedArticles, Params as FeedParams};
use self::get_article::CanGetArticle;
use self::list_articles::{CanListArticles, Params};
use self::unfavorite_article::CanUnfavoriteArticle;
use self::update_article::CanUpdateArticle;
use super::res::{ArticleListResponse, ArticleResponse};
use crate::auth::Auth;
use crate::db;
use crate::prelude::*;

#[derive(Debug, Deserialize)]
pub struct In<T> {
    article: T,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewArticle {
    title: String,
    description: String,
    body: String,
    tag_list: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArticleChange {
    pub title: Option<String>,
    pub description: Option<String>,
    pub body: Option<String>,
    pub tag_list: Option<Vec<String>>,
}

pub fn create<S>(
    (hub, auth, form): (State<S>, Auth, Json<In<NewArticle>>),
) -> Result<Json<ArticleResponse>>
where
    S: db::HaveDb + CanCreateArticle,
{
    let new_article = form.into_inner().article;
    let article = hub.use_db(|conn| hub.create_article(conn, auth.user, new_article))?;
    Ok(Json(ArticleResponse { article }))
}

pub fn get<S>(
    (hub, auth, slug): (State<S>, Option<Auth>, Path<String>),
) -> Result<Json<ArticleResponse>>
where
    S: db::HaveDb + CanGetArticle,
{
    let user = auth.map(|a| a.user);
    let article = hub.use_db(|conn| hub.get_article(conn, &slug, user.as_ref()))?;
    Ok(Json(ArticleResponse { article }))
}

pub fn update<S>(
    (hub, auth, slug, form): (State<S>, Auth, Path<String>, Json<In<ArticleChange>>),
) -> Result<Json<ArticleResponse>>
where
    S: db::HaveDb + CanUpdateArticle,
{
    let change = form.into_inner().article;
    let article = hub.use_db(|conn| hub.update_article(conn, &auth.user, &slug, change))?;
    Ok(Json(ArticleResponse { article }))
}

pub fn delete<S>((hub, auth, slug): (State<S>, Auth, Path<String>)) -> Result<Json<()>>
where
    S: db::HaveDb + CanDeleteArticle,
{
    hub.use_db(|conn| hub.delete_article(conn, &auth.user, &slug))?;
    Ok(Json(()))
}

pub fn favorite<S>(
    (hub, auth, slug): (State<S>, Auth, Path<String>),
) -> Result<Json<ArticleResponse>>
where
    S: db::HaveDb + CanFavoriteArticle,
{
    let article = hub.use_db(|conn| hub.favorite_article(conn, &auth.user, &slug))?;
    Ok(Json(ArticleResponse { article }))
}

pub fn unfavorite<S>(
    (hub, auth, slug): (State<S>, Auth, Path<String>),
) -> Result<Json<ArticleResponse>>
where
    S: db::HaveDb + CanUnfavoriteArticle,
{
    let article = hub.use_db(|conn| hub.unfavorite_article(conn, &auth.user, &slug))?;
    Ok(Json(ArticleResponse { article }))
}

pub fn list<S>(
    (hub, auth, params): (State<S>, Option<Auth>, Query<Params>),
) -> Result<Json<ArticleListResponse>>
where
    S: db::HaveDb + CanListArticles,
{
    let user = auth.map(|a| a.user);
    let articles =
        hub.use_db(|conn| hub.list_articles(conn, params.into_inner(), user.as_ref()))?;
    Ok(Json(ArticleListResponse::new(articles)))
}

pub fn feed<S>(
    (hub, auth, params): (State<S>, Auth, Query<FeedParams>),
) -> Result<Json<ArticleListResponse>>
where
    S: db::HaveDb + CanFeedArticles,
{
    let articles = hub.use_db(|conn| hub.feed_articles(conn, &auth.user, params.into_inner()))?;
    Ok(Json(ArticleListResponse::new(articles)))
}
