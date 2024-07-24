use obj_traits::{
    mapper::mapper_crud::MapperCRUD,
    request::{pagination_request_param::Pagination, request_param::RequestParam},
    response::data::Data,
};
use rocket::State;

use crate::models::{
    follow::{Follow, PatchFollow, PostFollow},
    follow_filter::FollowFilter,
};
use crab_rocket_schema::{establish_pg_connection, schema::follow_table::dsl, DbPool};
use diesel::prelude::*;

use super::follow_mapper_trait::FollowMapperTrait;
pub struct FollowMapper {}
impl MapperCRUD for FollowMapper {
    type Item = Follow;
    type PostItem = PostFollow;
    type PatchItem = PatchFollow;
    type Filter = FollowFilter;
    fn get_all(
        pool: &State<DbPool>,
        param: &RequestParam<Self::Item, Self::Filter>,
    ) -> Result<Data<Vec<Follow>>, diesel::result::Error> {
        let mut conn = establish_pg_connection(pool).expect("msg");
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
        let pagination = param.pagination.unwrap_or_default().clone();

        let page = (pagination.offset.unwrap() / pagination.limit.unwrap()) + 1;
        let per_page = pagination.limit.unwrap();
        // 获取总记录数
        let total_count = dsl::follow_table.count().get_result::<i64>(&mut conn)? as i32;
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
        let data = dsl::follow_table
            .order(dsl::created_at.desc())
            .limit(per_page as i64)
            .offset(((page - 1) * per_page) as i64)
            .load::<Follow>(&mut conn)?;
        let body = Data::new(data, Some(pagination));
        println!("{body}");
        Ok(body)
    }

    fn get_by_id(pool: &State<DbPool>, pid: i32) -> Result<Data<Follow>, diesel::result::Error> {
        let mut conn = establish_pg_connection(pool).expect("msg");
        let data = dsl::follow_table.filter(dsl::follow_id.eq(pid)).first(&mut conn)?;
        Ok(Data::new(data, None))
    }

    fn add_single(
        pool: &State<DbPool>,
        obj: &PostFollow,
    ) -> Result<Data<Follow>, diesel::result::Error> {
        let mut conn = establish_pg_connection(pool).expect("msg");
        // check if exist before ccreate.
        if !check_exist_follow(pool, obj.following_user_id, obj.followed_user_id) {
            let data = diesel::insert_into(dsl::follow_table)
                .values(obj)
                .returning(Follow::as_returning())
                .get_result(&mut conn)?;
            Ok(Data::new(data, None))
        } else {
            Err(diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UniqueViolation,
                Box::new(String::from("Follow relation already exists")),
            )) // 返回关注关系已经存在的错误信息
        }
    }

    fn delete_by_id(pool: &State<DbPool>, pid: i32) -> Result<Data<Follow>, diesel::result::Error> {
        let mut conn = establish_pg_connection(pool).expect("msg");
        let data = diesel::delete(dsl::follow_table.filter(dsl::follow_id.eq(pid)))
            .get_result(&mut conn)?;
        Ok(Data::new(data, None))
    }

    fn update_by_id(
        pool: &State<DbPool>,
        pid: i32,
        obj: &PatchFollow,
    ) -> Result<Data<Follow>, diesel::result::Error> {
        let mut conn = establish_pg_connection(pool).expect("msg");
        let data = diesel::update(dsl::follow_table.filter(dsl::follow_id.eq(pid)))
            .set((
                dsl::followed_user_id.eq(obj.followed_user_id),
                dsl::following_user_id.eq(obj.following_user_id),
            ))
            .get_result(&mut conn)?;
        Ok(Data::new(data, None))
    }

    fn filter(
        pool: &State<DbPool>,
        param: &RequestParam<Self::Item, Self::Filter>,
    ) -> Result<Data<Vec<Follow>>, diesel::result::Error> {
        let mut conn = establish_pg_connection(pool).expect("msg");
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
        let filter = &param.filter;
        println!("{filter:?}");
        // 计算分页相关
        let pagination = param.pagination.unwrap_or_default().clone();

        let page = (pagination.offset.unwrap() / pagination.limit.unwrap()) + 1;
        let per_page = pagination.limit.unwrap();
        // 获取总记录数
        let total_count = dsl::follow_table.count().get_result::<i64>(&mut conn)? as i32;
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

        let mut query = dsl::follow_table.into_boxed();

        // 分页查询
        query = query
            .order(dsl::created_at.desc())
            .limit(per_page as i64)
            .offset(((page - 1) * per_page) as i64);

        if let Some(f) = filter {
            if let Some(created_at_max) = &f.created_at_max {
                query = query.filter(dsl::created_at.le(created_at_max));
            }
            if let Some(created_at_min) = &f.created_at_min {
                query = query.filter(dsl::created_at.ge(created_at_min));
            }
            if let Some(followed_user_id) = &f.followed_user_id {
                query = query.filter(dsl::followed_user_id.eq(followed_user_id));
            }
            if let Some(following_user_id) = &f.following_user_id {
                query = query.filter(dsl::following_user_id.eq(following_user_id));
            }
            if let Some(follow_id) = &f.follow_id {
                query = query.filter(dsl::follow_id.eq(follow_id));
            }
        }
        let data = query.load::<Follow>(&mut conn)?;
        let body = Data::new(data, Some(pagination));
        Ok(body)
    }
}

