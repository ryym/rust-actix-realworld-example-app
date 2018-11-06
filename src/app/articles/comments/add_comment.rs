use super::NewComment;
use crate::app::res;
use crate::db;
use crate::mdl::{self, User};
use crate::prelude::*;

register_service!(AddComment);

pub trait CanAddComment {
    fn add_comment(&self, slug: &str, author: User, comment: NewComment) -> Result<res::Comment>;
}

pub trait AddComment: db::HaveConn {}
impl<T: AddComment> CanAddComment for T {
    fn add_comment(&self, slug: &str, author: User, comment: NewComment) -> Result<res::Comment> {
        use crate::schema::articles;
        use diesel::prelude::*;

        let article_id = articles::table
            .filter(articles::slug.eq(slug))
            .select(articles::id)
            .get_result::<i32>(self.conn())?;

        let comment = db::comments::insert(
            self.conn(),
            &mdl::NewComment {
                article_id,
                user_id: author.id,
                body: comment.body,
            },
        )?;

        Ok(res::Comment::new(
            comment,
            res::Profile::from_user(author, false),
        ))
    }
}
