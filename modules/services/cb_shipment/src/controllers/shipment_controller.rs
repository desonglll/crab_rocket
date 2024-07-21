use crate::models::shipment::{PatchShipment, PostShipment, Shipment};
use crate::models::shipment_filter::ShipmentFilter;
use crate::services::shipment_service::ShipmentService;
use obj_traits::controller::controller_crud::{
    controller_add_single, controller_delete_by_id, controller_filter, controller_get_all,
    controller_get_by_id, controller_update_by_id, ControllerCRUD,
};
use obj_traits::request::pagination_request_param::PaginationParam;
use obj_traits::request::request_param::RequestParam;
use obj_traits::response::api_response::ApiResponse;
use obj_traits::response::data::Data;
use std::error::Error;

pub struct ShipmentController {}

impl ControllerCRUD for ShipmentController {
    type Item = Shipment;
    type PostItem = PostShipment;
    type PatchItem = PatchShipment;
    type Param = RequestParam<PaginationParam, ShipmentFilter>;
    fn get_all(
        param: &RequestParam<PaginationParam, ShipmentFilter>,
    ) -> Result<ApiResponse<Data<Vec<Shipment>>>, Box<dyn Error>> {
        controller_get_all::<Shipment, ShipmentService, ShipmentFilter>(param)
    }

    fn get_by_id(pid: i32) -> Result<ApiResponse<Shipment>, Box<dyn Error>> {
        controller_get_by_id::<Shipment, ShipmentService>(pid)
    }

    fn add_single(obj: &mut PostShipment) -> Result<ApiResponse<Shipment>, Box<dyn Error>> {
        controller_add_single::<Shipment, ShipmentService, PostShipment>(obj)
    }

    fn delete_by_id(pid: i32) -> Result<ApiResponse<Shipment>, Box<dyn Error>> {
        controller_delete_by_id::<Shipment, ShipmentService>(pid)
    }

    fn update_by_id(
        pid: i32,
        obj: &PatchShipment,
    ) -> Result<ApiResponse<Shipment>, Box<dyn Error>> {
        controller_update_by_id::<Shipment, ShipmentService, PatchShipment>(pid, obj)
    }
    fn filter(
        param: &RequestParam<PaginationParam, ShipmentFilter>,
    ) -> Result<ApiResponse<Data<Vec<Shipment>>>, Box<dyn std::error::Error>> {
        controller_filter::<Shipment, ShipmentService, ShipmentFilter>(param)
    }
}
