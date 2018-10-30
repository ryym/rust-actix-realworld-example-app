use super::NewComment;
use crate::app::res;
use crate::db;
use crate::hub::Hub;
use crate::mdl::{self, Comment, User};
use crate::prelude::*;

impl CanAddComment for Hub {}

pub trait CanAddComment: db::HaveDb {
    fn add_comment(&self, slug: &str, author: User, comment: NewComment) -> Result<res::Comment> {
        let comment = self.use_db(|conn| {
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
            diesel::insert_into(comments::table)
                .values(&new_comment)
                .get_result::<Comment>(conn)
                .map_err(|e| e.into())
        })?;

        Ok(res::Comment::new(
            comment,
            res::Profile::from_user(author, false),
        ))
    }
}
