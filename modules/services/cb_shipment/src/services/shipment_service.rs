use crate::mappers::shipment_mapper::ShipmentMapper;
use crate::models::shipment::{PatchShipment, PostShipment, Shipment};
use crate::models::shipment_filter::ShipmentFilter;
use obj_traits::request::request_param::RequestParam;
use obj_traits::response::data::Data;
use obj_traits::service::service_crud::{
    service_add_single, service_delete_by_id, service_filter, service_get_all, service_get_by_id,
    service_update_by_id, ServiceCRUD,
};
use std::error::Error;

pub struct ShipmentService {}

impl ServiceCRUD for ShipmentService {
    type Item = Shipment;
    type PostItem = PostShipment;
    type PatchItem = PatchShipment;
    type Param = RequestParam<ShipmentFilter>;
    fn get_all(
        param: &RequestParam<ShipmentFilter>,
    ) -> Result<Data<Vec<Shipment>>, Box<dyn Error>> {
        service_get_all::<Shipment, ShipmentMapper, ShipmentFilter>(param)
    }
    fn get_by_id(pid: i32) -> Result<Shipment, Box<dyn Error>> {
        service_get_by_id::<Shipment, ShipmentMapper>(pid)
    }

    fn add_single(obj: &PostShipment) -> Result<Shipment, Box<dyn Error>> {
        service_add_single::<Shipment, ShipmentMapper, PostShipment>(obj)
    }

    fn delete_by_id(pid: i32) -> Result<Shipment, Box<dyn Error>> {
        service_delete_by_id::<Shipment, ShipmentMapper>(pid)
    }

    fn update_by_id(pid: i32, obj: &PatchShipment) -> Result<Shipment, Box<dyn Error>> {
        service_update_by_id::<Shipment, ShipmentMapper, PatchShipment>(pid, obj)
    }
    fn filter(
        param: &RequestParam<ShipmentFilter>,
    ) -> Result<Data<Vec<Shipment>>, Box<dyn std::error::Error>> {
        service_filter::<Shipment, ShipmentMapper, ShipmentFilter>(param)
    }
}

#[cfg(test)]
mod test {
    use crate::services::shipment_service::ShipmentService;
    use obj_traits::request::request_param::RequestParam;
    use obj_traits::service::service_crud::ServiceCRUD;

    #[test]
    fn test_insert_single_shipment() {
        use crate::models::shipment::PostShipment;
        let shipment = PostShipment::default();
        match ShipmentService::add_single(&shipment) {
            Ok(result) => println!("{result:?}"),
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn test_get_all_shipments() {
        let param = RequestParam::new(None, None);
        match ShipmentService::get_all(&param) {
            Ok(res) => println!("{res}"),
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn test_get_shipment_by_id() {
        match ShipmentService::get_by_id(1) {
            Ok(res) => println!("{res:?}"),
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn test_delete_shipment_by_id() {
        match ShipmentService::delete_by_id(2) {
            Ok(res) => println!("{res:?}"),
            Err(e) => println!("{e:?}"),
        }
    }
}
