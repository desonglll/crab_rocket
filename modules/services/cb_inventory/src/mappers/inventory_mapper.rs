use diesel::prelude::*;
use rocket::State;

use crab_rocket_schema::{DbPool, establish_pg_connection, schema::inventory_table::dsl};
use crab_rocket_utils::time::get_e8_time;
use obj_traits::{
    mapper::mapper_crud::MapperCRUD,
    request::{pagination_request_param::Pagination, request_param::RequestParam},
    response::data::Data,
};

use crate::models::{
    inventory::{Inventory, PatchInventory, PostInventory},
    inventory_filter::InventoryFilter,
};

pub struct InventoryMapper {}

impl MapperCRUD for InventoryMapper {
    type Item = Inventory;
    type PostItem = PostInventory;
    type PatchItem = PatchInventory;
    type Filter = InventoryFilter;
    fn get_all(
        pool: &State<DbPool>,
        param: &RequestParam<Self::Item, Self::Filter>,
    ) -> Result<Data<Vec<Inventory>>, diesel::result::Error> {
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
        let total_count = dsl::inventory_table.count().get_result::<i64>(&mut conn)? as i32;
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
        // let custom: Vec<Inventory> =
        //     diesel::sql_query("SELECT * FROM inventory_table").load::<Inventory>(&mut conn)?;

        // 分页查询
        let data = dsl::inventory_table
            .order(dsl::last_updated.desc())
            .limit(per_page as i64)
            .offset(((page - 1) * per_page) as i64)
            .load::<Inventory>(&mut conn)?;
        let resp_body = Data::new(data, Some(pagination));
        Ok(resp_body)
    }
    fn get_by_id(pool: &State<DbPool>, pid: i32) -> Result<Data<Inventory>, diesel::result::Error> {
        let mut conn = establish_pg_connection(pool).expect("msg");
        // 配合 use crate::schema::inventory_table::dsl::*;
        let data = dsl::inventory_table.filter(dsl::inventory_id.eq(pid)).first(&mut conn)?;
        Ok(Data::new(data, None))
    }

    fn add_single(
        pool: &State<DbPool>,
        obj: &PostInventory,
    ) -> Result<Data<Inventory>, diesel::result::Error> {
        let mut conn = establish_pg_connection(pool).expect("msg");
        let data = diesel::insert_into(dsl::inventory_table)
            .values(obj)
            .returning(Inventory::as_returning())
            .get_result(&mut conn)?;
        Ok(Data::new(data, None))
    }

    fn delete_by_id(
        pool: &State<DbPool>,
        pid: i32,
    ) -> Result<Data<Inventory>, diesel::result::Error> {
        let mut conn = establish_pg_connection(pool).expect("msg");
        let data = diesel::delete(dsl::inventory_table.filter(dsl::inventory_id.eq(pid)))
            .get_result(&mut conn)?;
        Ok(Data::new(data, None))
    }

    fn update_by_id(
        pool: &State<DbPool>,
        pid: i32,
        obj: &PatchInventory,
    ) -> Result<Data<Inventory>, diesel::result::Error> {
        let mut conn = establish_pg_connection(pool).expect("msg");
        let data = diesel::update(dsl::inventory_table.filter(dsl::inventory_id.eq(pid)))
            .set((
                dsl::product_id.eq(obj.product_id),
                dsl::location.eq(&obj.location),
                dsl::quantity.eq(obj.quantity),
                dsl::last_updated.eq(get_e8_time()), // 使用当前时间
            ))
            .get_result(&mut conn)?;
        Ok(Data::new(data, None))
    }
    fn filter(
        pool: &State<DbPool>,
        param: &RequestParam<Self::Item, Self::Filter>,
    ) -> Result<Data<Vec<Inventory>>, diesel::result::Error> {
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
        let total_count = dsl::inventory_table.count().get_result::<i64>(&mut conn)? as i32;
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
        let mut query = dsl::inventory_table.into_boxed();
        let filter = &param.filter;
        if let Some(f) = filter {
            // 篩選條件
            if let Some(inventory_id) = f.inventory_id {
                query = query.filter(dsl::inventory_id.eq(inventory_id));
            }

            if let Some(pid) = f.product_id {
                query = query.filter(dsl::product_id.eq(pid));
            }

            if let Some(location) = &f.location {
                query = query.filter(dsl::location.like(format!("%{}%", location)));
            }

            if let Some(quantity_min) = f.quantity_min {
                query = query.filter(dsl::quantity.ge(quantity_min));
            }

            if let Some(quantity_max) = f.quantity_max {
                query = query.filter(dsl::quantity.le(quantity_max));
            }

            if let Some(last_updated_min) = f.last_updated_min {
                query = query.filter(dsl::last_updated.ge(last_updated_min));
            }

            if let Some(last_updated_max) = f.last_updated_max {
                query = query.filter(dsl::last_updated.le(last_updated_max));
            }
        }
        let data = query.load::<Inventory>(&mut conn)?;
        let body = Data::new(data, Some(pagination));
        Ok(body)
    }
}

