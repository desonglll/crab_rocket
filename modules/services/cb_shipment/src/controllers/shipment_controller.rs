use obj_traits::controller::controller_crud::ControllerCRUD;

use crate::models::shipment::{PatchShipment, PostShipment, Shipment};
use crate::models::shipment_filter::ShipmentFilter;
use crate::services::shipment_service::ShipmentService;

pub struct ShipmentController {}

impl ControllerCRUD for ShipmentController {
    type Item = Shipment;
    type PostItem = PostShipment;
    type PatchItem = PatchShipment;
    type Filter = ShipmentFilter;
    type Service = ShipmentService;
}
