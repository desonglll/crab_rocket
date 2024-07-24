use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use rocket::State;

use crab_rocket_schema::{DbPool, establish_pg_connection};
use crab_rocket_schema::schema::user_table;
use crab_rocket_user::models::user::User;
use crab_rocket_user::services::user_service::UserService;
use crate::models::log_trait::LogTrait;

use super::log_error::LogError;
use super::session::Session;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(crate = "rocket::serde")]
pub struct Login {
    pub username: String,
    pub password: String,
}

impl Login {
    pub fn login(&self, pool: &State<DbPool>) -> Result<Session, LogError> {
        let user = UserService::get_by_username(pool, self.username.clone()).unwrap();
        match <Login as LogTrait>::is_user_exists(pool, self.username.clone()) {
            Ok(_) => match self.is_valid(pool) {
                Ok(_) => {
                    let session = Session::new(user.user_id, 1);
                    let result_session: Result<Session, diesel::result::Error> =
                        session.add_session(pool);
                    match result_session {
                        Ok(ok_result) => Ok(ok_result),
                        Err(e) => {
                            println!("{:?}", e.to_string());
                            Ok(Session::default())
                        }
                    }
                }
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        }
    }


    pub fn is_valid(&self, pool: &State<DbPool>) -> Result<bool, LogError> {
        let mut conn = establish_pg_connection(pool).expect("msg");
        let result: Result<User, diesel::result::Error> = user_table::dsl::user_table
            .filter(user_table::dsl::username.eq(&self.username))
            .filter(user_table::dsl::password.eq(&self.password))
            .first::<User>(&mut conn);
        match result {
            Ok(_) => Ok(true),
            Err(e) => {
                println!("{:#?}", e);
                Err(LogError::PasswordError)
            }
        }
    }
}

impl LogTrait for Login {}

#[cfg(test)]
mod tests {
    use crab_rocket_schema::establish_pool;

    use super::*;

    #[test]
    fn test_login_in_success() {
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
        let login = Login {
            username: "john_doe".to_string(),
            password: "password1".to_string(),
        };

        let result = login.login(pool).expect("Login failed");
        println!("{:#?}", result);
        assert!(!result.session_id.is_nil());
    }

    #[test]
    fn test_login_in_failure() {
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
        let login = Login {
            username: "john_doe".to_string(),
            password: "wrong_password".to_string(),
        };

        let result = login.login(pool);
        match result {
            Err(LogError::PasswordError) => assert!(true),
            Err(LogError::NotFound) => assert!(true),
            Ok(result) => assert_eq!(result.user_id, -1, "Expected session with invalid user"),
        }
    }

    #[test]
    fn test_user_exists() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        let login = Login {
            username: "john_doe".to_string(),
            password: "password1".to_string(),
        };

        let result = login.is_user_exists(pool);
        assert_eq!(result.unwrap(), true, "Expected user to exist");
    }

    #[test]
    fn test_user_does_not_exist() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        let login = Login {
            username: "user3".to_string(),
            password: "password3".to_string(),
        };

        let result = login.is_user_exists(pool);
        match result {
            Err(LogError::NotFound) => assert!(true),
            _ => assert!(false, "Expected LogError::NotFound"),
        }
    }

    #[test]
    fn test_valid_user() {
        // ('john_doe', 1, 'john.doe@example.com', 'password1', 'John Doe', 'https://example.com/avatar1.jpg', 'Hello, I am John Doe', '123-456-7890'),
        // ('jane_smith', 2, 'jane.smith@example.com', 'password2', 'Jane Smith', 'https://example.com/avatar2.jpg', 'Hi, I am Jane Smith', '987-654-3210'),
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        let login = Login {
            username: "john_doe".to_string(),
            password: "password1".to_string(),
        };

        // 将 mock 函数替换到 Login 实例中
        let result = login.is_valid(pool).unwrap();
        assert_eq!(result, true, "Expected user to be valid");
    }

    #[test]
    fn test_invalid_user() {
        // ('john_doe', 1, 'john.doe@example.com', 'password1', 'John Doe', 'https://example.com/avatar1.jpg', 'Hello, I am John Doe', '123-456-7890'),
        // ('jane_smith', 2, 'jane.smith@example.com', 'password2', 'Jane Smith', 'https://example.com/avatar2.jpg', 'Hi, I am Jane Smith', '987-654-3210'),
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        let login = Login {
            username: "jane_smith".to_string(),
            password: "password3".to_string(),
        };

        let result = login.is_valid(pool);
        match result {
            Err(LogError::PasswordError) => assert!(true),
            _ => assert!(false, "Expected LogError::NotFound"),
        }
    }
}
