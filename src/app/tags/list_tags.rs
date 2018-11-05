use crate::{db, prelude::*};

register_service!(ListTags);

pub trait CanListTags {
    fn list_tags(&self) -> Result<Vec<String>>;
}

pub trait ListTags: db::HaveConn {}
impl<T: ListTags> CanListTags for T {
    // XXX: The tags should be ordered by articles count.
    fn list_tags(&self) -> Result<Vec<String>> {
        use crate::schema::article_tags::dsl::*;
        use diesel::prelude::*;

        let tags = article_tags.select(tag_name).distinct().load(self.conn())?;
        Ok(tags)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::password::HashedPassword;
    use crate::{db, mdl, test};

    #[test]
    fn list_all_tags() -> Result<()> {
        let t = test::init()?;

        let conn = t.db_conn()?;

        let author = db::users::insert(
            &conn,
            &mdl::NewUser {
                username: String::new(),
                email: String::new(),
                bio: None,
                image: None,
            },
            HashedPassword::new("test"),
        )?;

        let article = db::articles::insert(
            &conn,
            &mdl::NewArticle {
                author_id: author.id,
                slug: String::new(),
                title: String::new(),
                description: String::new(),
                body: String::new(),
            },
        )?;

        let tags = vec!["tag_a".to_owned(), "tag_b".to_owned()];
        db::articles::add_tags(&conn, article.id, tags.into_iter())?;

        struct Mock {
            conn: db::Conn,
        }
        impl_have_conn!(Mock(conn));
        impl ListTags for Mock {}

        let tags = Mock { conn }.list_tags()?;
        assert_eq!(&tags, &["tag_a", "tag_b"]);

        Ok(())
    }
}
