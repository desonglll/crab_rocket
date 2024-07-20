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
    inventory::{Inventory, PatchInventory, PostInventory},
    inventory_filter::InventoryFilter,
};
use crab_rocket_schema::schema::inventory_table::dsl;
use diesel::prelude::*;

pub struct InventoryMapper {}

impl MapperCRUD for InventoryMapper {
    type Item = Inventory;
    type PostItem = PostInventory;
    type PatchItem = PatchInventory;
    type Param = RequestParam<PaginationParam, InventoryFilter>;
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
        let total_count = dsl::inventory_table.count().get_result::<i64>(conn)? as i32;
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
        //     diesel::sql_query("SELECT * FROM inventory_table").load::<Inventory>(conn)?;

        // 分页查询
        let data = dsl::inventory_table
            .order(dsl::last_updated.desc())
            .limit(per_page as i64)
            .offset(((page - 1) * per_page) as i64)
            .load::<Inventory>(conn)?;
        let resp_body = Data::new(data, pagination);
        Ok(resp_body)
    }
    fn get_by_id(conn: &mut PgConnection, pid: i32) -> Result<Inventory, diesel::result::Error> {
        // 配合 use crate::schema::inventory_table::dsl::*;
        dsl::inventory_table.filter(dsl::inventory_id.eq(pid)).first(conn)
    }

    fn add_single(
        conn: &mut PgConnection,
        obj: &PostInventory,
    ) -> Result<Inventory, diesel::result::Error> {
        match diesel::insert_into(dsl::inventory_table)
            .values(obj)
            .returning(Inventory::as_returning())
            .get_result(conn)
        {
            Ok(inserted_inventory) => Ok(inserted_inventory),
            Err(e) => Err(e),
        }
    }

    fn delete_by_id(conn: &mut PgConnection, pid: i32) -> Result<Inventory, diesel::result::Error> {
        diesel::delete(dsl::inventory_table.filter(dsl::inventory_id.eq(pid))).get_result(conn)
    }

    fn update_by_id(
        conn: &mut PgConnection,
        pid: i32,
        obj: &PatchInventory,
    ) -> Result<Inventory, diesel::result::Error> {
        diesel::update(dsl::inventory_table.filter(dsl::inventory_id.eq(pid)))
            .set((
                dsl::product_id.eq(obj.product_id),
                dsl::location.eq(&obj.location),
                dsl::quantity.eq(obj.quantity),
                dsl::last_updated.eq(get_e8_time()), // 使用当前时间
            ))
            .get_result(conn)
    }
    fn filter(
        conn: &mut PgConnection,
        param: &RequestParam<PaginationParam, InventoryFilter>,
    ) -> Result<Data<Vec<Inventory>>, diesel::result::Error> {
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
        let total_count = dsl::inventory_table.count().get_result::<i64>(conn)? as i32;
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
        let data = query.load::<Inventory>(conn)?;
        let body = Data::new(data, pagination);
        Ok(body)
    }
}

mod test {

    #[test]
    fn test_fetch_all_inventory_table() {
        use crab_rocket_schema::establish_pg_connection;
        use obj_traits::{mapper::mapper_crud::MapperCRUD, request::request_param::RequestParam};

        use super::InventoryMapper;
        let param = RequestParam::default();
        match establish_pg_connection() {
            Ok(mut conn) => match InventoryMapper::get_all(&mut conn, &param) {
                Ok(data) => {
                    println!("{:#?}", data);
                }
                Err(e) => {
                    println!("{:?}", e);
                }
            },
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }
}
