use diesel::prelude::*;
use diesel::{PgConnection, QueryDsl, RunQueryDsl};

use crate::auth::jwt;
use crate::model::users::UserLogin;
use crate::model::User;

pub struct AuthRepository;
impl AuthRepository {
    pub fn login(conn: &mut PgConnection, auth: UserLogin) -> Result<User, jwt::AuthError> {
        use crate::schema::users::dsl::*;
        let user = users
            .filter(username.eq(auth.username))
            .filter(hashed_password.eq(auth.password))
            .get_result::<User>(conn)
            .map_err(|_| jwt::AuthError::WrongCredentials);
        user
    }
}
