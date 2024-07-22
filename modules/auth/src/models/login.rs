use crab_rocket_schema::establish_pg_connection;
use crab_rocket_schema::schema::user_table;
use crab_rocket_user::models::user::User;
use diesel::prelude::*;

use super::session::Session;

pub struct Login {
    pub user_id: i32,
    pub username: String,
    pub password: String,
}

impl Login {
    pub fn login(&self) -> Result<Session, Box<dyn std::error::Error>> {
        if self.is_user_exists() && self.is_valid() {
            let session = Session::new(self.user_id);
            let result_session = Session::add_session(session);
            match result_session {
                Ok(ok_result) => Ok(ok_result),
                Err(e) => Err(e),
            }
        } else {
            Ok(Session::new(-1))
        }
    }
    pub fn is_user_exists(&self) -> bool {
        match establish_pg_connection() {
            Ok(mut conn) => {
                let result = user_table::dsl::user_table
                    .filter(user_table::dsl::user_id.eq(self.user_id))
                    .filter(user_table::dsl::username.eq(&self.username))
                    .first::<User>(&mut conn);
                match result {
                    Ok(_) => true,
                    Err(e) => {
                        println!("{:#?}", e);
                        false
                    }
                }
            }
            Err(e) => {
                println!("{:?}", e);
                false
            }
        }
    }

    pub fn is_valid(&self) -> bool {
        match establish_pg_connection() {
            Ok(mut conn) => {
                let result = user_table::dsl::user_table
                    .filter(user_table::dsl::user_id.eq(self.user_id))
                    .filter(user_table::dsl::username.eq(&self.username))
                    .filter(user_table::dsl::password.eq(&self.password))
                    .first::<User>(&mut conn);
                match result {
                    Ok(_) => true,
                    Err(e) => {
                        println!("{:#?}", e);
                        false
                    }
                }
            }
            Err(e) => {
                println!("{:?}", e);
                false
            }
        }
    }
}

#[cfg(test)]
mod tests {
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

        let login = Login {
            user_id: 1,
            username: "john_doe".to_string(),
            password: "password1".to_string(),
        };

        let result = login.login().expect("Login failed");
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

        let login = Login {
            user_id: 1,
            username: "john_doe".to_string(),
            password: "wrong_password".to_string(),
        };

        let result = login.login().expect("Login failed");
        assert_eq!(result.user_id, -1, "Expected session with invalid user");
    }
    #[test]
    fn test_user_exists() {
        let login = Login {
            user_id: 1,
            username: "john_doe".to_string(),
            password: "password1".to_string(),
        };

        let result = login.is_user_exists();
        assert_eq!(result, true, "Expected user to exist");
    }

    #[test]
    fn test_user_does_not_exist() {
        let login = Login {
            user_id: 99,
            username: "user3".to_string(),
            password: "password3".to_string(),
        };

        let result = login.is_user_exists();
        assert_eq!(result, false, "Expected user not to exist");
    }
    #[test]
    fn test_valid_user() {
        // ('john_doe', 1, 'john.doe@example.com', 'password1', 'John Doe', 'https://example.com/avatar1.jpg', 'Hello, I am John Doe', '123-456-7890'),
        // ('jane_smith', 2, 'jane.smith@example.com', 'password2', 'Jane Smith', 'https://example.com/avatar2.jpg', 'Hi, I am Jane Smith', '987-654-3210'),
        let login = Login {
            user_id: 1,
            username: "john_doe".to_string(),
            password: "password1".to_string(),
        };

        // 将 mock 函数替换到 Login 实例中
        let result = login.is_valid();
        assert_eq!(result, true, "Expected user to be valid");
    }

    #[test]
    fn test_invalid_user() {
        // ('john_doe', 1, 'john.doe@example.com', 'password1', 'John Doe', 'https://example.com/avatar1.jpg', 'Hello, I am John Doe', '123-456-7890'),
        // ('jane_smith', 2, 'jane.smith@example.com', 'password2', 'Jane Smith', 'https://example.com/avatar2.jpg', 'Hi, I am Jane Smith', '987-654-3210'),
        let login = Login {
            user_id: 2,
            username: "jane_smith".to_string(),
            password: "password3".to_string(),
        };

        // 将 mock 函数替换到 Login 实例中
        let result = login.is_valid();
        assert_eq!(result, false, "Expected user to be invalid");
    }
}
