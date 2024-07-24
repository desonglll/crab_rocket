use std::error::Error;
use std::fmt::{Display, Formatter};

use chrono::{Duration, NaiveDateTime};
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use rocket::State;
use uuid::Uuid;

use crab_rocket_schema::{establish_pg_connection, schema::session_table};
use crab_rocket_schema::DbPool;
use crab_rocket_schema::schema::session_table::dsl;
use crab_rocket_utils::time::get_e8_time;

// 添加这一行
#[derive(Debug)]
pub enum SessionError {
    NotFound,
    Expired,
}

impl Error for SessionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl Display for SessionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            SessionError::NotFound => write!(f, "Session not found"),
            SessionError::Expired => write!(f, "Session expired"),
        }
    }
}

#[derive(
    Insertable, Serialize, Deserialize, Selectable, Debug, Queryable, Default, Clone, Copy,
)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = crab_rocket_schema::schema::session_table)]
pub struct Session {
    // pub id: i32,
    pub user_id: i32,
    pub session_id: Uuid,
    pub expires: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

impl Session {
    pub fn new(user_id: i32, days: i64) -> Self {
        Self {
            user_id,
            session_id: Uuid::new_v4(),
            expires: get_e8_time() + Duration::days(days),
            created_at: get_e8_time(),
        }
    }
    pub fn is_expires(&self) -> Result<bool, SessionError> {
        let now = get_e8_time();
        if self.expires > now {
            Ok(false)
        } else {
            Err(SessionError::Expired)
        }
    }

    pub fn add_session(&self, pool: &State<DbPool>) -> Result<Self, diesel::result::Error> {
        let mut conn = establish_pg_connection(pool).expect("msg");
        let result = diesel::insert_into(dsl::session_table)
            .values((
                dsl::user_id.eq(self.user_id),
                dsl::session_id.eq(self.session_id),
                dsl::expires.eq(self.expires),
                dsl::created_at.eq(self.created_at),
            ))
            .on_conflict(dsl::user_id)
            .do_update()
            .set((
                // dsl::session_id.eq(session.session_id),
                dsl::expires.eq(self.expires + Duration::days(1)),
                dsl::created_at.eq(self.created_at),
            ))
            .returning(Session::as_returning())
            .get_result(&mut conn);
        match result {
            Ok(data) => Ok(data),
            Err(e) => Err(e),
        }
    }

    pub fn remove_session(&self, pool: &State<DbPool>) -> Result<Self, diesel::result::Error> {
        let mut conn = establish_pg_connection(pool).expect("msg");
        let result = diesel::delete(
            dsl::session_table.filter(session_table::session_id.eq(self.session_id)),
        )
            .returning((dsl::user_id, dsl::session_id, dsl::expires, dsl::created_at))
            .get_result::<Session>(&mut conn);
        match result {
            Ok(deleted_session) => Ok(deleted_session),
            Err(e) => {
                println!("function: remove_session\t=>\t{:#?}", e);
                Err(e)
            }
        }
    }
    pub fn get_session_by_uid(
        pool: &State<DbPool>,
        uid: i32,
    ) -> Result<Session, diesel::result::Error> {
        let mut conn = establish_pg_connection(pool).expect("msg");
        session_table::dsl::session_table
            .filter(session_table::dsl::user_id.eq(uid))
            .select(Session::as_select()) // Ensure to select the fields properly
            .first::<Session>(&mut conn)
    }
    pub fn get_session_by_id(
        pool: &State<DbPool>,
        session_id: Uuid,
    ) -> Result<Session, diesel::result::Error> {
        let mut conn = establish_pg_connection(pool).expect("msg");
        session_table::dsl::session_table
            .filter(session_table::dsl::session_id.eq(session_id))
            .select(Session::as_select()) // Ensure to select the fields properly
            .first::<Session>(&mut conn)
    }

    pub fn is_exists(&self, pool: &State<DbPool>) -> Result<bool, SessionError> {
        let mut conn = establish_pg_connection(pool).expect("");
        let result = session_table::dsl::session_table
            .filter(session_table::dsl::session_id.eq(self.session_id))
            .select(Session::as_select()) // Ensure to select the fields properly
            .first::<Session>(&mut conn);
        match result {
            Ok(_) => Ok(true),
            Err(e) => {
                println!("{:#?}", e);
                Err(SessionError::NotFound)
            }
        }
    }

    pub fn is_valid(&self, pool: &State<DbPool>) -> Result<bool, SessionError> {
        println!("{:#?}", self);
        match self.is_exists(pool) {
            Ok(_) => match self.is_expires() {
                Ok(_) => Ok(true),
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use rocket::State;

    use crab_rocket_schema::{DbPool, establish_pool};

    use super::*;

    #[test]
    fn test_create_session() {
        let session = Session::new(1, 1);
        println!("{:#?}", session);

        assert_eq!(session.user_id, 1);
        // 检查 UUID 是否已正确生成
        assert!(session.session_id.to_string().len() > 0);
        // 检查时间字段是否已正确生成
        assert!(session.expires.and_utc().timestamp() > 0);
        assert!(session.created_at.and_utc().timestamp() > 0);
    }

    #[test]
    fn test_add_session() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        let session = Session::new(3, 1);
        println!("{:#?}", session);
        let added_session = session.add_session(pool).unwrap();
        println!("{:#?}", added_session);

        if !added_session.is_exists(pool).unwrap() {
            assert_eq!(added_session.session_id, session.session_id);
        }
        assert_eq!(added_session.user_id, 3);
        assert!(added_session.expires.and_utc().timestamp() > 0);
        assert!(added_session.created_at.and_utc().timestamp() > 0);

        // Clean up the test data
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        let mut conn = establish_pg_connection(pool).expect("Failed to establish connection");
        diesel::delete(dsl::session_table.filter(dsl::session_id.eq(session.session_id)))
            .execute(&mut conn)
            .expect("Failed to clean up test data");
    }

    #[test]
    fn test_remove_session() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        // First, add a session
        let session = Session::new(3, 1);

        let added_session = session.add_session(pool).unwrap();
        assert_eq!(added_session.user_id, 3);

        // Now remove the session
        let result = session.remove_session(pool);
        println!("{:#?}", result);
        assert!(result.is_ok(), "Expected session to be removed");

        // Verify that the session has been removed
        let mut conn = establish_pg_connection(pool).expect("msg_connection");
        let query_result: Result<Session, diesel::result::Error> = dsl::session_table
            .filter(dsl::session_id.eq(added_session.session_id))
            .select(Session::as_select()) // Ensure to select the fields properly
            .first::<Session>(&mut conn);

        assert!(query_result.is_err(), "Expected session to be removed from database");
    }
}
