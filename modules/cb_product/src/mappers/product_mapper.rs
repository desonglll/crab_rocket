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
    product::{PatchProduct, PostProduct, Product},
    product_filter::ProductFilter,
};
use crab_rocket_schema::schema::product_table::dsl;
use diesel::prelude::*;

pub struct ProductMapper {}

impl MapperCRUD for ProductMapper {
    type Item = Product;
    type PostItem = PostProduct;
    type PatchItem = PatchProduct;
    type Param = RequestParam<PaginationParam, ProductFilter>;
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
        let total_count = dsl::product_table.count().get_result::<i64>(conn)? as i32;
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
        // let custom: Vec<Product> =
        //     diesel::sql_query("SELECT * FROM product_table").load::<Product>(conn)?;

        // 分页查询
        let data = dsl::product_table
            .order(dsl::updated_at.desc())
            .limit(per_page as i64)
            .offset(((page - 1) * per_page) as i64)
            .load::<Product>(conn)?;
        let resp_body = Data::new(data, pagination);
        Ok(resp_body)
    }
    fn get_by_id(conn: &mut PgConnection, pid: i32) -> Result<Product, diesel::result::Error> {
        // 配合 use crate::schema::product_table::dsl::*;
        dsl::product_table.filter(dsl::product_id.eq(pid)).first(conn)
    }

    fn add_single(
        conn: &mut PgConnection,
        obj: &PostProduct,
    ) -> Result<Product, diesel::result::Error> {
        match diesel::insert_into(dsl::product_table)
            .values(obj)
            .returning(Product::as_returning())
            .get_result(conn)
        {
            Ok(inserted_product) => Ok(inserted_product),
            Err(e) => Err(e),
        }
    }

    fn delete_by_id(conn: &mut PgConnection, pid: i32) -> Result<Product, diesel::result::Error> {
        diesel::delete(dsl::product_table.filter(dsl::product_id.eq(pid))).get_result(conn)
    }

    fn update_by_id(
        conn: &mut PgConnection,
        pid: i32,
        obj: &PatchProduct,
    ) -> Result<Product, diesel::result::Error> {
        diesel::update(dsl::product_table.filter(dsl::product_id.eq(pid)))
            .set((
                dsl::name.eq(obj.name()),
                dsl::address.eq(obj.address()),
                dsl::phone_number.eq(obj.phone_number()),
                dsl::email.eq(obj.email()),
                dsl::created_at.eq(obj.created_at()),
                dsl::updated_at.eq(get_e8_time()),
            ))
            .get_result(conn)
    }
    fn filter(
        conn: &mut PgConnection,
        param: &RequestParam<PaginationParam, ProductFilter>,
    ) -> Result<Data<Vec<Product>>, diesel::result::Error> {
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
        let total_count = dsl::product_table.count().get_result::<i64>(conn)? as i32;
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
        let mut query = dsl::product_table.into_boxed();
        let filter = &param.filter;
        if let Some(f) = filter {
            if let Some(product_id) = f.product_id {
                query = query.filter(dsl::product_id.eq(product_id));
            }
            if let Some(name) = &f.name {
                query = query.filter(dsl::name.like(format!("%{}%", name)));
            }
            if let Some(address) = &f.address {
                query = query.filter(dsl::address.like(format!("%{}%", address)));
            }
            if let Some(phone_number) = &f.phone_number {
                query = query.filter(dsl::phone_number.like(format!("%{}%", phone_number)));
            }
            if let Some(email) = &f.email {
                query = query.filter(dsl::email.like(format!("%{}%", email)));
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
        let data = query.load::<Product>(conn)?;
        let body = Data::new(data, pagination);
        Ok(body)
    }
}

mod test {

    #[test]
    fn test_fetch_all_product_table() {
        use crab_rocket_schema::establish_pg_connection;
        use obj_traits::{mapper::mapper_crud::MapperCRUD, request::request_param::RequestParam};

        use super::ProductMapper;
        let param = RequestParam::default();
        match establish_pg_connection() {
            Ok(mut conn) => match ProductMapper::get_all(&mut conn, &param) {
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
