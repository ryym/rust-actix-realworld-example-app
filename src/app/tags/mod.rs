mod list_tags;

use actix_web::{Json, State};

use self::list_tags::CanListTags;
use crate::app::res::TagListResponse;
use crate::db;
use crate::prelude::*;

pub fn list<S>(hub: State<S>) -> Result<Json<TagListResponse>>
where
    S: db::HaveDb + CanListTags,
{
    let tags = hub.use_db(|conn| hub.list_tags(conn))?;
    Ok(Json(TagListResponse { tags }))
}

#[cfg(test)]
mod test {
    use super::*;
    use actix_web::{test::TestRequest, FromRequest, State};
    use crate::{db, test};

    #[test]
    fn list_returns_tag_response() -> Result<()> {
        struct Mock {
            conn: db::Conn,
        }
        impl_have_db!(Mock(conn));

        impl CanListTags for Mock {
            fn list_tags(&self, _: &db::Conn) -> Result<Vec<String>> {
                Ok(vec!["tag_a".to_owned(), "tag_b".to_owned()])
            }
        }

        let t = test::init()?;

        let mock = Mock { conn: t.db_conn()? };
        let req = TestRequest::with_state(mock).finish();
        let state = State::extract(&req);
        let res = list(state)?;
        assert_eq!(res.tags, &["tag_a", "tag_b"]);

        Ok(())
    }
}
