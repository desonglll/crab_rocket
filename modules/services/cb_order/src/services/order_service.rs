use crate::mappers::order_mapper::OrderMapper;
use crate::models::order::{Order, PatchOrder, PostOrder};
use crate::models::order_filter::OrderFilter;
use obj_traits::request::pagination_request_param::PaginationParam;
use obj_traits::request::request_param::RequestParam;
use obj_traits::response::data::Data;
use obj_traits::service::service_crud::{
    service_add_single, service_delete_by_id, service_filter, service_get_all, service_get_by_id,
    service_update_by_id, ServiceCRUD,
};
use std::error::Error;

pub struct OrderService {}

impl ServiceCRUD for OrderService {
    type Item = Order;
    type PostItem = PostOrder;
    type PatchItem = PatchOrder;
    type Param = RequestParam<PaginationParam, OrderFilter>;
    fn get_all(
        param: &RequestParam<PaginationParam, OrderFilter>,
    ) -> Result<Data<Vec<Order>>, Box<dyn Error>> {
        service_get_all::<Order, OrderMapper, OrderFilter>(param)
    }
    fn get_by_id(pid: i32) -> Result<Order, Box<dyn Error>> {
        service_get_by_id::<Order, OrderMapper>(pid)
    }

    fn add_single(obj: &PostOrder) -> Result<Order, Box<dyn Error>> {
        service_add_single::<Order, OrderMapper, PostOrder>(obj)
    }

    fn delete_by_id(pid: i32) -> Result<Order, Box<dyn Error>> {
        service_delete_by_id::<Order, OrderMapper>(pid)
    }

    fn update_by_id(pid: i32, obj: &PatchOrder) -> Result<Order, Box<dyn Error>> {
        service_update_by_id::<Order, OrderMapper, PatchOrder>(pid, obj)
    }
    fn filter(
        param: &RequestParam<PaginationParam, OrderFilter>,
    ) -> Result<Data<Vec<Order>>, Box<dyn std::error::Error>> {
        service_filter::<Order, OrderMapper, OrderFilter>(param)
    }
}

#[cfg(test)]
mod test {
    use crate::services::order_service::OrderService;
    use obj_traits::request::pagination_request_param::{PaginationParam, PaginationParamTrait};
    use obj_traits::request::request_param::RequestParam;
    use obj_traits::service::service_crud::ServiceCRUD;

    #[test]
    fn test_insert_single_order() {
        use crate::models::order::PostOrder;
        let order = PostOrder::default();
        match OrderService::add_single(&order) {
            Ok(result) => println!("{result:?}"),
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn test_get_all_orders() {
        let param = RequestParam::new(PaginationParam::demo(), None);
        match OrderService::get_all(&param) {
            Ok(res) => println!("{res}"),
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn test_get_order_by_id() {
        match OrderService::get_by_id(1) {
            Ok(res) => println!("{res:?}"),
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn test_delete_order_by_id() {
        match OrderService::delete_by_id(2) {
            Ok(res) => println!("{res:?}"),
            Err(e) => println!("{e:?}"),
        }
    }
}
