mod add;

use actix_web::{Json, Path, State};

use self::add::CanAddComment;
use crate::app::res::CommentResponse;
use crate::auth::Auth;
use crate::prelude::*;

#[derive(Debug, Deserialize)]
pub struct In<T> {
    comment: T,
}

#[derive(Debug, Deserialize)]
pub struct NewComment {
    body: String,
}

pub fn add<S>(
    (hub, auth, slug, form): (State<S>, Auth, Path<String>, Json<In<NewComment>>),
) -> Result<Json<CommentResponse>>
where
    S: CanAddComment,
{
    let comment = form.into_inner().comment;
    let comment = hub.add_comment(&slug, auth.user, comment)?;
    Ok(Json(CommentResponse { comment }))
}
