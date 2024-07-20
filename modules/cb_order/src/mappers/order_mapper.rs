use obj_traits::{
    mapper::mapper_crud::MapperCRUD,
    request::{
        pagination_request_param::{Pagination, PaginationParam},
        request_param::RequestParam,
    },
    response::data::Data,
};

use crate::models::{
    order::{Order, PatchOrder, PostOrder},
    order_filter::OrderFilter,
};
use crab_rocket_schema::schema::order_table::dsl;
use diesel::prelude::*;

pub struct OrderMapper {}

impl MapperCRUD for OrderMapper {
    type Item = Order;
    type PostItem = PostOrder;
    type PatchItem = PatchOrder;
    type Param = RequestParam<PaginationParam, OrderFilter>;
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
        let total_count = dsl::order_table.count().get_result::<i64>(conn)? as i32;
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
        //     diesel::sql_query("SELECT * FROM order_table").load::<Order>(conn)?;

        // 分页查询
        let data = dsl::order_table
            .order(dsl::order_date.desc())
            .limit(per_page as i64)
            .offset(((page - 1) * per_page) as i64)
            .load::<Order>(conn)?;
        let resp_body = Data::new(data, pagination);
        Ok(resp_body)
    }
    fn get_by_id(conn: &mut PgConnection, pid: i32) -> Result<Order, diesel::result::Error> {
        // 配合 use crate::schema::order_table::dsl::*;
        dsl::order_table.filter(dsl::order_id.eq(pid)).first(conn)
    }

    fn add_single(
        conn: &mut PgConnection,
        obj: &PostOrder,
    ) -> Result<Order, diesel::result::Error> {
        match diesel::insert_into(dsl::order_table)
            .values(obj)
            .returning(Order::as_returning())
            .get_result(conn)
        {
            Ok(inserted_order) => Ok(inserted_order),
            Err(e) => Err(e),
        }
    }

    fn delete_by_id(conn: &mut PgConnection, pid: i32) -> Result<Order, diesel::result::Error> {
        diesel::delete(dsl::order_table.filter(dsl::order_id.eq(pid))).get_result(conn)
    }

    fn update_by_id(
        conn: &mut PgConnection,
        pid: i32,
        obj: &PatchOrder,
    ) -> Result<Order, diesel::result::Error> {
        diesel::update(dsl::order_table.filter(dsl::order_id.eq(pid)))
            .set((
                dsl::customer_id.eq(obj.customer_id),
                dsl::order_date.eq(obj.order_date),
                dsl::total_amount.eq(obj.total_amount),
                dsl::status.eq(&obj.status),
            ))
            .get_result(conn)
    }
    fn filter(
        conn: &mut PgConnection,
        param: &RequestParam<PaginationParam, OrderFilter>,
    ) -> Result<Data<Vec<Order>>, diesel::result::Error> {
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
        let total_count = dsl::order_table.count().get_result::<i64>(conn)? as i32;
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
        let data = query.load::<Order>(conn)?;
        let body = Data::new(data, pagination);
        Ok(body)
    }
}

mod test {

    #[test]
    fn test_fetch_all_order_table() {
        use crab_rocket_schema::establish_pg_connection;
        use obj_traits::{mapper::mapper_crud::MapperCRUD, request::request_param::RequestParam};

        use super::OrderMapper;
        let param = RequestParam::default();
        match establish_pg_connection() {
            Ok(mut conn) => match OrderMapper::get_all(&mut conn, &param) {
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
