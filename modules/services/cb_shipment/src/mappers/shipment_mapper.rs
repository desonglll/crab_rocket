use obj_traits::{
    mapper::mapper_crud::MapperCRUD,
    request::{pagination_request_param::Pagination, request_param::RequestParam},
    response::data::Data,
};
use rocket::State;

use crate::models::{
    shipment::{PatchShipment, PostShipment, Shipment},
    shipment_filter::ShipmentFilter,
};
use crab_rocket_schema::{establish_pg_connection, schema::shipment_table::dsl, DbPool};
use diesel::prelude::*;

pub struct ShipmentMapper {}

impl MapperCRUD for ShipmentMapper {
    type Item = Shipment;
    type PostItem = PostShipment;
    type PatchItem = PatchShipment;
    type Filter = ShipmentFilter;
    fn get_all(
        pool: &State<DbPool>,
        param: &RequestParam<Self::Item, Self::Filter>,
    ) -> Result<Data<Vec<Shipment>>, diesel::result::Error> {
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
        let total_count = dsl::shipment_table.count().get_result::<i64>(&mut conn)? as i32;
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
        // let custom: Vec<Shipment> =
        //     diesel::sql_query("SELECT * FROM shipment_table").load::<Shipment>(&mut conn)?;

        // 分页查询
        let data = dsl::shipment_table
            .order(dsl::shipment_date.desc())
            .limit(per_page as i64)
            .offset(((page - 1) * per_page) as i64)
            .load::<Shipment>(&mut conn)?;
        let resp_body = Data::new(data, Some(pagination));
        Ok(resp_body)
    }
    fn get_by_id(pool: &State<DbPool>, pid: i32) -> Result<Data<Shipment>, diesel::result::Error> {
        let mut conn = establish_pg_connection(pool).expect("msg");
        // 配合 use crate::schema::shipment_table::dsl::*;
        let data = dsl::shipment_table.filter(dsl::shipment_id.eq(pid)).first(&mut conn)?;
        Ok(Data::new(data, None))
    }

    fn add_single(
        pool: &State<DbPool>,
        obj: &PostShipment,
    ) -> Result<Data<Shipment>, diesel::result::Error> {
        let mut conn = establish_pg_connection(pool).expect("msg");
        let data = diesel::insert_into(dsl::shipment_table)
            .values(obj)
            .returning(Shipment::as_returning())
            .get_result(&mut conn)?;
        Ok(Data::new(data, None))
    }

    fn delete_by_id(
        pool: &State<DbPool>,
        pid: i32,
    ) -> Result<Data<Shipment>, diesel::result::Error> {
        let mut conn = establish_pg_connection(pool).expect("msg");
        let data = diesel::delete(dsl::shipment_table.filter(dsl::shipment_id.eq(pid)))
            .get_result(&mut conn)?;
        Ok(Data::new(data, None))
    }

    fn update_by_id(
        pool: &State<DbPool>,
        pid: i32,
        obj: &PatchShipment,
    ) -> Result<Data<Shipment>, diesel::result::Error> {
        let mut conn = establish_pg_connection(pool).expect("msg");
        let data = diesel::update(dsl::shipment_table.filter(dsl::shipment_id.eq(pid)))
            .set((
                dsl::order_id.eq(obj.order_id),
                dsl::shipment_date.eq(obj.shipment_date),
                dsl::delivery_address.eq(&obj.delivery_address),
                dsl::status.eq(&obj.status),
            ))
            .get_result(&mut conn)?;
        Ok(Data::new(data, None))
    }
    fn filter(
        pool: &State<DbPool>,
        param: &RequestParam<Self::Item, Self::Filter>,
    ) -> Result<Data<Vec<Shipment>>, diesel::result::Error> {
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
        let total_count = dsl::shipment_table.count().get_result::<i64>(&mut conn)? as i32;
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
        let mut query = dsl::shipment_table.into_boxed();
        let filter = &param.filter;
        if let Some(f) = filter {
            if let Some(shipment_id) = f.shipment_id {
                query = query.filter(dsl::shipment_id.eq(shipment_id));
            }

            if let Some(order_id) = f.order_id {
                query = query.filter(dsl::order_id.eq(order_id));
            }

            if let Some(shipment_date_min) = f.shipment_date_min {
                query = query.filter(dsl::shipment_date.ge(shipment_date_min));
            }

            if let Some(shipment_date_max) = f.shipment_date_max {
                query = query.filter(dsl::shipment_date.le(shipment_date_max));
            }

            if let Some(delivery_address) = &f.delivery_address {
                query = query.filter(dsl::delivery_address.like(format!("%{}%", delivery_address)));
            }

            if let Some(status) = &f.status {
                query = query.filter(dsl::status.like(format!("%{}%", status)));
            }
        }
        let data = query.load::<Shipment>(&mut conn)?;
        let body = Data::new(data, Some(pagination));
        Ok(body)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crab_rocket_schema::{establish_pool, DbPool};
    use crab_rocket_utils::time::get_e8_time;
    use rocket::State;

    // Helper function to create a new shipment for testing
    fn create_test_shipment(pool: &State<DbPool>) -> Result<Data<Shipment>, diesel::result::Error> {
        let new_shipment = PostShipment {
            order_id: Some(1),
            shipment_date: Some(get_e8_time()),
            delivery_address: Some("123 Test St".to_string()),
            status: Some("Pending".to_string()),
        };
        ShipmentMapper::add_single(pool, &new_shipment)
    }

    #[test]
    fn test_get_all_shipments() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        let param = RequestParam::demo();
        match ShipmentMapper::get_all(pool, &param) {
            Ok(data) => println!("{:#?}", data),
            Err(e) => panic!("Error fetching all shipments: {:?}", e),
        }
    }

