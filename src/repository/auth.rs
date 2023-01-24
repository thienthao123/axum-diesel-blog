use diesel::prelude::*;
use diesel::{PgConnection, QueryDsl, RunQueryDsl};

use crate::controllers::auth_controller::AuthError;
use crate::model::users::UserLogin;
use crate::model::User;

pub struct AuthRepository;
impl AuthRepository {
    pub fn login(conn: &mut PgConnection, auth: UserLogin) -> Result<User, AuthError> {
        use crate::schema::users::dsl::*;
        let user = users
            .filter(username.eq(auth.username))
            .filter(hashed_password.eq(auth.password))
            .get_result::<User>(conn)
            .map_err(|_| AuthError::WrongCredentials);
        user
    }
}