#[cfg(test)]
mod test {
    use rocket::State;

    use crab_rocket_schema::{DbPool, establish_pool};
    use obj_traits::{mapper::mapper_crud::MapperCRUD, request::request_param::RequestParam};

    use crate::models::{
        inventory::{PatchInventory, PostInventory},
        inventory_filter::InventoryFilter,
    };

    use super::*;

    #[test]
    fn test_fetch_all_inventory_table() {
        let param = RequestParam::default();
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match InventoryMapper::get_all(pool, &param) {
            Ok(data) => {
                assert!(!data.data.is_empty(), "Inventory table should not be empty");
            }
            Err(e) => {
                panic!("Error fetching all inventory: {:?}", e);
            }
        }
    }

    #[test]
    fn test_get_by_id() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);

        let pid = 1; // 假设ID为1的记录存在
        match InventoryMapper::get_by_id(pool, pid) {
            Ok(data) => {
                assert_eq!(
                    data.data.inventory_id, pid,
                    "Fetched inventory ID should match requested ID"
                );
            }
            Err(diesel::result::Error::NotFound) => {
                panic!("Inventory with ID {} not found", pid);
            }
            Err(e) => {
                panic!("Error fetching inventory by ID: {:?}", e);
            }
        }
    }

    #[test]
    fn test_add_single() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);

        let new_inventory = PostInventory {
            product_id: Some(1),
            location: Some("Test Location".to_string()),
            quantity: Some(100),
            last_updated: None,
        };
        match InventoryMapper::add_single(pool, &new_inventory) {
            Ok(data) => {
                assert_eq!(
                    data.data.location,
                    Some("Test Location".to_string()),
                    "Location should match the added inventory"
                );
            }
            Err(e) => {
                panic!("Error adding inventory: {:?}", e);
            }
        }
    }

    #[test]
    fn test_update_by_id() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);

        let pid = 2; // 假设ID为1的记录存在
        let updated_inventory = PatchInventory {
            product_id: Some(2),
            location: Some("Updated Location".to_string()),
            quantity: Some(200),
            last_updated: None,
        };
        match InventoryMapper::update_by_id(pool, pid, &updated_inventory) {
            Ok(data) => {
                assert_eq!(
                    data.data.location,
                    Some("Updated Location".to_string()),
                    "Location should be updated"
                );
            }
            Err(diesel::result::Error::NotFound) => {
                panic!("Inventory with ID {} not found for update", pid);
            }
            Err(e) => {
                panic!("Error updating inventory: {:?}", e);
            }
        }
    }

    #[test]
    fn test_delete_by_id() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);

        let pid = 3; // 假设ID为1的记录存在
        match InventoryMapper::delete_by_id(pool, pid) {
            Ok(data) => {
                assert_eq!(
                    data.data.inventory_id, pid,
                    "Deleted inventory ID should match requested ID"
                );
            }
            Err(diesel::result::Error::NotFound) => {
                panic!("Inventory with ID {} not found for deletion", pid);
            }
            Err(e) => {
                panic!("Error deleting inventory: {:?}", e);
            }
        }
    }

    #[test]
    fn test_filter() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);

        let filter = InventoryFilter {
            inventory_id: Some(1),
            product_id: None,
            location: None,
            quantity_min: None,
            quantity_max: None,
            last_updated_min: None,
            last_updated_max: None,
        };
        let param = RequestParam::new(None, Some(filter));
        match InventoryMapper::filter(pool, &param) {
            Ok(data) => {
                assert!(
                    data.data.iter().all(|item| item.inventory_id == 1),
                    "Filtered result should have inventory_id 1"
                );
            }
            Err(e) => {
                panic!("Error filtering inventory: {:?}", e);
            }
        }
    }
}
