use obj_traits::{
    mapper::mapper_crud::MapperCRUD,
    request::{
        pagination_request_param::{Pagination, PaginationParam},
        request_param::RequestParam,
    },
    response::data::Data,
};

use crate::models::follow::{Follow, NewFollow, PatchFollow};
use crab_rocket_schema::schema::follows::dsl;
use diesel::prelude::*;

use super::follow_mapper_trait::FollowMapperTrait;
pub struct FollowMapper {}
impl MapperCRUD<Follow, NewFollow, PatchFollow, RequestParam<PaginationParam>> for FollowMapper {
    fn get_all(
        conn: &mut diesel::PgConnection,
        param: &RequestParam<PaginationParam>,
    ) -> Result<obj_traits::response::data::Data<Vec<Follow>>, diesel::result::Error> {
        // 当前页码（page）
        // 每页条目数（per_page）
        //
        // 总页数（total_pages）
        //
        // 公式
        //
        // 当前页的 offset: (page - 1) * per_page
        //
        // 下一页的 offset: page * per_page
        //
        // 上一页的 offset: (page - 2) * per_page （如果 page > 1）
        //
        // limit 始终为 per_page
        // 计算分页相关
        let page = (param.pagination.offset.unwrap() / param.pagination.limit.unwrap()) + 1;
        let per_page = param.pagination.limit.unwrap();
        // 获取总记录数
        let total_count = dsl::follows.count().get_result::<i64>(conn)? as i32;
        // 计算总页数
        let total_pages = (total_count + per_page - 1) / per_page;

        let previous_page_offset = (page - 2) * per_page;
        let next_page_offset = page * per_page;
        let pagination = Pagination::new(
            page,
            per_page,
            total_pages,
            total_count,
            Some(format!("?limit={}&offset={}", per_page, next_page_offset)),
            Some(format!("?limit={}&offset={}", per_page, previous_page_offset)),
        );

        // 分页查询
        let data = dsl::follows
            .order(dsl::created_at.desc())
            .limit(per_page as i64)
            .offset(((page - 1) * per_page) as i64)
            .load::<Follow>(conn)?;
        let body = Data::new(data, pagination);
        println!("{body}");
        Ok(body)
    }

    fn get_by_id(conn: &mut PgConnection, pid: i32) -> Result<Follow, diesel::result::Error> {
        dsl::follows.filter(dsl::follow_id.eq(pid)).first(conn)
    }

