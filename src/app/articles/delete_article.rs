use crate::db;
use crate::error::ErrorKindAuth;
use crate::hub::Hub;
use crate::mdl::User;
use crate::prelude::*;

impl CanDeleteArticle for Hub {}

pub trait CanDeleteArticle: db::HaveDb {
    fn delete_article(&self, user: &User, slug: &str) -> Result<()> {
        self.use_db(|conn| {
            use crate::schema::articles;
            use diesel::{self, prelude::*};

            let (id, author_id) = articles::table
                .filter(articles::slug.eq(slug))
                .select((articles::id, articles::author_id))
                .get_result::<(i32, i32)>(conn)?;

            if author_id != user.id {
                return Err(ErrorKindAuth::Forbidden.into());
            }

            diesel::delete(articles::table.filter(articles::id.eq(id))).execute(conn)?;

            Ok(())
        })
    }
}
