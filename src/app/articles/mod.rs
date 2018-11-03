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
use crate::hub::Store;
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
    (store, auth, form): (State<impl Store<S>>, Auth, Json<In<NewArticle>>),
) -> Result<Json<ArticleResponse>>
where
    S: CanCreateArticle,
{
    let new_article = form.into_inner().article;
    let article = store.hub()?.create_article(auth.user, new_article)?;
    Ok(Json(ArticleResponse { article }))
}

pub fn get<S>(
    (store, auth, slug): (State<impl Store<S>>, Option<Auth>, Path<String>),
) -> Result<Json<ArticleResponse>>
where
    S: CanGetArticle,
{
    let user = auth.map(|a| a.user);
    let article = store.hub()?.get_article(&slug, user.as_ref())?;
    Ok(Json(ArticleResponse { article }))
}

pub fn update<S>(
    (store, auth, slug, form): (
        State<impl Store<S>>,
        Auth,
        Path<String>,
        Json<In<ArticleChange>>,
    ),
) -> Result<Json<ArticleResponse>>
where
    S: CanUpdateArticle,
{
    let change = form.into_inner().article;
    let article = store.hub()?.update_article(&auth.user, &slug, change)?;
    Ok(Json(ArticleResponse { article }))
}

pub fn delete<S>(
    (store, auth, slug): (State<impl Store<S>>, Auth, Path<String>),
) -> Result<Json<()>>
where
    S: CanDeleteArticle,
{
    store.hub()?.delete_article(&auth.user, &slug)?;
    Ok(Json(()))
}

pub fn favorite<S>(
    (store, auth, slug): (State<impl Store<S>>, Auth, Path<String>),
) -> Result<Json<ArticleResponse>>
where
    S: CanFavoriteArticle,
{
    let article = store.hub()?.favorite_article(&auth.user, &slug)?;
    Ok(Json(ArticleResponse { article }))
}

pub fn unfavorite<S>(
    (store, auth, slug): (State<impl Store<S>>, Auth, Path<String>),
) -> Result<Json<ArticleResponse>>
where
    S: CanUnfavoriteArticle,
{
    let article = store.hub()?.unfavorite_article(&auth.user, &slug)?;
    Ok(Json(ArticleResponse { article }))
}

pub fn list<S>(
    (store, auth, params): (State<impl Store<S>>, Option<Auth>, Query<Params>),
) -> Result<Json<ArticleListResponse>>
where
    S: CanListArticles,
{
    let user = auth.map(|a| a.user);
    let articles = store
        .hub()?
        .list_articles(params.into_inner(), user.as_ref())?;
    Ok(Json(ArticleListResponse::new(articles)))
}

pub fn feed<S>(
    (store, auth, params): (State<impl Store<S>>, Auth, Query<FeedParams>),
) -> Result<Json<ArticleListResponse>>
where
    S: CanFeedArticles,
{
    let articles = store
        .hub()?
        .feed_articles(&auth.user, params.into_inner())?;
    Ok(Json(ArticleListResponse::new(articles)))
}
