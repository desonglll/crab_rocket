use crate::mappers::shipment_mapper::ShipmentMapper;
use crate::models::shipment::{PatchShipment, PostShipment, Shipment};
use crate::models::shipment_filter::ShipmentFilter;

use obj_traits::service::service_crud::ServiceCRUD;

pub struct ShipmentService {}

impl ServiceCRUD for ShipmentService {
    type Item = Shipment;
    type PostItem = PostShipment;
    type PatchItem = PatchShipment;
    type Filter = ShipmentFilter;
    type Mapper = ShipmentMapper;
}

#[cfg(test)]
mod test {
    use crate::services::shipment_service::ShipmentService;
    use crab_rocket_schema::{establish_pool, DbPool};
    use obj_traits::request::pagination_request_param::{PaginationParam, PaginationParamTrait};
    use obj_traits::request::request_param::RequestParam;
    use obj_traits::service::service_crud::ServiceCRUD;
    use rocket::State;

    #[test]
    fn test_insert_single_shipment() {
        use crate::models::shipment::PostShipment;
        let shipment = PostShipment::demo();
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match ShipmentService::add_single(pool, &shipment) {
            Ok(result) => println!("{result}"),
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn test_get_all_shipments() {
        let param = RequestParam::new(Some(PaginationParam::demo()), None);
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match ShipmentService::get_all(pool, &param) {
            Ok(res) => println!("{res}"),
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn test_get_shipment_by_id() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match ShipmentService::get_by_id(pool, 1) {
            Ok(res) => println!("{res:?}"),
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn test_delete_shipment_by_id() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match ShipmentService::delete_by_id(pool, 2) {
            Ok(res) => println!("{res:?}"),
            Err(e) => println!("{e:?}"),
        }
    }
}
