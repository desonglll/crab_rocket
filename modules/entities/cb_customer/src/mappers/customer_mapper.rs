use obj_traits::{
    mapper::mapper_crud::MapperCRUD,
    request::{pagination_request_param::Pagination, request_param::RequestParam},
    response::data::Data,
};
use rocket::State;

use crate::models::{
    customer::{Customer, PatchCustomer, PostCustomer},
    customer_filter::CustomerFilter,
};
use crab_rocket_schema::{establish_pg_connection, schema::customer_table::dsl, DbPool};
use diesel::prelude::*;

pub struct CustomerMapper {}

impl MapperCRUD for CustomerMapper {
    type Item = Customer;
    type PostItem = PostCustomer;
    type PatchItem = PatchCustomer;
    type Filter = CustomerFilter;
    fn get_all(
        pool: &State<DbPool>,
        param: &RequestParam<Self::Item, Self::Filter>,
    ) -> Result<Data<Vec<Customer>>, diesel::result::Error> {
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
        let total_count = dsl::customer_table.count().get_result::<i64>(&mut conn)? as i32;
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
        // let custom: Vec<Customer> =
        //     diesel::sql_query("SELECT * FROM customer_table").load::<Customer>(conn)?;

        // 分页查询
        let data = dsl::customer_table
            .order(dsl::customer_id.desc())
            .limit(per_page as i64)
            .offset(((page - 1) * per_page) as i64)
            .load::<Customer>(&mut conn)?;
        let resp_body = Data::new(data, Some(pagination));
        Ok(resp_body)
    }
    fn get_by_id(pool: &State<DbPool>, pid: i32) -> Result<Data<Customer>, diesel::result::Error> {
        let mut conn = establish_pg_connection(pool).expect("msg");
        // 配合 use crate::schema::customer_table::dsl::*;
        let data = dsl::customer_table.filter(dsl::customer_id.eq(pid)).first(&mut conn)?;
        Ok(Data::new(data, None))
    }

    fn add_single(
        pool: &State<DbPool>,
        obj: &PostCustomer,
    ) -> Result<Data<Customer>, diesel::result::Error> {
        let mut conn = establish_pg_connection(pool).expect("msg");
        let data = diesel::insert_into(dsl::customer_table)
            .values(obj)
            .returning(Customer::as_returning())
            .get_result(&mut conn)?;
        Ok(Data::new(data, None))
    }

    fn delete_by_id(
        pool: &State<DbPool>,
        pid: i32,
    ) -> Result<Data<Customer>, diesel::result::Error> {
        let mut conn = establish_pg_connection(pool).expect("msg");
        let data = diesel::delete(dsl::customer_table.filter(dsl::customer_id.eq(pid)))
            .get_result(&mut conn)?;
        Ok(Data::new(data, None))
    }

