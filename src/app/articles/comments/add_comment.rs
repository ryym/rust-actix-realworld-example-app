use super::NewComment;
use crate::app::res;
use crate::db;
use crate::hub::Hub;
use crate::mdl::{self, Comment, User};
use crate::prelude::*;

impl CanAddComment for Hub {}

pub trait CanAddComment {
    fn add_comment(
        &self,
        conn: &db::Connection,
        slug: &str,
        author: User,
        comment: NewComment,
    ) -> Result<res::Comment> {
        use crate::schema::{articles, comments};
        use diesel::prelude::*;

        let article_id = articles::table
            .filter(articles::slug.eq(slug))
            .select(articles::id)
            .get_result::<i32>(conn)?;

        let new_comment = mdl::NewComment {
            article_id,
            user_id: author.id,
            body: comment.body,
        };

        let comment = diesel::insert_into(comments::table)
            .values(&new_comment)
            .get_result::<Comment>(conn)?;

        Ok(res::Comment::new(
            comment,
            res::Profile::from_user(author, false),
        ))
    }
}
