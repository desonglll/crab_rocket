use chrono::NaiveDateTime;
use crab_rocket_schema::schema::session_table::dsl;
use crab_rocket_schema::DbPool;
use crab_rocket_schema::{establish_pg_connection, schema::session_table};
use crab_rocket_utils::time::get_e8_time;
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use rocket::State;
use uuid::Uuid; // 添加这一行

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
    pub fn new(user_id: i32) -> Self {
        Self {
            user_id,
            session_id: Uuid::new_v4(),
            expires: get_e8_time(),
            created_at: get_e8_time(),
        }
    }
    pub fn is_expires(&self) -> bool {
        let now = get_e8_time();
        self.expires < now
    }

    pub fn add_session(
        pool: &State<DbPool>,
        session: Session,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        match establish_pg_connection(pool) {
            Ok(mut conn) => {
                let result = diesel::insert_into(dsl::session_table)
                    .values((
                        dsl::user_id.eq(session.user_id),
                        dsl::session_id.eq(session.session_id),
                        dsl::expires.eq(session.expires),
                        dsl::created_at.eq(session.created_at),
                    ))
                    .on_conflict(dsl::user_id)
                    .do_update()
                    .set((
                        dsl::session_id.eq(session.session_id),
                        dsl::expires.eq(session.expires),
                        dsl::created_at.eq(session.created_at),
                    ))
                    .returning(Session::as_returning())
                    .get_result(&mut conn);
                match result {
                    Ok(data) => Ok(data),
                    Err(e) => Err(Box::new(e)),
                }
            }
            Err(e) => Err(Box::new(e)),
        }
    }

    pub fn remove_session(pool: &State<DbPool>, session_id: Uuid) -> bool {
        match establish_pg_connection(pool) {
            Ok(mut conn) => {
                let result = diesel::delete(
                    dsl::session_table.filter(session_table::session_id.eq(session_id)),
                )
                .returning((dsl::user_id, dsl::session_id, dsl::expires, dsl::created_at))
                .get_result::<Session>(&mut conn);
                match result {
                    Ok(_) => true,
                    Err(e) => {
                        println!("{:#?}", e);
                        false
                    }
                }
            }
            Err(err) => {
                println!("{:#?}", err);
                false
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use crab_rocket_schema::{establish_pool, DbPool};
    use rocket::State;

    use super::*;

    #[test]
    fn test_create_session() {
        let session = Session::new(1);
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
        let session = Session::new(3);
        let added_session = Session::add_session(pool, session.clone()).unwrap();

        assert_eq!(added_session.user_id, 3);
        assert_eq!(added_session.session_id, session.session_id);
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
        let session = Session::new(3);
        let added_session = Session::add_session(pool, session.clone()).unwrap();

        assert_eq!(added_session.user_id, 3);

        // Now remove the session
        let result = Session::remove_session(pool, session.session_id);
        println!("Result: {:?}", result);
        // assert!(result, "Expected session to be removed");

        // Verify that the session has been removed
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        let mut conn = establish_pg_connection(pool).expect("Failed to establish connection");
        let query_result = dsl::session_table
            .filter(dsl::session_id.eq(added_session.session_id))
            .select(Session::as_select()) // Ensure to select the fields properly
            .first::<Session>(&mut conn);

        assert!(query_result.is_err(), "Expected session to be removed from database");
    }
}