impl FollowMapperTrait for FollowMapper {
    fn delete_follow_specifically(
        pool: &State<DbPool>,
        obj: &PostFollow,
    ) -> Result<Data<Follow>, diesel::result::Error> {
        let mut conn = establish_pg_connection(pool).expect("msg");
        if check_exist_follow(pool, obj.following_user_id, obj.followed_user_id) {
            let data = diesel::delete(
                dsl::follow_table.filter(
                    dsl::following_user_id
                        .eq(obj.following_user_id)
                        .and(dsl::followed_user_id.eq(obj.followed_user_id)),
                ),
            )
            .get_result(&mut conn)?;
            Ok(Data::new(data, None))
        } else {
            Err(diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UniqueViolation,
                Box::new(String::from("Follow relation does not exists")),
            )) // 返回关注关系已经存在的错误信息
        }
    }

    fn get_followings_by_user_id(
        pool: &State<DbPool>,
        uid: i32,
        param: &RequestParam<Follow, FollowFilter>,
    ) -> Result<Data<Vec<Follow>>, diesel::result::Error> {
        let mut conn = establish_pg_connection(pool).expect("msg");
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
        let pagination = param.pagination.unwrap_or_default().clone();

        let page = (pagination.offset.unwrap() / pagination.limit.unwrap()) + 1;
        let per_page = pagination.limit.unwrap();
        // 获取总记录数
        let total_count = dsl::follow_table.count().get_result::<i64>(&mut conn)? as i32;
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
        let data = dsl::follow_table
            .filter(dsl::followed_user_id.eq(uid))
            .order(dsl::created_at.desc())
            .limit(per_page as i64)
            .offset(((page - 1) * per_page) as i64)
            .load::<Follow>(&mut conn)?;
        let body = Data::new(data, Some(pagination));
        println!("{body}");
        Ok(body)
    }

    fn get_followeds_by_user_id(
        pool: &State<DbPool>,
        uid: i32,
        param: &RequestParam<Follow, FollowFilter>,
    ) -> Result<Data<Vec<Follow>>, diesel::result::Error> {
        let mut conn = establish_pg_connection(pool).expect("msg");
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
        let pagination = param.pagination.unwrap_or_default().clone();

        let page = (pagination.offset.unwrap() / pagination.limit.unwrap()) + 1;
        let per_page = pagination.limit.unwrap();
        // 获取总记录数
        let total_count = dsl::follow_table.count().get_result::<i64>(&mut conn)? as i32;
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
        let data = dsl::follow_table
            .filter(dsl::following_user_id.eq(uid))
            .order(dsl::created_at.desc())
            .limit(per_page as i64)
            .offset(((page - 1) * per_page) as i64)
            .load::<Follow>(&mut conn)?;
        let body = Data::new(data, Some(pagination));
        println!("{body}");
        Ok(body)
    }
}

