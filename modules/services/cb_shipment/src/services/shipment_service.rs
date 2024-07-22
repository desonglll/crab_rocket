use crate::mappers::shipment_mapper::ShipmentMapper;
use crate::models::shipment::{PatchShipment, PostShipment, Shipment};
use crate::models::shipment_filter::ShipmentFilter;
use crab_rocket_schema::DbPool;
use obj_traits::request::request_param::RequestParam;
use obj_traits::response::data::Data;
use obj_traits::service::service_crud::{
    service_add_single, service_delete_by_id, service_filter, service_get_all, service_get_by_id,
    service_update_by_id, ServiceCRUD,
};
use rocket::State;
use std::error::Error;

pub struct ShipmentService {}

impl ServiceCRUD for ShipmentService {
    type Item = Shipment;
    type PostItem = PostShipment;
    type PatchItem = PatchShipment;
    type Param = RequestParam<ShipmentFilter>;
    fn get_all(
        pool: &State<DbPool>,
        param: &RequestParam<ShipmentFilter>,
    ) -> Result<Data<Vec<Shipment>>, Box<dyn Error>> {
        service_get_all::<Shipment, ShipmentMapper, ShipmentFilter>(pool, param)
    }
    fn get_by_id(pool: &State<DbPool>, pid: i32) -> Result<Shipment, Box<dyn Error>> {
        service_get_by_id::<Shipment, ShipmentMapper>(pool, pid)
    }

    fn add_single(pool: &State<DbPool>, obj: &PostShipment) -> Result<Shipment, Box<dyn Error>> {
        service_add_single::<Shipment, ShipmentMapper, PostShipment>(pool, obj)
    }

    fn delete_by_id(pool: &State<DbPool>, pid: i32) -> Result<Shipment, Box<dyn Error>> {
        service_delete_by_id::<Shipment, ShipmentMapper>(pool, pid)
    }

    fn update_by_id(
        pool: &State<DbPool>,
        pid: i32,
        obj: &PatchShipment,
    ) -> Result<Shipment, Box<dyn Error>> {
        service_update_by_id::<Shipment, ShipmentMapper, PatchShipment>(pool, pid, obj)
    }
    fn filter(
        pool: &State<DbPool>,
        param: &RequestParam<ShipmentFilter>,
    ) -> Result<Data<Vec<Shipment>>, Box<dyn std::error::Error>> {
        service_filter::<Shipment, ShipmentMapper, ShipmentFilter>(pool, param)
    }
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
            Ok(result) => println!("{result:?}"),
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