    #[test]
    fn test_get_by_id() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        let shipment = create_test_shipment(pool).expect("Failed to create test shipment");

        match ShipmentMapper::get_by_id(pool, shipment.data.shipment_id) {
            Ok(data) => assert_eq!(data.data.shipment_id, shipment.data.shipment_id),
            Err(e) => panic!("Error fetching shipment by ID: {:?}", e),
        }
    }

    #[test]
    fn test_add_single_shipment() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        let new_shipment = PostShipment {
            order_id: Some(1),
            shipment_date: Some(get_e8_time()),
            delivery_address: Some("123 Test St".to_string()),
            status: Some("Pending".to_string()),
        };
        match ShipmentMapper::add_single(pool, &new_shipment) {
            Ok(data) => {
                assert_eq!(data.data.order_id, new_shipment.order_id);
                assert_eq!(data.data.delivery_address, new_shipment.delivery_address);
            }
            Err(e) => panic!("Error adding shipment: {:?}", e),
        }
    }

    #[test]
    fn test_delete_by_id() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        let shipment = create_test_shipment(pool).expect("Failed to create test shipment");

        match ShipmentMapper::delete_by_id(pool, shipment.data.shipment_id) {
            Ok(deleted_shipment) => {
                assert_eq!(deleted_shipment.data.shipment_id, shipment.data.shipment_id);
                // Verify that it has been deleted
                assert!(ShipmentMapper::get_by_id(pool, shipment.data.shipment_id).is_err());
            }
            Err(e) => panic!("Error deleting shipment: {:?}", e),
        }
    }

    #[test]
    fn test_update_by_id() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        let shipment = create_test_shipment(pool).expect("Failed to create test shipment");

        let updated_info = PatchShipment {
            order_id: Some(2),
            shipment_date: Some(get_e8_time()),
            delivery_address: Some("456 Updated St".to_string()),
            status: Some("Shipped".to_string()),
        };

        match ShipmentMapper::update_by_id(pool, shipment.data.shipment_id, &updated_info) {
            Ok(updated_shipment) => {
                assert_eq!(updated_shipment.data.order_id, updated_info.order_id);
                assert_eq!(updated_shipment.data.delivery_address, updated_info.delivery_address);
                assert_eq!(updated_shipment.data.status, updated_info.status);
            }
            Err(e) => panic!("Error updating shipment: {:?}", e),
        }
    }

    #[test]
    fn test_filter_shipments() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        let filter = ShipmentFilter {
            shipment_id: None,
            order_id: None,
            shipment_date_min: None,
            shipment_date_max: None,
            delivery_address: None,
            status: None,
        };
        let param = RequestParam::new(None, Some(filter));

        match ShipmentMapper::filter(pool, &param) {
            Ok(data) => {
                assert!(!data.data.is_empty());
            }
            Err(e) => panic!("Error filtering shipments: {:?}", e),
        }
    }
}
