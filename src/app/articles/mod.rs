mod build_article_list;
mod create_article;
mod delete_article;
mod favorite_article;
mod get_article;
mod list_articles;
mod slugify;
mod unfavorite_article;
mod update_article;

pub(self) use super::res;

use actix_web::{Json, Path, Query, State};

use self::create_article::CanCreateArticle;
use self::delete_article::CanDeleteArticle;
use self::favorite_article::CanFavoriteArticle;
use self::get_article::CanGetArticle;
use self::list_articles::{CanListArticles, Params};
use self::unfavorite_article::CanUnfavoriteArticle;
use self::update_article::CanUpdateArticle;
use super::res::{ArticleListResponse, ArticleResponse};
use auth::Auth;
use prelude::*;

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
pub struct ArticleChange {
    pub title: Option<String>,
    pub description: Option<String>,
    pub body: Option<String>,
}

pub fn create_article<S>(
    (hub, auth, form): (State<S>, Auth, Json<In<NewArticle>>),
) -> Result<Json<ArticleResponse>>
where
    S: CanCreateArticle,
{
    let new_article = form.into_inner().article;
    let article = hub.create_article(auth.user, new_article)?;
    Ok(Json(ArticleResponse { article }))
}

pub fn get_article<S>(
    (hub, auth, slug): (State<S>, Option<Auth>, Path<String>),
) -> Result<Json<ArticleResponse>>
where
    S: CanGetArticle,
{
    let user = auth.map(|a| a.user);
    let article = hub.get_article(&slug, user.as_ref())?;
    Ok(Json(ArticleResponse { article }))
}

pub fn update_article<S>(
    (hub, auth, slug, form): (State<S>, Auth, Path<String>, Json<In<ArticleChange>>),
) -> Result<Json<ArticleResponse>>
where
    S: CanUpdateArticle,
{
    let change = form.into_inner().article;
    let article = hub.update_article(&auth.user, &slug, change)?;
    Ok(Json(ArticleResponse { article }))
}

pub fn delete_article<S>((hub, auth, slug): (State<S>, Auth, Path<String>)) -> Result<Json<()>>
where
    S: CanDeleteArticle,
{
    hub.delete_article(&auth.user, &slug)?;
    Ok(Json(()))
}

pub fn favorite_article<S>(
    (hub, auth, slug): (State<S>, Auth, Path<String>),
) -> Result<Json<ArticleResponse>>
where
    S: CanFavoriteArticle,
{
    let article = hub.favorite_article(&auth.user, &slug)?;
    Ok(Json(ArticleResponse { article }))
}

pub fn unfavorite_article<S>(
    (hub, auth, slug): (State<S>, Auth, Path<String>),
) -> Result<Json<ArticleResponse>>
where
    S: CanUnfavoriteArticle,
{
    let article = hub.unfavorite_article(&auth.user, &slug)?;
    Ok(Json(ArticleResponse { article }))
}

pub fn list_articles<S>(
    (hub, auth, params): (State<S>, Option<Auth>, Query<Params>),
) -> Result<Json<ArticleListResponse>>
where
    S: CanListArticles,
{
    let user = auth.map(|a| a.user);
    let articles = hub.list_articles(params.into_inner(), user.as_ref())?;
    Ok(Json(ArticleListResponse::new(articles)))
}
