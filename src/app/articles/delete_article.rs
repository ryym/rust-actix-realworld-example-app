use db;
use hub::Hub;
use mdl::User;
use prelude::*;

impl CanDeleteArticle for Hub {}

pub trait CanDeleteArticle: db::HaveDb {
    fn delete_article(&self, user: &User, slug: &str) -> Result<()> {
        self.use_db(|conn| {
            use diesel::{self, prelude::*};
            use schema::articles;

            let (id, author_id) = articles::table
                .filter(articles::slug.eq(slug))
                .select((articles::id, articles::author_id))
                .get_result::<(i32, i32)>(conn)?;

            if author_id != user.id {
                return Err(ErrorKind::Auth.into());
            }

            diesel::delete(articles::table.filter(articles::id.eq(id))).execute(conn)?;

            Ok(())
        })
    }
}
