use crate::{db, hub::Hub, prelude::*};

impl ListTags for Hub {}

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
    use crate::schema::{article_tags, articles, users};
    use crate::{db, mdl, test};
    use diesel::prelude::*;

    #[test]
    fn list_all_tags() -> Result<()> {
        let t = test::init()?;

        let conn = t.db_conn()?;

        let author_id = diesel::insert_into(users::table)
            .values(&mdl::NewUser {
                username: String::new(),
                email: String::new(),
                bio: None,
                image: None,
            }).returning(users::id)
            .get_result::<i32>(&conn)?;

        let article_id = diesel::insert_into(articles::table)
            .values(mdl::NewArticle {
                author_id,
                slug: String::new(),
                title: String::new(),
                description: String::new(),
                body: String::new(),
            }).returning(articles::id)
            .get_result::<i32>(&conn)?;

        let tags = vec![
            mdl::NewArticleTag {
                article_id,
                tag_name: "tag_a".to_owned(),
            },
            mdl::NewArticleTag {
                article_id,
                tag_name: "tag_b".to_owned(),
            },
        ];
        diesel::insert_into(article_tags::table)
            .values(&tags)
            .execute(&conn)?;

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
