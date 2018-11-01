mod list;

use actix_web::{Json, State};

use self::list::CanListTags;
use crate::app::res::TagListResponse;
use crate::prelude::*;

pub fn list<S>(hub: State<S>) -> Result<Json<TagListResponse>>
where
    S: CanListTags,
{
    let tags = hub.list_tags()?;
    Ok(Json(TagListResponse { tags }))
}
