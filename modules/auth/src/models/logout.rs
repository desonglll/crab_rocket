use crab_rocket_schema::DbPool;
use crab_rocket_user::services::user_service::UserService;
use rocket::serde::{Deserialize, Serialize};
use rocket::State;

use crate::models::log_trait::LogTrait;

use super::log_error::LogError;
use super::session::Session;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(crate = "rocket::serde")]
pub struct Logout {
    pub username: String,
    pub password: Option<String>,
}

impl Logout {
    pub fn logout(&self, pool: &State<DbPool>) -> Result<Session, LogError> {
        let user = UserService::get_by_username(pool, self.username.clone()).unwrap();
        match <Logout as LogTrait>::is_user_exists(pool, self.username.clone()) {
            Ok(_) => {
                let session = Session::get_session_by_uid(pool, user.user_id).unwrap();
                let result_session: Result<Session, diesel::result::Error> =
                    session.remove_session(pool);
                match result_session {
                    Ok(ok_result) => Ok(ok_result),
                    Err(e) => {
                        println!("{:?}", e.to_string());
                        Ok(Session::default())
                    }
                }
            }
            Err(e) => Err(e),
        }
    }
}

impl LogTrait for Logout {}

#[cfg(test)]
mod test {
    use crab_rocket_schema::{DbPool, establish_pool};
    use rocket::State;

    use crate::models::logout::Logout;

    #[test]
    fn test_logout_in_success() {
        // let mut connection = establish_pg_connection().expect("db connection error");
        // // 插入测试用户数据
        // diesel::insert_into(user_table::dsl::user_table)
        //     .values((
        //         user_table::dsl::user_id.eq(1),
        //         user_table::dsl::username.eq("john_doe"),
        //         user_table::dsl::password.eq("password1"),
        //     ))
        //     .execute(&mut connection)
        //     .expect("Failed to insert test user");

        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        let logout = Logout {
            username: "john_doe".to_string(),
            password: Some("password1".to_string()),
        };

        let result = logout.logout(pool).expect("Logout failed");
        println!("{:#?}", result);
        assert!(!result.session_id.is_nil());
    }
}
