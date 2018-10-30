mod add;
mod delete;

use actix_web::{Json, Path, State};

use self::add::CanAddComment;
use self::delete::CanDeleteComment;
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

pub fn delete<S>((hub, auth, path): (State<S>, Auth, Path<(String, i32)>)) -> Result<Json<()>>
where
    S: CanDeleteComment,
{
    let slug = &path.0;
    let comment_id = path.1;
    hub.delete_comment(slug, &auth.user, comment_id)?;
    Ok(Json(()))
}
