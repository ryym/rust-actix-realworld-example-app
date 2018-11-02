mod list_tags;

use actix_web::{Json, State};

use self::list_tags::CanListTags;
use crate::app::res::TagListResponse;
use crate::db;
use crate::prelude::*;

pub fn list<S>(hub: State<S>) -> Result<Json<TagListResponse>>
where
    S: db::HaveDb + CanListTags,
{
    let tags = hub.use_db(|conn| hub.list_tags(conn))?;
    Ok(Json(TagListResponse { tags }))
}
