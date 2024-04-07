use crate::models::follow::{Follow, NewFollow};
use crate::schema::follows::dsl::*;
use crate::schema::follows::{self};
use diesel::prelude::*;

pub fn create_new_follow(
    conn: &mut PgConnection,
    follow: &NewFollow,
) -> Result<Follow, diesel::result::Error> {
    // check if exist before ccreate.
    if !check_exist_follow(conn, follow.following_user_id, follow.followed_user_id) {
        match diesel::insert_into(follows::table)
            .values(follow)
            .returning(Follow::as_returning())
            .get_result(conn)
        {
            Ok(inserted_follow) => Ok(inserted_follow),
            Err(e) => Err(e),
        }
    } else {
        Err(diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::UniqueViolation,
            Box::new(String::from("Follow relation already exists")),
        )) // 返回关注关系已经存在的错误信息
    }
}

pub fn fetch_all_follows(conn: &mut PgConnection) -> Result<Vec<Follow>, diesel::result::Error> {
    follows.order(follows::follow_id.asc()).load::<Follow>(conn)
}

pub fn fetch_following_by_id(
    conn: &mut PgConnection,
    following_id: i32,
) -> Result<Vec<Follow>, diesel::result::Error> {
    follows
        .filter(follows::following_user_id.eq(following_id))
        .load::<Follow>(conn)
}
pub fn fetch_followed_by_id(
    conn: &mut PgConnection,
    followed_id: i32,
) -> Result<Vec<Follow>, diesel::result::Error> {
    follows
        .filter(follows::followed_user_id.eq(followed_id))
        .load::<Follow>(conn)
}
pub fn delete_follow(
    conn: &mut PgConnection,
    follow: &NewFollow,
) -> Result<Follow, diesel::result::Error> {
    if check_exist_follow(conn, follow.following_user_id, follow.followed_user_id) {
        diesel::delete(
            follows.filter(
                follows::following_user_id
                    .eq(follow.following_user_id)
                    .and(follows::followed_user_id.eq(follow.followed_user_id)),
            ),
        )
        .get_result(conn)
    } else {
        Err(diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::UniqueViolation,
            Box::new(String::from("Follow relation does not exists")),
        )) // 返回关注关系已经存在的错误信息
    }
}

pub fn check_exist_follow(
    conn: &mut PgConnection,
    check_following_user_id: i32,
    check_followed_user_id: i32,
) -> bool {
    // 执行查询，查看是否存在给定的关注关系
    match follows
        .filter(
            follows::following_user_id
                .eq(check_following_user_id)
                .and(follows::followed_user_id.eq(check_followed_user_id)),
        )
        .first::<Follow>(conn)
    {
        Ok(_) => true,   // 如果查询到结果，则关注关系存在
        Err(_) => false, // 如果没有查询到结果，则关注关系不存在
    }
}

#[cfg(test)]
mod test {
    use crate::{
        mappers::follow_mapper::{delete_follow, fetch_following_by_id},
        models::follow::NewFollow,
    };

    #[test]
    fn test_create_new_follow() {
        use super::create_new_follow;
        use crate::{establish_pg_connection, models::follow::NewFollow};
        let follow = NewFollow::new(1, 3);
        match establish_pg_connection() {
            Ok(mut conn) => match create_new_follow(&mut conn, &follow) {
                Ok(inserted_follow) => println!("{inserted_follow:?}"),
                Err(e) => {
                    println!("{e:?}");
                    ()
                }
            },
            Err(_) => (),
        }
    }

    #[test]
    fn test_check_exist_follow() {
        use super::check_exist_follow;
        use crate::establish_pg_connection;
        let following_id = 1;
        let followed_id = 3;
        match establish_pg_connection() {
            Ok(mut conn) => {
                let result = check_exist_follow(&mut conn, following_id, followed_id);
                println!("following: {following_id} -> followed: {followed_id}: {result}");
            }
            Err(_) => (),
        }
    }

    #[test]
    fn test_delete_follow() {
        use crate::establish_pg_connection;
        let follow = NewFollow::new(3, 2);
        match establish_pg_connection() {
            Ok(mut conn) => {
                let deleted_follow = delete_follow(&mut conn, &follow);
                println!("{deleted_follow:?}");
            }
            Err(_) => (),
        }
    }

    #[test]
    fn test_fetch_following_by_id() {
        use crate::establish_pg_connection;
        match establish_pg_connection() {
            Ok(mut conn) => {
                let follows = fetch_following_by_id(&mut conn, 2);
                println!("{follows:?}");
            }
            Err(_) => (),
        }
    }
}
