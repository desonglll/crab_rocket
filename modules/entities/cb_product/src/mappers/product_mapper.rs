use diesel::prelude::*;
use rocket::State;

use crab_rocket_schema::{DbPool, establish_pg_connection};
//配合下面的 `products.filter()`
use crab_rocket_schema::schema::product_table::{self};
use crab_rocket_schema::schema::product_table::dsl;
use crab_rocket_utils::time::get_e8_time;
use obj_traits::mapper::mapper_crud::MapperCRUD;
use obj_traits::request::pagination_request_param::Pagination;
use obj_traits::request::request_param::RequestParam;
use obj_traits::response::data::Data;

use crate::models::product::{PatchProduct, PostProduct, Product};
use crate::models::product_filter::ProductFilter;

pub struct ProductMapper {}

impl MapperCRUD for ProductMapper {
    type Item = Product;
    type PostItem = PostProduct;
    type PatchItem = PatchProduct;
    type Filter = ProductFilter;
    fn get_all(
        pool: &State<DbPool>,
        param: &RequestParam<Self::Item, Self::Filter>,
    ) -> Result<Data<Vec<Product>>, diesel::result::Error> {
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
        let total_count = dsl::product_table.count().get_result::<i64>(&mut conn)? as i32;
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

        // 分页查询
        let data = dsl::product_table
            .order(dsl::updated_at.desc())
            .limit(per_page as i64)
            .offset(((page - 1) * per_page) as i64)
            .load::<Product>(&mut conn)?;
        let body = Data::new(data, Some(pagination));
        println!("Getting products successfully.");
        Ok(body)
    }

    fn get_by_id(pool: &State<DbPool>, pid: i32) -> Result<Data<Product>, diesel::result::Error> {
        let mut conn = establish_pg_connection(pool).expect("msg");
        let data = dsl::product_table.filter(product_table::product_id.eq(pid)).first(&mut conn)?;
        Ok(Data::new(data, None))
    }

    fn add_single(
        pool: &State<DbPool>,
        obj: &PostProduct,
    ) -> Result<Data<Product>, diesel::result::Error> {
        let mut conn = establish_pg_connection(pool).expect("msg");
        let data = diesel::insert_into(product_table::table)
            .values(obj)
            .returning(Product::as_returning())
            .get_result(&mut conn)?;
        Ok(Data::new(data, None))
    }

    fn delete_by_id(
        pool: &State<DbPool>,
        pid: i32,
    ) -> Result<Data<Product>, diesel::result::Error> {
        let mut conn = establish_pg_connection(pool).expect("msg");
        let data = diesel::delete(dsl::product_table.filter(product_table::product_id.eq(pid)))
            .get_result(&mut conn)?;
        Ok(Data::new(data, None))
    }

    fn update_by_id(
        pool: &State<DbPool>,
        pid: i32,
        obj: &PatchProduct,
    ) -> Result<Data<Product>, diesel::result::Error> {
        let mut conn = establish_pg_connection(pool).expect("msg");
        let data = diesel::update(dsl::product_table.filter(dsl::product_id.eq(pid)))
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
            .get_result(&mut conn)?;
        Ok(Data::new(data, None))
    }
    fn filter(
        pool: &State<DbPool>,
        param: &RequestParam<Self::Item, Self::Filter>,
    ) -> Result<Data<Vec<Product>>, diesel::result::Error> {
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
        let total_count = dsl::product_table.count().get_result::<i64>(&mut conn)? as i32;
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

        // 分页查询
        query = query
            .order(dsl::created_at.desc())
            .limit(per_page as i64)
            .offset(((page - 1) * per_page) as i64);

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
        let data = query.load::<Product>(&mut conn)?;
        let body = Data::new(data, Some(pagination));
        Ok(body)
    }
}

#[cfg(test)]
mod tests {
    use rocket::State;

    use crab_rocket_schema::{DbPool, establish_pool};
    use obj_traits::request::pagination_request_param::{PaginationParam, PaginationParamTrait};
    use obj_traits::request::request_param::RequestParam;

    use super::*;

    #[test]
    fn test_get_all_products() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);

        let param = RequestParam::new(Some(PaginationParam::demo()), None);

        let result = ProductMapper::get_all(pool, &param);
        assert!(result.is_ok());
        let data = result.unwrap();
        println!("{:#?}", data);
        assert!(data.data.len() > 0); // Assuming there are products in the database
    }

    #[test]
    fn test_add_single_product() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);

        let product = PostProduct {
            user_id: Some(4),
            name: "Test Product".to_string(),
            description: Some("This is a test product".to_string()),
            sku: "TEST123a".to_string(),
            image: Some("test_image.jpg".to_string()),
            price: Some(100.0),
            discount_price: Some(90.0),
            is_discounted: Some(true),
            is_valid: Some(true),
            inventory: Some(10),
            is_in_stock: Some(true),
            created_at: Some(get_e8_time()),
            updated_at: Some(get_e8_time()),
            supplier_id: Some(4),
            weight: Some(1.5),
            dimensions: Some("10x10x10".to_string()),
            status: Some("available".to_string()),
            public: Some(true),
        };

        let result = ProductMapper::add_single(pool, &product);
        assert!(result.is_ok());
        let inserted_product = result.unwrap();
        assert_eq!(inserted_product.data.name, "Test Product");
    }

    #[test]
    fn test_get_product_by_id() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);

        // Assuming a product with ID 1 exists in the database
        let product_id = 1;
        let result = ProductMapper::get_by_id(pool, product_id);
        assert!(result.is_ok());
        let product = result.unwrap();
        assert_eq!(product.data.product_id, product_id);
    }

    #[test]
    fn test_update_product_by_id() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);

        let patch_product = PatchProduct {
            user_id: Some(4),
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
            supplier_id: Some(4),
            weight: Some(2.0),
            dimensions: Some("20x20x20".to_string()),
            status: Some("updated".to_string()),
            public: Some(false),
        };

        // Assuming a product with ID 1 exists in the database
        let product_id = 1;
        let result = ProductMapper::update_by_id(pool, product_id, &patch_product);
        println!("{:#?}", result);
        assert!(result.is_ok());
        let updated_product = result.unwrap();
        assert_eq!(updated_product.data.name, "Updated Product");
    }

    #[test]
    fn test_delete_product_by_id() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);

        // Add a product to delete
        let product = PostProduct {
            user_id: Some(1),
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
        let inserted_product =
            ProductMapper::add_single(pool, &product).expect("Failed to insert product");
        let product_id = inserted_product.data.product_id;

        // Delete the product
        let result = ProductMapper::delete_by_id(pool, product_id);
        assert!(result.is_ok());
        let deleted_product = result.unwrap();
        assert_eq!(deleted_product.data.product_id, product_id);

        // Verify deletion
        let result = ProductMapper::get_by_id(pool, product_id);
        assert!(result.is_err());
    }
}
