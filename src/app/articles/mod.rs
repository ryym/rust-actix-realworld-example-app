mod create_article;

pub(self) use super::res;

use actix_web::{Json, State};

use self::create_article::CanCreateArticle;
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
