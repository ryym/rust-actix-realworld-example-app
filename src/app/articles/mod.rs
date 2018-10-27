mod create_article;
mod get_article;

pub(self) use super::res;

use actix_web::{Json, Path, State};

use self::create_article::CanCreateArticle;
use self::get_article::CanGetArticle;
use super::res::ArticleResponse;
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
