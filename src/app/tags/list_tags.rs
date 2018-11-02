use crate::{db, hub::Hub, prelude::*};

impl CanListTags for Hub {}

pub trait CanListTags {
    fn list_tags(&self, conn: &db::Connection) -> Result<Vec<String>> {
        use crate::schema::article_tags::dsl::*;
        use diesel::prelude::*;

        let tags = article_tags.select(tag_name).distinct().load(conn)?;
        Ok(tags)
    }
}