    fn add_single(
        conn: &mut PgConnection,
        obj: &NewFollow,
    ) -> Result<Follow, diesel::result::Error> {
        // check if exist before ccreate.
        if !check_exist_follow(conn, obj.following_user_id(), obj.followed_user_id()) {
            match diesel::insert_into(dsl::follows)
                .values(obj)
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

    fn delete_by_id(conn: &mut PgConnection, pid: i32) -> Result<Follow, diesel::result::Error> {
        diesel::delete(dsl::follows.filter(dsl::follow_id.eq(pid))).get_result(conn)
    }

    fn update_by_id(
        conn: &mut PgConnection,
        pid: i32,
        obj: &PatchFollow,
    ) -> Result<Follow, diesel::result::Error> {
        diesel::update(dsl::follows.filter(dsl::follow_id.eq(pid)))
            .set((
                dsl::followed_user_id.eq(obj.followed_user_id()),
                dsl::following_user_id.eq(obj.following_user_id()),
            ))
            .get_result(conn)
    }
}

impl FollowMapperTrait<RequestParam<PaginationParam>> for FollowMapper {
    fn delete_follow_specifically(
        conn: &mut PgConnection,
        obj: &NewFollow,
    ) -> Result<Follow, diesel::result::Error> {
        if check_exist_follow(conn, obj.following_user_id(), obj.followed_user_id()) {
            diesel::delete(
                dsl::follows.filter(
                    dsl::following_user_id
                        .eq(obj.following_user_id())
                        .and(dsl::followed_user_id.eq(obj.followed_user_id())),
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

    fn get_followings_by_user_id(
        conn: &mut PgConnection,
        uid: i32,
        param: &RequestParam<PaginationParam>,
    ) -> Result<Data<Vec<Follow>>, diesel::result::Error> {
        // 当前页码（page）
        // 每页条目数（per_page）
        //
        // 总页数（total_pages）
        //
        // 公式
        //
        // 当前页的 offset: (page - 1) * per_page
        //
        // 下一页的 offset: page * per_page
        //
        // 上一页的 offset: (page - 2) * per_page （如果 page > 1）
        //
        // limit 始终为 per_page
        // 计算分页相关
        let page = (param.pagination.offset.unwrap() / param.pagination.limit.unwrap()) + 1;
        let per_page = param.pagination.limit.unwrap();
        // 获取总记录数
        let total_count = dsl::follows.count().get_result::<i64>(conn)? as i32;
        // 计算总页数
        let total_pages = (total_count + per_page - 1) / per_page;

        let previous_page_offset = (page - 2) * per_page;
        let next_page_offset = page * per_page;
        let pagination = Pagination::new(
            page,
            per_page,
            total_pages,
            total_count,
            Some(format!("?limit={}&offset={}", per_page, next_page_offset)),
            Some(format!("?limit={}&offset={}", per_page, previous_page_offset)),
        );

        // 分页查询
        let data = dsl::follows
            .filter(dsl::followed_user_id.eq(uid))
            .order(dsl::created_at.desc())
            .limit(per_page as i64)
            .offset(((page - 1) * per_page) as i64)
            .load::<Follow>(conn)?;
        let body = Data::new(data, pagination);
        println!("{body}");
        Ok(body)
    }

    fn get_followeds_by_user_id(
        conn: &mut PgConnection,
        uid: i32,
        param: &RequestParam<PaginationParam>,
    ) -> Result<Data<Vec<Follow>>, diesel::result::Error> {
        // 当前页码（page）
        // 每页条目数（per_page）
        //
        // 总页数（total_pages）
        //
        // 公式
        //
        // 当前页的 offset: (page - 1) * per_page
        //
        // 下一页的 offset: page * per_page
        //
        // 上一页的 offset: (page - 2) * per_page （如果 page > 1）
        //
        // limit 始终为 per_page
        // 计算分页相关
        let page = (param.pagination.offset.unwrap() / param.pagination.limit.unwrap()) + 1;
        let per_page = param.pagination.limit.unwrap();
        // 获取总记录数
        let total_count = dsl::follows.count().get_result::<i64>(conn)? as i32;
        // 计算总页数
        let total_pages = (total_count + per_page - 1) / per_page;

        let previous_page_offset = (page - 2) * per_page;
        let next_page_offset = page * per_page;
        let pagination = Pagination::new(
            page,
            per_page,
            total_pages,
            total_count,
            Some(format!("?limit={}&offset={}", per_page, next_page_offset)),
            Some(format!("?limit={}&offset={}", per_page, previous_page_offset)),
        );

        // 分页查询
        let data = dsl::follows
            .filter(dsl::following_user_id.eq(uid))
            .order(dsl::created_at.desc())
            .limit(per_page as i64)
            .offset(((page - 1) * per_page) as i64)
            .load::<Follow>(conn)?;
        let body = Data::new(data, pagination);
        println!("{body}");
        Ok(body)
    }
}

pub fn check_exist_follow(
    conn: &mut PgConnection,
    check_following_user_id: i32,
    check_followed_user_id: i32,
) -> bool {
    // 执行查询，查看是否存在给定的关注关系
    match dsl::follows
        .filter(
            dsl::following_user_id
                .eq(check_following_user_id)
                .and(dsl::followed_user_id.eq(check_followed_user_id)),
        )
        .first::<Follow>(conn)
    {
        Ok(_) => true,   // 如果查询到结果，则关注关系存在
        Err(_) => false, // 如果没有查询到结果，则关注关系不存在
    }
}

#[cfg(test)]
mod test {
    use obj_traits::mapper::mapper_crud::MapperCRUD;

    use crate::{
        mappers::{follow_mapper::FollowMapper, follow_mapper_trait::FollowMapperTrait},
        models::follow::NewFollow,
    };

    #[test]
    fn test_create_new_follow() {
        use crate::models::follow::NewFollow;
        use crab_rocket_schema::establish_pg_connection;
        let follow = NewFollow::new(1, 3, None);
        match establish_pg_connection() {
            Ok(mut conn) => match FollowMapper::add_single(&mut conn, &follow) {
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
        use crab_rocket_schema::establish_pg_connection;
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
        use crab_rocket_schema::establish_pg_connection;
        match establish_pg_connection() {
            Ok(mut conn) => {
                let deleted_follow = FollowMapper::delete_by_id(&mut conn, 1);
                println!("{deleted_follow:?}");
            }
            Err(_) => (),
        }
    }

    #[test]
    fn test_delete_follow_specifically() {
        use crab_rocket_schema::establish_pg_connection;
        let follow = NewFollow::demo();
        match establish_pg_connection() {
            Ok(mut conn) => {
                let deleted_follow = FollowMapper::delete_follow_specifically(&mut conn, &follow);
                println!("{deleted_follow:?}");
            }
            Err(_) => (),
        }
    }
}
