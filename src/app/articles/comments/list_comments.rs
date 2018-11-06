use diesel::prelude::*;
use std::collections::HashSet;

use crate::app::res;
use crate::mdl::{Comment, User};
use crate::{db, prelude::*};

register_service!(ListComments);

pub trait CanListComments {
    fn list_comments(&self, slug: &str, user: Option<&User>) -> Result<Vec<res::Comment>>;
}

pub trait ListComments: db::HaveConn {}
impl<T: ListComments> CanListComments for T {
    fn list_comments(&self, slug: &str, user: Option<&User>) -> Result<Vec<res::Comment>> {
        use crate::schema::{articles, comments, users};

        let conn = self.conn();

        let article_id = articles::table
            .filter(articles::slug.eq(slug))
            .select(articles::id)
            .get_result::<i32>(conn)?;

        let comments = comments::table
            .inner_join(users::table)
            .filter(comments::article_id.eq(article_id))
            .load::<(Comment, User)>(conn)?;

        let followings = match user {
            Some(user) => {
                let author_ids = comments.iter().map(|(_, u)| u.id).collect::<Vec<_>>();
                db::followers::filter_followee_ids(conn, user.id, &author_ids)?.collect()
            }
            None => HashSet::with_capacity(0),
        };

        let comments = comments
            .into_iter()
            .map(|(c, u)| {
                let following = followings.contains(&u.id);
                let author = res::Profile::from_user(u, following);
                res::Comment::new(c, author)
            }).collect();

        Ok(comments)
    }
}
