use crate::mdl::NewArticleTag;
use crate::{db, hub::Hub, prelude::*};

impl CanReplaceTags for Hub {}

pub trait CanReplaceTags {
    fn replace_tags(
        &self,
        conn: &db::Connection,
        article_id: i32,
        tags: Vec<String>,
    ) -> Result<Vec<String>> {
        use crate::schema::article_tags;
        use diesel::prelude::*;

        diesel::delete(article_tags::table.filter(article_tags::article_id.eq(article_id)))
            .execute(conn)?;

        let records = tags
            .into_iter()
            .map(|tag_name| NewArticleTag {
                article_id,
                tag_name,
            }).collect::<Vec<_>>();

        let tags = diesel::insert_into(article_tags::table)
            .values(&records)
            .returning(article_tags::tag_name)
            .get_results::<String>(conn)?;

        Ok(tags)
    }
}
