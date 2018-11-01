use crate::{db, hub::Hub, prelude::*};

impl CanListTags for Hub {}

pub trait CanListTags: db::HaveDb {
    fn list_tags(&self) -> Result<Vec<String>> {
        self.use_db(|conn| {
            use crate::schema::article_tags::dsl::*;
            use diesel::prelude::*;

            let tags = article_tags.select(tag_name).distinct().load(conn)?;
            Ok(tags)
        })
    }
}
