use crab_rocket_utils::time::get_e8_time;
use obj_traits::{
    mapper::mapper_crud::MapperCRUD,
    request::{
        pagination_request_param::{Pagination, PaginationParam},
        request_param::RequestParam,
    },
    response::data::Data,
};

use crate::models::{
    category::{Category, PatchCategory, PostCategory},
    category_filter::CategoryFilter,
};
use crab_rocket_schema::schema::category_table::dsl;
use diesel::prelude::*;

pub struct CategoryMapper {}

impl MapperCRUD for CategoryMapper {
    type Item = Category;
    type PostItem = PostCategory;
    type PatchItem = PatchCategory;
    type Param = RequestParam<PaginationParam, CategoryFilter>;
    fn get_all(
        conn: &mut diesel::PgConnection,
        param: &Self::Param,
    ) -> Result<obj_traits::response::data::Data<Vec<Self::Item>>, diesel::result::Error> {
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
        let total_count = dsl::category_table.count().get_result::<i64>(conn)? as i32;
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
        // need to add macro QueryableByName to struct.
        // let custom: Vec<Category> =
        //     diesel::sql_query("SELECT * FROM category_table").load::<Category>(conn)?;

        // 分页查询
        let data = dsl::category_table
            .order(dsl::updated_at.desc())
            .limit(per_page as i64)
            .offset(((page - 1) * per_page) as i64)
            .load::<Category>(conn)?;
        let resp_body = Data::new(data, pagination);
        Ok(resp_body)
    }
    fn get_by_id(conn: &mut PgConnection, pid: i32) -> Result<Category, diesel::result::Error> {
        // 配合 use crate::schema::category_table::dsl::*;
        dsl::category_table.filter(dsl::category_id.eq(pid)).first(conn)
    }

    fn add_single(
        conn: &mut PgConnection,
        obj: &PostCategory,
    ) -> Result<Category, diesel::result::Error> {
        match diesel::insert_into(dsl::category_table)
            .values(obj)
            .returning(Category::as_returning())
            .get_result(conn)
        {
            Ok(inserted_category) => Ok(inserted_category),
            Err(e) => Err(e),
        }
    }

    fn delete_by_id(conn: &mut PgConnection, pid: i32) -> Result<Category, diesel::result::Error> {
        diesel::delete(dsl::category_table.filter(dsl::category_id.eq(pid))).get_result(conn)
    }

