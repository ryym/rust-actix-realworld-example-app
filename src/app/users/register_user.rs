use super::SignupUser;
use crate::db;
use crate::mdl::{NewUser, User};
use crate::password::CanHashPassword;
use crate::prelude::*;

register_service!(RegisterUser);

pub trait CanRegisterUser {
    fn register_user(&self, form: &SignupUser) -> Result<User>;
}

pub trait RegisterUser: db::HaveConn + CanHashPassword {}
impl<T: RegisterUser> CanRegisterUser for T {
    fn register_user(&self, form: &SignupUser) -> Result<User> {
        let new_user = NewUser {
            username: form.username.clone(),
            email: form.email.clone(),
            bio: None,
            image: None,
        };
        let password_hash = self.hash_password(&form.password)?;
        db::users::insert(self.conn(), &new_user, password_hash)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::password::HashedPassword;
    use crate::{db, mdl, test};
    use diesel::prelude::*;

    #[test]
    fn register_with_hashed_password() -> Result<()> {
        let t = test::init()?;
        let conn = t.db_conn()?;

        struct Mock {
            conn: db::Conn,
        }
        impl_have_conn!(Mock(conn));

        impl CanHashPassword for Mock {
            fn hash_password(&self, password: &str) -> Result<HashedPassword> {
                Ok(HashedPassword::new(format!("{}-hashed", password)))
            }
        }
        impl RegisterUser for Mock {}

        let form = SignupUser {
            username: "hello".to_owned(),
            email: "hello@example.com".to_owned(),
            password: "password".to_owned(),
        };

        let mock = Mock { conn };
        mock.register_user(&form)?;

        let (user, cred) = {
            use crate::schema::{credentials, users};
            users::table
                .inner_join(credentials::table)
                .filter(users::username.eq(&form.username))
                .get_result::<(mdl::User, mdl::Credential)>(&mock.conn)
        }?;

        assert_eq!(form.email, user.email);
        assert_eq!(cred.password_hash, format!("{}-hashed", form.password));

        Ok(())
    }
}