    fn update_by_id(
        pool: &State<DbPool>,
        pid: i32,
        obj: &PatchCustomer,
    ) -> Result<Data<Customer>, diesel::result::Error> {
        let mut conn = establish_pg_connection(pool).expect("msg");
        let data = diesel::update(dsl::customer_table.filter(dsl::customer_id.eq(pid)))
            .set((
                dsl::name.eq(&obj.name),
                dsl::email.eq(&obj.email),
                dsl::phone.eq(&obj.phone),
                dsl::address.eq(&obj.address),
            ))
            .get_result(&mut conn)?;
        Ok(Data::new(data, None))
    }
    fn filter(
        pool: &State<DbPool>,
        param: &RequestParam<Self::Item, Self::Filter>,
    ) -> Result<Data<Vec<Customer>>, diesel::result::Error> {
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
        let total_count = dsl::customer_table.count().get_result::<i64>(&mut conn)? as i32;
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
        let mut query = dsl::customer_table.into_boxed();
        let filter = &param.filter;
        if let Some(f) = filter {
            // 篩選條件
            if let Some(customer_id) = f.customer_id {
                query = query.filter(dsl::customer_id.eq(customer_id));
            }

            if let Some(name) = &f.name {
                query = query.filter(dsl::name.like(format!("%{}%", name)));
            }

            if let Some(email) = &f.email {
                query = query.filter(dsl::email.like(format!("%{}%", email)));
            }

            if let Some(phone) = &f.phone {
                query = query.filter(dsl::phone.like(format!("%{}%", phone)));
            }

            if let Some(address) = &f.address {
                query = query.filter(dsl::address.like(format!("%{}%", address)));
            }
        }
        let data = query.load::<Customer>(&mut conn)?;
        let body = Data::new(data, Some(pagination));
        Ok(body)
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::models::{
        customer::{PatchCustomer, PostCustomer},
        customer_filter::CustomerFilter,
    };
    use crab_rocket_schema::{establish_pool, DbPool};
    use rocket::State;
    #[test]
    fn test_get_all() {
        let filter = CustomerFilter {
            customer_id: None,
            name: Some("John".to_string()),
            email: None,
            phone: None,
            address: None,
        };
        let param = RequestParam::new(None, Some(filter));

        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match CustomerMapper::get_all(pool, &param) {
            Ok(data) => {
                assert!(!data.data.is_empty(), "Customer table should not be empty");
                println!("{:#?}", data);
            }
            Err(e) => {
                panic!("Error fetching all customers: {:?}", e);
            }
        }
    }

    #[test]
    fn test_get_by_id() {
        let test_id = 1; // 请确保你的数据库中有这个ID的数据
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match CustomerMapper::get_by_id(pool, test_id) {
            Ok(customer) => {
                assert_eq!(
                    customer.data.customer_id, test_id,
                    "Fetched customer ID should match the requested ID"
                );
            }
            Err(diesel::result::Error::NotFound) => {
                panic!("Customer with ID {} not found", test_id);
            }
            Err(e) => {
                panic!("Error fetching customer by ID: {:?}", e);
            }
        }
    }

    #[test]
    fn test_add_single() {
        let new_customer = PostCustomer {
            name: "John Doe".to_string(),
            email: "john.doe@example.com".to_string(),
            phone: Some("1234567890".to_string()),
            address: Some("123 Test St".to_string()),
        };

        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match CustomerMapper::add_single(pool, &new_customer) {
            Ok(customer) => {
                assert_eq!(customer.data.name, "John Doe", "Name should match the added customer");
            }
            Err(e) => {
                panic!("Error adding customer: {:?}", e);
            }
        }
    }

    #[test]
    fn test_delete_by_id() {
        let test_id = 3; // 请确保你的数据库中有这个ID的数据
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match CustomerMapper::delete_by_id(pool, test_id) {
            Ok(customer) => {
                assert_eq!(
                    customer.data.customer_id, test_id,
                    "Deleted customer ID should match the requested ID"
                );
            }
            Err(diesel::result::Error::NotFound) => {
                panic!("Customer with ID {} not found for deletion", test_id);
            }
            Err(e) => {
                panic!("Error deleting customer: {:?}", e);
            }
        }
    }

    #[test]
    fn test_update_by_id() {
        let test_id = 1; // 请确保你的数据库中有这个ID的数据
        let updated_customer = PatchCustomer {
            name: "Jane Doe".to_string(),
            email: "jane.doe@example.com".to_string(),
            phone: Some("0987654321".to_string()),
            address: Some("456 Test Ave".to_string()),
        };

        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match CustomerMapper::update_by_id(pool, test_id, &updated_customer) {
            Ok(customer) => {
                assert_eq!(customer.data.name, "Jane Doe", "Name should be updated");
            }
            Err(diesel::result::Error::NotFound) => {
                panic!("Customer with ID {} not found for update", test_id);
            }
            Err(e) => {
                panic!("Error updating customer: {:?}", e);
            }
        }
    }

    #[test]
    fn test_filter() {
        let filter = CustomerFilter {
            customer_id: None,
            name: Some("John".to_string()),
            email: None,
            phone: None,
            address: None,
        };
        let param = RequestParam::new(None, Some(filter));

        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match CustomerMapper::filter(pool, &param) {
            Ok(data) => {
                assert!(
                    data.data.iter().all(|c| c.name.contains("John")),
                    "Filtered result should contain customers with 'John' in the name"
                );
            }
            Err(e) => {
                panic!("Error filtering customers: {:?}", e);
            }
        }
    }
}
