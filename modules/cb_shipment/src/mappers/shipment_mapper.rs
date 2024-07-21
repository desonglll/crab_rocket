use obj_traits::{
    mapper::mapper_crud::MapperCRUD,
    request::{
        pagination_request_param::{Pagination, PaginationParam},
        request_param::RequestParam,
    },
    response::data::Data,
};

use crate::models::{
    shipment::{PatchShipment, PostShipment, Shipment},
    shipment_filter::ShipmentFilter,
};
use crab_rocket_schema::schema::shipment_table::dsl;
use diesel::prelude::*;

pub struct ShipmentMapper {}

impl MapperCRUD for ShipmentMapper {
    type Item = Shipment;
    type PostItem = PostShipment;
    type PatchItem = PatchShipment;
    type Param = RequestParam<PaginationParam, ShipmentFilter>;
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
        let total_count = dsl::shipment_table.count().get_result::<i64>(conn)? as i32;
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
        //     diesel::sql_query("SELECT * FROM shipment_table").load::<Shipment>(conn)?;

        // 分页查询
        let data = dsl::shipment_table
            .order(dsl::shipment_date.desc())
            .limit(per_page as i64)
            .offset(((page - 1) * per_page) as i64)
            .load::<Shipment>(conn)?;
        let resp_body = Data::new(data, pagination);
        Ok(resp_body)
    }
    fn get_by_id(conn: &mut PgConnection, pid: i32) -> Result<Shipment, diesel::result::Error> {
        // 配合 use crate::schema::shipment_table::dsl::*;
        dsl::shipment_table.filter(dsl::shipment_id.eq(pid)).first(conn)
    }

    fn add_single(
        conn: &mut PgConnection,
        obj: &PostShipment,
    ) -> Result<Shipment, diesel::result::Error> {
        match diesel::insert_into(dsl::shipment_table)
            .values(obj)
            .returning(Shipment::as_returning())
            .get_result(conn)
        {
            Ok(inserted_shipment) => Ok(inserted_shipment),
            Err(e) => Err(e),
        }
    }

    fn delete_by_id(conn: &mut PgConnection, pid: i32) -> Result<Shipment, diesel::result::Error> {
        diesel::delete(dsl::shipment_table.filter(dsl::shipment_id.eq(pid))).get_result(conn)
    }

    fn update_by_id(
        conn: &mut PgConnection,
        pid: i32,
        obj: &PatchShipment,
    ) -> Result<Shipment, diesel::result::Error> {
        diesel::update(dsl::shipment_table.filter(dsl::shipment_id.eq(pid)))
            .set((
                dsl::order_id.eq(obj.order_id),
                dsl::shipment_date.eq(obj.shipment_date),
                dsl::delivery_address.eq(&obj.delivery_address),
                dsl::status.eq(&obj.status),
            ))
            .get_result(conn)
    }
    fn filter(
        conn: &mut PgConnection,
        param: &RequestParam<PaginationParam, ShipmentFilter>,
    ) -> Result<Data<Vec<Shipment>>, diesel::result::Error> {
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
        let total_count = dsl::shipment_table.count().get_result::<i64>(conn)? as i32;
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
        let data = query.load::<Shipment>(conn)?;
        let body = Data::new(data, pagination);
        Ok(body)
    }
}

mod test {

    #[test]
    fn test_fetch_all_shipment_table() {
        use crab_rocket_schema::establish_pg_connection;
        use obj_traits::{mapper::mapper_crud::MapperCRUD, request::request_param::RequestParam};

        use super::ShipmentMapper;
        let param = RequestParam::default();
        match establish_pg_connection() {
            Ok(mut conn) => match ShipmentMapper::get_all(&mut conn, &param) {
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