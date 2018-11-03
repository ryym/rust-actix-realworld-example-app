mod list_tags;

use actix_web::{Json, State};

use self::list_tags::CanListTags;
use crate::app::res::TagListResponse;
use crate::hub::Store;
use crate::prelude::*;

pub fn list<S>(store: State<impl Store<Svc = S>>) -> Result<Json<TagListResponse>>
where
    S: CanListTags,
{
    let hub = store.service()?;
    let tags = hub.list_tags()?;
    Ok(Json(TagListResponse { tags }))
}

#[cfg(test)]
mod test {
    use super::*;
    use actix_web::{test::TestRequest, FromRequest, State};
    use crate::test::{Mock, Store};

    #[test]
    fn list_returns_tag_response() -> Result<()> {
        impl CanListTags for Mock {
            fn list_tags(&self) -> Result<Vec<String>> {
                Ok(vec!["tag_a".to_owned(), "tag_b".to_owned()])
            }
        }

        let store = Store(Mock {});
        let req = TestRequest::with_state(store).finish();
        let state = State::extract(&req);
        let res = list(state)?;
        assert_eq!(res.tags, &["tag_a", "tag_b"]);

        Ok(())
    }
}
