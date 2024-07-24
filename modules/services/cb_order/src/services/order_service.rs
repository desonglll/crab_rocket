use std::error::Error;

use rocket::State;

use crab_rocket_schema::DbPool;
use obj_traits::request::request_param::RequestParam;
use obj_traits::response::data::Data;
use obj_traits::service::service_crud::{
    service_add_single, service_delete_by_id, service_filter, service_get_all, service_get_by_id,
    service_update_by_id, ServiceCRUD,
};

use crate::mappers::order_mapper::OrderMapper;
use crate::models::order::{Order, PatchOrder, PostOrder};
use crate::models::order_filter::OrderFilter;

pub struct OrderService {}

impl ServiceCRUD for OrderService {
    type Item = Order;
    type PostItem = PostOrder;
    type PatchItem = PatchOrder;
    type Param = RequestParam<OrderFilter>;
    fn get_all(
        pool: &State<DbPool>,
        param: &RequestParam<OrderFilter>,
    ) -> Result<Data<Vec<Order>>, Box<dyn Error>> {
        service_get_all::<Order, OrderMapper, OrderFilter>(pool, param)
    }
    fn get_by_id(pool: &State<DbPool>, pid: i32) -> Result<Order, Box<dyn Error>> {
        service_get_by_id::<Order, OrderMapper>(pool, pid)
    }

    fn add_single(pool: &State<DbPool>, obj: &PostOrder) -> Result<Order, Box<dyn Error>> {
        service_add_single::<Order, OrderMapper, PostOrder>(pool, obj)
    }

    fn delete_by_id(pool: &State<DbPool>, pid: i32) -> Result<Order, Box<dyn Error>> {
        service_delete_by_id::<Order, OrderMapper>(pool, pid)
    }

    fn update_by_id(
        pool: &State<DbPool>,
        pid: i32,
        obj: &PatchOrder,
    ) -> Result<Order, Box<dyn Error>> {
        service_update_by_id::<Order, OrderMapper, PatchOrder>(pool, pid, obj)
    }
    fn filter(
        pool: &State<DbPool>,
        param: &RequestParam<OrderFilter>,
    ) -> Result<Data<Vec<Order>>, Box<dyn std::error::Error>> {
        service_filter::<Order, OrderMapper, OrderFilter>(pool, param)
    }
}

#[cfg(test)]
mod test {
    use rocket::State;

    use crab_rocket_schema::{DbPool, establish_pool};
    use obj_traits::request::pagination_request_param::{PaginationParam, PaginationParamTrait};
    use obj_traits::request::request_param::RequestParam;
    use obj_traits::service::service_crud::ServiceCRUD;

    use crate::services::order_service::OrderService;

    #[test]
    fn test_insert_single_order() {
        use crate::models::order::PostOrder;
        let order = PostOrder::demo();
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match OrderService::add_single(pool, &order) {
            Ok(result) => println!("{result:?}"),
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn test_get_all_orders() {
        let param = RequestParam::new(Some(PaginationParam::demo()), None);
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match OrderService::get_all(pool, &param) {
            Ok(res) => println!("{res}"),
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn test_get_order_by_id() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match OrderService::get_by_id(pool, 1) {
            Ok(res) => println!("{res:?}"),
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn test_delete_order_by_id() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match OrderService::delete_by_id(pool, 2) {
            Ok(res) => println!("{res:?}"),
            Err(e) => println!("{e:?}"),
        }
    }
}
