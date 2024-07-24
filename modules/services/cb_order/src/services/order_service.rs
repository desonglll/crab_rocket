use obj_traits::service::service_crud::ServiceCRUD;

use crate::mappers::order_mapper::OrderMapper;
use crate::models::order::{Order, PatchOrder, PostOrder};
use crate::models::order_filter::OrderFilter;

pub struct OrderService {}

impl ServiceCRUD for OrderService {
    type Item = Order;
    type PostItem = PostOrder;
    type PatchItem = PatchOrder;
    type Filter = OrderFilter;
    type Mapper = OrderMapper;
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
            Ok(result) => println!("{result}"),
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
