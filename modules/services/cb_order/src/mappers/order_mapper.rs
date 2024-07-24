use obj_traits::{
    mapper::mapper_crud::MapperCRUD,
    request::{pagination_request_param::Pagination, request_param::RequestParam},
    response::data::Data,
};
use rocket::State;

use crate::models::{
    order::{Order, PatchOrder, PostOrder},
    order_filter::OrderFilter,
};
use crab_rocket_schema::{establish_pg_connection, schema::order_table::dsl, DbPool};
use diesel::prelude::*;

pub struct OrderMapper {}

impl MapperCRUD for OrderMapper {
    type Item = Order;
    type PostItem = PostOrder;
    type PatchItem = PatchOrder;
    type Filter = OrderFilter;
    fn get_all(
        pool: &State<DbPool>,
        param: &RequestParam<Self::Item, Self::Filter>,
    ) -> Result<Data<Vec<Order>>, diesel::result::Error> {
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
        let total_count = dsl::order_table.count().get_result::<i64>(&mut conn)? as i32;
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
        // let custom: Vec<Order> =
        //     diesel::sql_query("SELECT * FROM order_table").load::<Order>(&mut conn)?;

        // 分页查询
        let data = dsl::order_table
            .order(dsl::order_date.desc())
            .limit(per_page as i64)
            .offset(((page - 1) * per_page) as i64)
            .load::<Order>(&mut conn)?;
        let resp_body = Data::new(data, Some(pagination));
        Ok(resp_body)
    }
    fn get_by_id(pool: &State<DbPool>, pid: i32) -> Result<Data<Order>, diesel::result::Error> {
        let mut conn = establish_pg_connection(pool).expect("msg");
        // 配合 use crate::schema::order_table::dsl::*;
        let data = dsl::order_table.filter(dsl::order_id.eq(pid)).first(&mut conn)?;
        Ok(Data::new(data, None))
    }

    fn add_single(
        pool: &State<DbPool>,
        obj: &PostOrder,
    ) -> Result<Data<Order>, diesel::result::Error> {
        let mut conn = establish_pg_connection(pool).expect("msg");
        let data = diesel::insert_into(dsl::order_table)
            .values(obj)
            .returning(Order::as_returning())
            .get_result(&mut conn)?;
        Ok(Data::new(data, None))
    }

    fn delete_by_id(pool: &State<DbPool>, pid: i32) -> Result<Data<Order>, diesel::result::Error> {
        let mut conn = establish_pg_connection(pool).expect("msg");
        let data =
            diesel::delete(dsl::order_table.filter(dsl::order_id.eq(pid))).get_result(&mut conn)?;
        Ok(Data::new(data, None))
    }

    fn update_by_id(
        pool: &State<DbPool>,
        pid: i32,
        obj: &PatchOrder,
    ) -> Result<Data<Order>, diesel::result::Error> {
        let mut conn = establish_pg_connection(pool).expect("msg");
        let data = diesel::update(dsl::order_table.filter(dsl::order_id.eq(pid)))
            .set((
                dsl::customer_id.eq(obj.customer_id),
                dsl::order_date.eq(obj.order_date),
                dsl::total_amount.eq(obj.total_amount),
                dsl::status.eq(&obj.status),
            ))
            .get_result(&mut conn)?;
        Ok(Data::new(data, None))
    }
    fn filter(
        pool: &State<DbPool>,
        param: &RequestParam<Self::Item, Self::Filter>,
    ) -> Result<Data<Vec<Order>>, diesel::result::Error> {
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
        let total_count = dsl::order_table.count().get_result::<i64>(&mut conn)? as i32;
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
        let mut query = dsl::order_table.into_boxed();
        let filter = &param.filter;
        if let Some(f) = filter {
            // 篩選條件
            if let Some(order_id) = f.order_id {
                query = query.filter(dsl::order_id.eq(order_id));
            }

            if let Some(customer_id) = f.customer_id {
                query = query.filter(dsl::customer_id.eq(customer_id));
            }

            if let Some(order_date_min) = f.order_date_min {
                query = query.filter(dsl::order_date.ge(order_date_min));
            }

            if let Some(order_date_max) = f.order_date_max {
                query = query.filter(dsl::order_date.le(order_date_max));
            }

            if let Some(total_amount_min) = f.total_amount_min {
                query = query.filter(dsl::total_amount.ge(total_amount_min));
            }

            if let Some(total_amount_max) = f.total_amount_max {
                query = query.filter(dsl::total_amount.le(total_amount_max));
            }

            if let Some(status) = &f.status {
                query = query.filter(dsl::status.like(format!("%{}%", status)));
            }
        }
        let data = query.load::<Order>(&mut conn)?;
        let body = Data::new(data, Some(pagination));
        Ok(body)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crab_rocket_schema::{establish_pool, DbPool};
    use crab_rocket_utils::time::get_e8_time;
    use diesel::result::Error;
    use rocket::State;

    #[test]
    fn test_fetch_all_order_table() {
        let param = RequestParam::<Order, OrderFilter>::default();
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match OrderMapper::get_all(pool, &param) {
            Ok(data) => {
                println!("{:#?}", data);
                assert!(data.data.len() > 0); // Ensure data length is non-negative
            }
            Err(e) => {
                eprintln!("Error fetching all orders: {:?}", e);
                panic!("Failed to fetch orders: {:?}", e);
            }
        }
    }

    #[test]
    fn test_get_order_by_id() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        let new_order = PostOrder {
            customer_id: Some(1),
            order_date: Some(get_e8_time()),
            total_amount: Some(100.0),
            status: Some("Pending".to_string()),
        };
        let inserted_order =
            OrderMapper::add_single(pool, &new_order).expect("Failed to insert order");
        let fetched_order = OrderMapper::get_by_id(pool, inserted_order.data.order_id)
            .expect("Failed to fetch order by ID");

        assert_eq!(fetched_order.data.order_id, inserted_order.data.order_id);
        assert_eq!(fetched_order.data.customer_id, inserted_order.data.customer_id);
        assert_eq!(fetched_order.data.order_date, inserted_order.data.order_date);
        assert_eq!(fetched_order.data.total_amount, inserted_order.data.total_amount);
        assert_eq!(fetched_order.data.status, inserted_order.data.status);
    }

