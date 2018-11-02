use crate::{db, hub::Hub, mdl::User, prelude::*};

impl CanDeleteComment for Hub {}

pub trait CanDeleteComment {
    fn delete_comment(
        &self,
        conn: &db::Connection,
        slug: &str,
        author: &User,
        comment_id: i32,
    ) -> Result<()> {
        use crate::schema::{articles, comments};
        use diesel::prelude::*;

        // Check if the given comment is
        // - associated with the specified article.
        // - written by the authenticated user.
        let comment_id = articles::table
            .inner_join(comments::table)
            .filter(articles::slug.eq(slug))
            .filter(comments::id.eq(comment_id))
            .filter(comments::user_id.eq(author.id))
            .select(comments::id)
            .get_result::<i32>(conn)?;

        diesel::delete(comments::table.filter(comments::id.eq(comment_id))).execute(conn)?;

        Ok(())
    }
}
