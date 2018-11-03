mod add_comment;
mod delete_comment;
mod list_comments;

use actix_web::{Json, Path, State};

use self::add_comment::CanAddComment;
use self::delete_comment::CanDeleteComment;
use self::list_comments::CanListComments;
use crate::app::res::{CommentListResponse, CommentResponse};
use crate::auth::Auth;
use crate::hub::Store;
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
    (store, auth, slug, form): (
        State<impl Store<Svc = S>>,
        Auth,
        Path<String>,
        Json<In<NewComment>>,
    ),
) -> Result<Json<CommentResponse>>
where
    S: CanAddComment,
{
    let comment = form.into_inner().comment;
    let comment = store.service()?.add_comment(&slug, auth.user, comment)?;
    Ok(Json(CommentResponse { comment }))
}

pub fn delete<S>(
    (store, auth, path): (State<impl Store<Svc = S>>, Auth, Path<(String, i32)>),
) -> Result<Json<()>>
where
    S: CanDeleteComment,
{
    let slug = &path.0;
    let comment_id = path.1;
    store
        .service()?
        .delete_comment(slug, &auth.user, comment_id)?;
    Ok(Json(()))
}

pub fn list<S>(
    (store, auth, slug): (State<impl Store<Svc = S>>, Option<Auth>, Path<String>),
) -> Result<Json<CommentListResponse>>
where
    S: CanListComments,
{
    let user = auth.map(|a| a.user);
    let comments = store.service()?.list_comments(&slug, user.as_ref())?;
    Ok(Json(CommentListResponse { comments }))
}
