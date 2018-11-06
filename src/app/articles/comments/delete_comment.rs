use crate::{db, mdl::User, prelude::*};

register_service!(DeleteComment);

pub trait CanDeleteComment {
    fn delete_comment(&self, slug: &str, author: &User, comment_id: i32) -> Result<()>;
}

pub trait DeleteComment: db::HaveConn {}
impl<T: DeleteComment> CanDeleteComment for T {
    fn delete_comment(&self, slug: &str, author: &User, comment_id: i32) -> Result<()> {
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
            .get_result::<i32>(self.conn())?;

        db::comments::delete(self.conn(), comment_id)?;

        Ok(())
    }
}
