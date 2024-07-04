use crate::models::user::{NewUser, PatchUser, User};
use crab_rocket_schema::schema::user_table::dsl;
use crab_rocket_schema::schema::user_table::{self};
use diesel::prelude::*;
use diesel::result::Error;
use crab_rocket_utils::time::get_e8_time;
use obj_traits::mapper::mapper_crud::MapperCRUD;
use obj_traits::request::pagination_request_param::{Pagination, PaginationParam};
use obj_traits::request::request_param::RequestParam;
use obj_traits::response::data::Data;

pub struct UserMapper {}

impl MapperCRUD<User, NewUser, PatchUser, RequestParam<PaginationParam>> for UserMapper {
    fn get_all(conn: &mut PgConnection, param: &RequestParam<PaginationParam>) -> Result<Data<Vec<User>>, Error> {
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
        let total_count = dsl::user_table.count().get_result::<i64>(conn)? as i32;
        // 计算总页数
        let total_pages = (total_count + per_page - 1) / per_page;

        let previous_page_offset = (page - 2) * per_page;
        let next_page_offset = page * per_page;
        let pagination =
            Pagination::new(page, per_page, total_pages, total_count, Some(format!("?limit={}&offset={}", per_page, next_page_offset)), Some(format!("?limit={}&offset={}", per_page, previous_page_offset)));

        // 分页查询
        let data = dsl::user_table
            .order(dsl::updated_at.desc())
            .limit(per_page as i64)
            .offset(((page - 1) * per_page) as i64)
            .load::<User>(conn)?;
        let body = Data::new(data, pagination);
        Ok(body)
    }
    fn get_by_id(conn: &mut PgConnection, pid: i32) -> Result<User, Error> {
        dsl::user_table.filter(dsl::user_id.eq(pid)).first(conn)
    }
    fn add_single(conn: &mut PgConnection, obj: &NewUser) -> Result<User, Error> {
        diesel::insert_into(dsl::user_table)
            .values(obj)
            .returning(User::as_returning())
            .get_result(conn)
    }
    fn delete_by_id(conn: &mut PgConnection, pid: i32) -> Result<User, Error> {
        diesel::delete(dsl::user_table.filter(dsl::user_id.eq(pid))).get_result(conn)
    }
    fn update_by_id(conn: &mut PgConnection, pid: i32, obj: &PatchUser) -> Result<User, Error> {
        diesel::update(dsl::user_table.filter(dsl::user_id.eq(pid)))
            .set((
                user_table::username.eq(obj.username()),
                user_table::password.eq(obj.password()),
                user_table::role_id.eq(obj.role_id()),
                user_table::email.eq(obj.email()),
                user_table::full_name.eq(obj.full_name()),
                user_table::avatar_url.eq(obj.avatar_url()),
                user_table::bio.eq(obj.bio()),
                user_table::updated_at.eq(Some(get_e8_time())),
                user_table::mobile_phone.eq(obj.mobile_phone()),
                user_table::created_at.eq(obj.created_at()),
            ))
            .get_result(conn)
    }
}

#[cfg(test)]
mod test {
    use obj_traits::mapper::mapper_crud::MapperCRUD;
    use obj_traits::request::pagination_request_param::PaginationParam;
    use obj_traits::request::request_param::RequestParam;
    use crate::mappers::user_mapper::UserMapper;
    use crate::models::user::{NewUser, PatchUser};

    #[test]
    fn test_insert_user() {
        use crab_rocket_schema::establish_pg_connection;

        let user = NewUser::demo();
        println!("{user:?}");
        match establish_pg_connection() {
            Ok(mut conn) => match UserMapper::add_single(&mut conn, &user) {
                Ok(result) => println!("{result}"),
                Err(e) => println!("{e:?}"),
            },
            Err(_) => println!("establish_pg_connection error"),
        }
    }

    #[test]
    fn test_fetch_all_users() {
        use crab_rocket_schema::establish_pg_connection;

        let param = RequestParam::new(PaginationParam::demo());

        match establish_pg_connection() {
            Ok(mut conn) => match UserMapper::get_all(&mut conn, &param) {
                Ok(res) => println!("{res}"),
                Err(e) => println!("{e:?}"),
            },
            Err(_) => println!("establish_pg_connection error"),
        }
    }

    #[test]
    fn test_fetch_user_by_id() {
        use crab_rocket_schema::establish_pg_connection;
        let id = 1;
        match establish_pg_connection() {
            Ok(mut conn) => match UserMapper::get_by_id(&mut conn, id) {
                Ok(res) => println!("{res}"),
                Err(e) => println!("{e:?}"),
            },
            Err(_) => println!("establish_pg_connection error"),
        }
    }

    #[test]
    fn test_update_user_by_id() {
        match crab_rocket_schema::establish_pg_connection() {
            Ok(mut conn) => {
                let id = 1;
                let user = PatchUser::default();
                match UserMapper::update_by_id(&mut conn, id, &user) {
                    Ok(res) => println!("{res}"),
                    Err(e) => println!("{e:?}"),
                }
            }
            Err(_) => println!("establish_pg_connection error"),
        }
    }

    #[test]
    fn test_delete_user_by_id() {
        match crab_rocket_schema::establish_pg_connection() {
            Ok(mut conn) => {
                let id = 1;
                match UserMapper::delete_by_id(&mut conn, id) {
                    Ok(res) => println!("{res}"),
                    Err(e) => println!("{e:?}"),
                }
            }
            Err(_) => println!("establish_pg_connection error"),
        }
    }
}
