use crate::db;
use crate::error::ErrorKindAuth;
use crate::mdl::User;
use crate::prelude::*;

register_service!(DeleteArticle);

pub trait CanDeleteArticle {
    fn delete_article(&self, user: &User, slug: &str) -> Result<()>;
}

pub trait DeleteArticle: db::HaveConn {}
impl<T: DeleteArticle> CanDeleteArticle for T {
    fn delete_article(&self, user: &User, slug: &str) -> Result<()> {
        use crate::schema::articles;
        use diesel::{self, prelude::*};

        let (id, author_id) = articles::table
            .filter(articles::slug.eq(slug))
            .select((articles::id, articles::author_id))
            .get_result::<(i32, i32)>(self.conn())?;

        if author_id != user.id {
            return Err(ErrorKindAuth::Forbidden.into());
        }

        diesel::delete(articles::table.filter(articles::id.eq(id))).execute(self.conn())?;

        Ok(())
    }
}