    fn update_by_id(
        conn: &mut PgConnection,
        pid: i32,
        obj: &PatchCategory,
    ) -> Result<Category, diesel::result::Error> {
        diesel::update(dsl::category_table.filter(dsl::category_id.eq(pid)))
            .set((
                dsl::name.eq(&obj.name),
                dsl::description.eq(&obj.description),
                dsl::parent_id.eq(obj.parent_id),
                dsl::created_at.eq(obj.created_at),
                dsl::updated_at.eq(get_e8_time()),
            ))
            .get_result(conn)
    }
    fn filter(
        conn: &mut PgConnection,
        param: &RequestParam<PaginationParam, CategoryFilter>,
    ) -> Result<Data<Vec<Category>>, diesel::result::Error> {
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
        let total_count = dsl::category_table.count().get_result::<i64>(conn)? as i32;
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
        let mut query = dsl::category_table.into_boxed();
        let filter = &param.filter;
        if let Some(f) = filter {
            if let Some(category_id) = f.category_id {
                query = query.filter(dsl::category_id.eq(category_id));
            }
            if let Some(name) = &f.name {
                query = query.filter(dsl::name.like(format!("%{}%", name)));
            }
            if let Some(description) = &f.description {
                query = query.filter(dsl::description.like(format!("%{}%", description)));
            }
            if let Some(parent_id) = &f.parent_id {
                query = query.filter(dsl::parent_id.eq(parent_id));
            }
            if let Some(created_at_min) = f.created_at_min {
                query = query.filter(dsl::created_at.ge(created_at_min));
            }
            if let Some(created_at_max) = f.created_at_max {
                query = query.filter(dsl::created_at.le(created_at_max));
            }
            if let Some(updated_at_min) = f.updated_at_min {
                query = query.filter(dsl::updated_at.ge(updated_at_min));
            }
            if let Some(updated_at_max) = f.updated_at_max {
                query = query.filter(dsl::updated_at.le(updated_at_max));
            }
        }
        let data = query.load::<Category>(conn)?;
        let body = Data::new(data, pagination);
        Ok(body)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crab_rocket_schema::establish_pg_connection;
    use obj_traits::{mapper::mapper_crud::MapperCRUD, request::request_param::RequestParam};

    #[test]
    fn test_fetch_all_category_table() {
        let param = RequestParam::default(); // 預設的請求參數
        match establish_pg_connection() {
            Ok(mut conn) => match CategoryMapper::get_all(&mut conn, &param) {
                Ok(data) => {
                    println!("{:#?}", data);
                }
                Err(e) => {
                    eprintln!("Error fetching categories: {:?}", e);
                }
            },
            Err(e) => {
                eprintln!("Error establishing connection: {:?}", e);
            }
        }
    }

    #[test]
    fn test_fetch_category_by_id() {
        let test_id = 1; // 測試用的 ID
        match establish_pg_connection() {
            Ok(mut conn) => match CategoryMapper::get_by_id(&mut conn, test_id) {
                Ok(category) => {
                    println!("{:#?}", category);
                }
                Err(e) => {
                    eprintln!("Error fetching category by ID: {:?}", e);
                }
            },
            Err(e) => {
                eprintln!("Error establishing connection: {:?}", e);
            }
        }
    }

    #[test]
    fn test_add_category() {
        let new_category = PostCategory {
            name: "Test Category".to_string(),
            description: Some("A description for the test category".to_string()),
            parent_id: None,
            created_at: None,
            updated_at: None,
        };
        match establish_pg_connection() {
            Ok(mut conn) => match CategoryMapper::add_single(&mut conn, &new_category) {
                Ok(category) => {
                    println!("Added category: {:#?}", category);
                }
                Err(e) => {
                    eprintln!("Error adding category: {:?}", e);
                }
            },
            Err(e) => {
                eprintln!("Error establishing connection: {:?}", e);
            }
        }
    }

    #[test]
    fn test_update_category() {
        let category_id = 1; // 測試用的 ID
        let updated_category = PatchCategory {
            name: "Updated Category".to_string(),
            description: Some("Updated description".to_string()),
            parent_id: None,
            created_at: None,
            updated_at: None,
        };
        match establish_pg_connection() {
            Ok(mut conn) => {
                match CategoryMapper::update_by_id(&mut conn, category_id, &updated_category) {
                    Ok(category) => {
                        println!("Updated category: {:#?}", category);
                    }
                    Err(e) => {
                        eprintln!("Error updating category: {:?}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Error establishing connection: {:?}", e);
            }
        }
    }

    #[test]
    fn test_delete_category() {
        let category_id = 1; // 測試用的 ID
        match establish_pg_connection() {
            Ok(mut conn) => match CategoryMapper::delete_by_id(&mut conn, category_id) {
                Ok(category) => {
                    println!("Deleted category: {:#?}", category);
                }
                Err(e) => {
                    eprintln!("Error deleting category: {:?}", e);
                }
            },
            Err(e) => {
                eprintln!("Error establishing connection: {:?}", e);
            }
        }
    }

    #[test]
    fn test_filter_categories() {
        let filter_params = CategoryFilter {
            category_id: None,
            name: Some("Test".to_string()), // 根據名稱過濾
            description: None,
            parent_id: None,
            created_at_min: None,
            created_at_max: None,
            updated_at_min: None,
            updated_at_max: None,
        };
        let param = RequestParam {
            pagination: PaginationParam {
                limit: Some(10),
                offset: Some(0),
            },
            filter: Some(filter_params),
        };

        match establish_pg_connection() {
            Ok(mut conn) => match CategoryMapper::filter(&mut conn, &param) {
                Ok(data) => {
                    println!("{:#?}", data);
                }
                Err(e) => {
                    eprintln!("Error filtering categories: {:?}", e);
                }
            },
            Err(e) => {
                eprintln!("Error establishing connection: {:?}", e);
            }
        }
    }
}