pub fn check_exist_follow(
    pool: &State<DbPool>,
    check_following_user_id: i32,
    check_followed_user_id: i32,
) -> bool {
    let mut conn = establish_pg_connection(pool).expect("msg");
    // 执行查询，查看是否存在给定的关注关系
    match dsl::follow_table
        .filter(
            dsl::following_user_id
                .eq(check_following_user_id)
                .and(dsl::followed_user_id.eq(check_followed_user_id)),
        )
        .first::<Follow>(&mut conn)
    {
        Ok(_) => true,   // 如果查询到结果，则关注关系存在
        Err(_) => false, // 如果没有查询到结果，则关注关系不存在
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::models::follow::{PatchFollow, PostFollow};
    use crab_rocket_schema::{establish_pool, DbPool};
    use rocket::State;

    #[test]
    fn test_create_new_follow() {
        let follow = PostFollow::new(1, 3);
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);

        let result = FollowMapper::add_single(pool, &follow);
        match result {
            Ok(inserted_follow) => {
                println!("Created follow: {:?}", inserted_follow);
                assert_eq!(inserted_follow.data.following_user_id, follow.following_user_id);
                assert_eq!(inserted_follow.data.followed_user_id, follow.followed_user_id);
            }
            Err(e) => {
                println!("Error creating follow: {:?}", e);
                assert!(false);
            }
        }
    }

    #[test]
    fn test_check_exist_follow() {
        let following_id = 1;
        let followed_id = 5;
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        let result = check_exist_follow(pool, following_id, followed_id);
        println!("Follow exists: {}", result);
        assert!(result);
    }

    #[test]
    fn test_delete_follow() {
        let follow_id = 1;
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);

        let result = FollowMapper::delete_by_id(pool, follow_id);
        match result {
            Ok(deleted_follow) => {
                println!("Deleted follow: {:?}", deleted_follow);
                assert_eq!(deleted_follow.data.follow_id, follow_id);
            }
            Err(e) => {
                println!("Error deleting follow: {:?}", e);
                assert!(false);
            }
        }
    }

    #[test]
    fn test_delete_follow_specifically() {
        let follow = PostFollow::demo();
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);

        let result = FollowMapper::delete_follow_specifically(pool, &follow);
        match result {
            Ok(deleted_follow) => {
                println!("Deleted specific follow: {:?}", deleted_follow);
                assert_eq!(deleted_follow.data.following_user_id, follow.following_user_id);
                assert_eq!(deleted_follow.data.followed_user_id, follow.followed_user_id);
            }
            Err(e) => {
                println!("Error deleting specific follow: {:?}", e);
                assert!(false);
            }
        }
    }

    #[test]
    fn test_update_follow() {
        let follow_id = 2;
        let updated_follow = PatchFollow::new(2, 4, None);
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);

        let result = FollowMapper::update_by_id(pool, follow_id, &updated_follow);
        match result {
            Ok(updated_follow) => {
                println!("Updated follow: {:?}", updated_follow);
                assert_eq!(
                    updated_follow.data.following_user_id,
                    updated_follow.data.following_user_id
                );
                assert_eq!(
                    updated_follow.data.followed_user_id,
                    updated_follow.data.followed_user_id
                );
            }
            Err(e) => {
                println!("Error updating follow: {:?}", e);
                assert!(false);
            }
        }
    }

    #[test]
    fn test_get_all_follows() {
        let param = RequestParam::demo();
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);

        let result = FollowMapper::get_all(pool, &param);
        match result {
            Ok(data) => {
                println!("Fetched all follows: {:?}", data);
                assert!(data.data.len() > 0);
            }
            Err(e) => {
                println!("Error fetching all follows: {:?}", e);
                assert!(false);
            }
        }
    }

    #[test]
    fn test_get_followings_by_user_id() {
        let user_id = 1;
        let param = RequestParam::demo();
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);

        let result = FollowMapper::get_followings_by_user_id(pool, user_id, &param);
        match result {
            Ok(data) => {
                println!("Fetched followings by user ID: {:?}", data);
                assert!(data.data.len() > 0);
            }
            Err(e) => {
                println!("Error fetching followings by user ID: {:?}", e);
                assert!(false);
            }
        }
    }

    #[test]
    fn test_get_followeds_by_user_id() {
        let user_id = 1;
        let param = RequestParam::demo();
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);

        let result = FollowMapper::get_followeds_by_user_id(pool, user_id, &param);
        match result {
            Ok(data) => {
                println!("Fetched followeds by user ID: {:?}", data);
                assert!(data.data.len() > 0);
            }
            Err(e) => {
                println!("Error fetching followeds by user ID: {:?}", e);
                assert!(false);
            }
        }
    }
}
