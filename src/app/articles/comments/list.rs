use diesel::prelude::*;
use std::collections::HashSet;

use crate::app::res;
use crate::mdl::{Comment, User};
use crate::{db, hub::Hub, prelude::*};

impl CanListComments for Hub {}

pub trait CanListComments: db::HaveDb {
    fn list_comments(&self, slug: &str, user: Option<&User>) -> Result<Vec<res::Comment>> {
        self.use_db(|conn| {
            use crate::schema::{articles, comments, users};

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
                    select_followings(conn, user.id, &author_ids)?
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
        })
    }
}

// TODO: DRY (copied from app::articles::build_article_list).
fn select_followings(
    conn: &db::Connection,
    user_id: i32,
    author_ids: &[i32],
) -> Result<HashSet<i32>> {
    use crate::schema::followers as flws;

    let ids = flws::table
        .filter(flws::user_id.eq_any(author_ids))
        .filter(flws::follower_id.eq(user_id))
        .select(flws::user_id)
        .load::<i32>(conn)?;

    Ok(ids.into_iter().collect())
}