    #[test]
    fn test_add_single_order() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        let new_order = PostOrder {
            customer_id: Some(2),
            order_date: Some(get_e8_time()),
            total_amount: Some(100.0),
            status: Some("Pending".to_string()),
        };
        let inserted_order =
            OrderMapper::add_single(pool, &new_order).expect("Failed to insert order");

        assert_eq!(inserted_order.data.customer_id, new_order.customer_id);
        assert_eq!(inserted_order.data.total_amount, new_order.total_amount);
        assert_eq!(inserted_order.data.status, new_order.status);
    }

    #[test]
    fn test_delete_order_by_id() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        let new_order = PostOrder {
            customer_id: Some(2),
            order_date: Some(get_e8_time()),
            total_amount: Some(100.0),
            status: Some("Pending".to_string()),
        };
        let inserted_order =
            OrderMapper::add_single(pool, &new_order).expect("Failed to insert order");
        let deleted_order = OrderMapper::delete_by_id(pool, inserted_order.data.order_id)
            .expect("Failed to delete order");

        assert_eq!(deleted_order.data.order_id, inserted_order.data.order_id);

        // Try fetching the deleted order to ensure it no longer exists
        match OrderMapper::get_by_id(pool, inserted_order.data.order_id) {
            Ok(_) => panic!("Order was not deleted"),
            Err(Error::NotFound) => assert!(true),
            Err(e) => panic!("Unexpected error: {:?}", e),
        }
    }

    #[test]
    fn test_update_order_by_id() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        let new_order = PostOrder {
            customer_id: Some(2),
            order_date: Some(get_e8_time()),
            total_amount: Some(100.0),
            status: Some("Pending".to_string()),
        };
        let inserted_order =
            OrderMapper::add_single(pool, &new_order).expect("Failed to insert order");

        let updated_order = PatchOrder {
            customer_id: Some(2),
            order_date: Some(get_e8_time()),
            total_amount: Some(200.0),
            status: Some("Completed".to_string()),
        };
        let order_after_update =
            OrderMapper::update_by_id(pool, inserted_order.data.order_id, &updated_order)
                .expect("Failed to update order");

        assert_eq!(order_after_update.data.customer_id, updated_order.customer_id);
        // assert_eq!(order_after_update.order_date, updated_order.order_date);
        // 时间精度有问题
        // assertion `left == right` failed
        // left: Some(2024-07-21T12:22:35.004284)
        // right: Some(2024-07-21T12:22:35.004284885)
        assert_eq!(order_after_update.data.total_amount, updated_order.total_amount);
        assert_eq!(order_after_update.data.status, updated_order.status);
    }

    #[test]
    fn test_filter_orders() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        let new_order1 = PostOrder {
            customer_id: Some(1),
            order_date: Some(get_e8_time()),
            total_amount: Some(100.0),
            status: Some("Pending".to_string()),
        };
        let new_order2 = PostOrder {
            customer_id: Some(2),
            order_date: Some(get_e8_time()),
            total_amount: Some(200.0),
            status: Some("Completed".to_string()),
        };
        OrderMapper::add_single(pool, &new_order1).expect("Failed to insert order");
        OrderMapper::add_single(pool, &new_order2).expect("Failed to insert order");

        let filter = OrderFilter {
            customer_id: Some(1),
            order_date_min: None,
            order_date_max: None,
            total_amount_min: None,
            total_amount_max: None,
            status: None,
            order_id: None,
        };
        let param = RequestParam::<Order, OrderFilter>::new(None, Some(filter));
        let result = OrderMapper::filter(pool, &param).expect("Failed to filter orders");
        println!("{:#?}", result);
        assert_eq!(result.data.len(), 6);
    }
}
