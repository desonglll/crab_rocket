use diesel::prelude::*;
use rocket::State;

use crab_rocket_schema::{DbPool, establish_pg_connection, schema::category_table::dsl};
use crab_rocket_utils::time::get_e8_time;
use obj_traits::{
    mapper::mapper_crud::MapperCRUD,
    request::{pagination_request_param::Pagination, request_param::RequestParam},
    response::data::Data,
};

use crate::models::{
    category::{Category, PatchCategory, PostCategory},
    category_filter::CategoryFilter,
};

pub struct CategoryMapper {}

impl MapperCRUD for CategoryMapper {
    type Item = Category;
    type PostItem = PostCategory;
    type PatchItem = PatchCategory;
    type Filter = CategoryFilter;
    fn get_all(
        pool: &State<DbPool>,
        param: &RequestParam<Self::Item, Self::Filter>,
    ) -> Result<Data<Vec<Category>>, diesel::result::Error> {
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
        let total_count = dsl::category_table.count().get_result::<i64>(&mut conn)? as i32;
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
            .load::<Category>(&mut conn)?;
        let resp_body = Data::new(data, Some(pagination));
        Ok(resp_body)
    }
    fn get_by_id(pool: &State<DbPool>, pid: i32) -> Result<Data<Category>, diesel::result::Error> {
        let mut conn = establish_pg_connection(pool).expect("msg");
        // 配合 use crate::schema::category_table::dsl::*;
        let data = dsl::category_table.filter(dsl::category_id.eq(pid)).first(&mut conn)?;
        Ok(Data::new(data, None))
    }

    fn add_single(
        pool: &State<DbPool>,
        obj: &PostCategory,
    ) -> Result<Data<Category>, diesel::result::Error> {
        let mut conn = establish_pg_connection(pool).expect("msg");
        let data = diesel::insert_into(dsl::category_table)
            .values(obj)
            .returning(Category::as_returning())
            .get_result(&mut conn)?;
        Ok(Data::new(data, None))
    }

    fn delete_by_id(
        pool: &State<DbPool>,
        pid: i32,
    ) -> Result<Data<Category>, diesel::result::Error> {
        let mut conn = establish_pg_connection(pool).expect("msg");
        let data = diesel::delete(dsl::category_table.filter(dsl::category_id.eq(pid)))
            .get_result(&mut conn)?;
        Ok(Data::new(data, None))
    }

    fn update_by_id(
        pool: &State<DbPool>,
        pid: i32,
        obj: &PatchCategory,
    ) -> Result<Data<Category>, diesel::result::Error> {
        let mut conn = establish_pg_connection(pool).expect("msg");
        let data = diesel::update(dsl::category_table.filter(dsl::category_id.eq(pid)))
            .set((
                dsl::name.eq(&obj.name),
                dsl::description.eq(&obj.description),
                dsl::parent_id.eq(obj.parent_id),
                dsl::created_at.eq(obj.created_at),
                dsl::updated_at.eq(get_e8_time()),
            ))
            .get_result(&mut conn)?;
        Ok(Data::new(data, None))
    }
    fn filter(
        pool: &State<DbPool>,
        param: &RequestParam<Self::Item, Self::Filter>,
    ) -> Result<Data<Vec<Category>>, diesel::result::Error> {
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
        let total_count = dsl::category_table.count().get_result::<i64>(&mut conn)? as i32;
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
        let data = query.load::<Category>(&mut conn)?;
        let body = Data::new(data, Some(pagination));
        Ok(body)
    }
}

#[cfg(test)]
mod test {
    use rocket::State;

    use crab_rocket_schema::{DbPool, establish_pool};
    use obj_traits::{mapper::mapper_crud::MapperCRUD, request::request_param::RequestParam};

    use super::*;

    #[test]
    fn test_fetch_all_category_table() {
        let param = RequestParam::default(); // 預設的請求參數
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match CategoryMapper::get_all(pool, &param) {
            Ok(data) => {
                println!("{:#?}", data);
            }
            Err(e) => {
                eprintln!("Error fetching categories: {:?}", e);
            }
        }
    }

    #[test]
    fn test_fetch_category_by_id() {
        let test_id = 1; // 測試用的 ID
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match CategoryMapper::get_by_id(pool, test_id) {
            Ok(category) => {
                println!("{:#?}", category);
            }
            Err(e) => {
                eprintln!("Error fetching category by ID: {:?}", e);
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
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match CategoryMapper::add_single(pool, &new_category) {
            Ok(category) => {
                println!("Added category: {:#?}", category);
            }
            Err(e) => {
                eprintln!("Error adding category: {:?}", e);
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
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match CategoryMapper::update_by_id(pool, category_id, &updated_category) {
            Ok(category) => {
                println!("Updated category: {:#?}", category);
            }
            Err(e) => {
                eprintln!("Error updating category: {:?}", e);
            }
        }
    }

    #[test]
    fn test_delete_category() {
        let category_id = 1; // 測試用的 ID
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match CategoryMapper::delete_by_id(pool, category_id) {
            Ok(category) => {
                println!("Deleted category: {:#?}", category);
            }
            Err(e) => {
                eprintln!("Error deleting category: {:?}", e);
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
        let param = RequestParam::new(None, Some(filter_params));

        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match CategoryMapper::filter(pool, &param) {
            Ok(data) => {
                println!("{:#?}", data);
            }
            Err(e) => {
                eprintln!("Error filtering categories: {:?}", e);
            }
        }
    }
}
