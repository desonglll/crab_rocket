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
                dsl::user_id.eq(obj.user_id),
                dsl::name.eq(&obj.name),
                dsl::description.eq(&obj.description),
                dsl::price.eq(obj.price),
                dsl::discount_price.eq(obj.discount_price),
                dsl::is_discounted.eq(obj.is_discounted),
                dsl::is_valid.eq(obj.is_valid),
                dsl::inventory.eq(obj.inventory),
                dsl::is_in_stock.eq(obj.is_in_stock),
                dsl::updated_at.eq(get_e8_time()),
                dsl::supplier_id.eq(obj.supplier_id),
                dsl::weight.eq(obj.weight),
                dsl::dimensions.eq(&obj.dimensions),
                dsl::status.eq(&obj.status),
                dsl::public.eq(obj.public),
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
            // 篩選條件
            if let Some(product_id) = f.product_id {
                query = query.filter(dsl::product_id.eq(product_id));
            }

            if let Some(user_id) = f.user_id {
                query = query.filter(dsl::user_id.eq(user_id));
            }

            if let Some(name) = &f.name {
                query = query.filter(dsl::name.ilike(format!("%{}%", name)));
            }

            if let Some(description) = &f.description {
                query = query.filter(dsl::description.ilike(format!("%{}%", description)));
            }

            if let Some(sku) = &f.sku {
                query = query.filter(dsl::sku.eq(sku));
            }

            if let Some(image) = &f.image {
                query = query.filter(dsl::image.ilike(format!("%{}%", image)));
            }

            if let Some(price_min) = f.price_min {
                query = query.filter(dsl::price.ge(price_min));
            }

            if let Some(price_max) = f.price_max {
                query = query.filter(dsl::price.le(price_max));
            }

            if let Some(discount_price_min) = f.discount_price_min {
                query = query.filter(dsl::discount_price.ge(discount_price_min));
            }

            if let Some(discount_price_max) = f.discount_price_max {
                query = query.filter(dsl::discount_price.le(discount_price_max));
            }

            if let Some(is_discounted) = f.is_discounted {
                query = query.filter(dsl::is_discounted.eq(is_discounted));
            }

            if let Some(is_valid) = f.is_valid {
                query = query.filter(dsl::is_valid.eq(is_valid));
            }

            if let Some(inventory_min) = f.inventory_min {
                query = query.filter(dsl::inventory.ge(inventory_min));
            }

            if let Some(inventory_max) = f.inventory_max {
                query = query.filter(dsl::inventory.le(inventory_max));
            }

            if let Some(is_in_stock) = f.is_in_stock {
                query = query.filter(dsl::is_in_stock.eq(is_in_stock));
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

            if let Some(supplier_id) = f.supplier_id {
                query = query.filter(dsl::supplier_id.eq(supplier_id));
            }

            if let Some(weight_min) = f.weight_min {
                query = query.filter(dsl::weight.ge(weight_min));
            }

            if let Some(weight_max) = f.weight_max {
                query = query.filter(dsl::weight.le(weight_max));
            }

            if let Some(dimensions) = &f.dimensions {
                query = query.filter(dsl::dimensions.ilike(format!("%{}%", dimensions)));
            }

            if let Some(status) = &f.status {
                query = query.filter(dsl::status.ilike(format!("%{}%", status)));
            }

            if let Some(public) = f.public {
                query = query.filter(dsl::public.eq(public));
            }
        }
        let data = query.load::<Product>(conn)?;
        let body = Data::new(data, pagination);
        Ok(body)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crab_rocket_schema::establish_pg_connection;
    use obj_traits::request::pagination_request_param::PaginationParamTrait;

    #[test]
    fn test_fetch_all_product_table() {
        let param = RequestParam::default();
        match establish_pg_connection() {
            Ok(mut conn) => match ProductMapper::get_all(&mut conn, &param) {
                Ok(data) => {
                    assert!(!data.data().is_empty(), "Product table should not be empty");
                }
                Err(e) => {
                    panic!("Error fetching all products: {:?}", e);
                }
            },
            Err(e) => {
                panic!("Error establishing connection: {:?}", e);
            }
        }
    }

    #[test]
    fn test_get_by_id() {
        match establish_pg_connection() {
            Ok(mut conn) => {
                let pid = 2; // 假设ID为1的记录存在
                match ProductMapper::get_by_id(&mut conn, pid) {
                    Ok(data) => {
                        assert_eq!(
                            data.product_id, pid,
                            "Fetched product ID should match requested ID"
                        );
                    }
                    Err(diesel::result::Error::NotFound) => {
                        panic!("Product with ID {} not found", pid);
                    }
                    Err(e) => {
                        panic!("Error fetching product by ID: {:?}", e);
                    }
                }
            }
            Err(e) => {
                panic!("Error establishing connection: {:?}", e);
            }
        }
    }

    #[test]
    fn test_add_single() {
        match establish_pg_connection() {
            Ok(mut conn) => {
                let new_product = PostProduct {
                    name: "Test Product".to_string(),
                    description: Some("This is a test product".to_string()),
                    sku: "TEST123".to_string(),
                    image: Some("test_image.jpg".to_string()),
                    price: Some(100.0),
                    discount_price: Some(90.0),
                    is_discounted: Some(true),
                    is_valid: Some(true),
                    inventory: Some(10),
                    is_in_stock: Some(true),
                    created_at: Some(get_e8_time()),
                    updated_at: Some(get_e8_time()),
                    supplier_id: Some(1),
                    weight: Some(1.5),
                    dimensions: Some("10x10x10".to_string()),
                    status: Some("available".to_string()),
                    public: Some(true),
                };
                match ProductMapper::add_single(&mut conn, &new_product) {
                    Ok(data) => {
                        assert_eq!(
                            data.name, "Test Product",
                            "Name should match the added product"
                        );
                    }
                    Err(e) => {
                        panic!("Error adding product: {:?}", e);
                    }
                }
            }
            Err(e) => {
                panic!("Error establishing connection: {:?}", e);
            }
        }
    }

    #[test]
    fn test_update_by_id() {
        match establish_pg_connection() {
            Ok(mut conn) => {
                let pid = 2; // 假设ID为1的记录存在
                let updated_product = PatchProduct {
                    user_id: Some(2),
                    name: "Updated Product".to_string(),
                    description: Some("This is an updated product description".to_string()),
                    sku: "UPDATED123".to_string(),
                    image: Some("updated_image.jpg".to_string()),
                    price: Some(150.0),
                    discount_price: Some(140.0),
                    is_discounted: Some(true),
                    is_valid: Some(true),
                    inventory: Some(20),
                    is_in_stock: Some(true),
                    updated_at: Some(get_e8_time()),
                    created_at: Some(get_e8_time()),
                    supplier_id: Some(2),
                    weight: Some(2.0),
                    dimensions: Some("20x20x20".to_string()),
                    status: Some("updated".to_string()),
                    public: Some(false),
                };
                match ProductMapper::update_by_id(&mut conn, pid, &updated_product) {
                    Ok(data) => {
                        assert_eq!(data.name, "Updated Product", "Name should be updated");
                    }
                    Err(diesel::result::Error::NotFound) => {
                        panic!("Product with ID {} not found for update", pid);
                    }
                    Err(e) => {
                        panic!("Error updating product: {:?}", e);
                    }
                }
            }
            Err(e) => {
                panic!("Error establishing connection: {:?}", e);
            }
        }
    }

    #[test]
    fn test_delete_by_id() {
        match establish_pg_connection() {
            Ok(mut conn) => {
                let pid = 1; // 假设ID为1的记录存在
                match ProductMapper::delete_by_id(&mut conn, pid) {
                    Ok(data) => {
                        assert_eq!(
                            data.product_id, pid,
                            "Deleted product ID should match requested ID"
                        );
                    }
                    Err(diesel::result::Error::NotFound) => {
                        panic!("Product with ID {} not found for deletion", pid);
                    }
                    Err(e) => {
                        panic!("Error deleting product: {:?}", e);
                    }
                }
            }
            Err(e) => {
                panic!("Error establishing connection: {:?}", e);
            }
        }
    }

    #[test]
    fn test_filter() {
        match establish_pg_connection() {
            Ok(mut conn) => {
                let param = RequestParam {
                    pagination: PaginationParam::demo(),
                    filter: Some(ProductFilter {
                        product_id: Some(1),
                        user_id: None,
                        name: None,
                        description: None,
                        sku: None,
                        image: None,
                        price_min: None,
                        price_max: None,
                        discount_price_min: None,
                        discount_price_max: None,
                        is_discounted: None,
                        is_valid: None,
                        inventory_min: None,
                        inventory_max: None,
                        is_in_stock: None,
                        created_at_min: None,
                        created_at_max: None,
                        updated_at_min: None,
                        updated_at_max: None,
                        supplier_id: None,
                        weight_min: None,
                        weight_max: None,
                        dimensions: None,
                        status: None,
                        public: None,
                    }),
                };
                match ProductMapper::filter(&mut conn, &param) {
                    Ok(data) => {
                        assert!(
                            data.data().iter().all(|item| item.product_id == 1),
                            "Filtered result should have product_id 1"
                        );
                    }
                    Err(e) => {
                        panic!("Error filtering products: {:?}", e);
                    }
                }
            }
            Err(e) => {
                panic!("Error establishing connection: {:?}", e);
            }
        }
    }
}
