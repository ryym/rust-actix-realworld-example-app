use crate::{db, prelude::*};

register_service!(ReplaceTags);

pub trait CanReplaceTags {
    fn replace_tags(&self, article_id: i32, tags: Vec<String>) -> Result<Vec<String>>;
}

pub trait ReplaceTags: db::HaveConn {}
impl<T: ReplaceTags> CanReplaceTags for T {
    fn replace_tags(&self, article_id: i32, tags: Vec<String>) -> Result<Vec<String>> {
        db::articles::delete_tags(self.conn(), article_id)?;
        db::articles::add_tags(self.conn(), article_id, tags.iter().cloned())?;

        Ok(tags)
    }
}
