use std::error::Error;

use rocket::State;

use crab_rocket_schema::DbPool;
use obj_traits::controller::controller_crud::{
    controller_add_single, controller_delete_by_id, controller_filter, controller_get_all,
    controller_get_by_id, controller_update_by_id, ControllerCRUD,
};
use obj_traits::request::request_param::RequestParam;
use obj_traits::response::api_response::ApiResponse;
use obj_traits::response::data::Data;

use crate::models::shipment::{PatchShipment, PostShipment, Shipment};
use crate::models::shipment_filter::ShipmentFilter;
use crate::services::shipment_service::ShipmentService;

pub struct ShipmentController {}

impl ControllerCRUD for ShipmentController {
    type Item = Shipment;
    type PostItem = PostShipment;
    type PatchItem = PatchShipment;
    type Param = RequestParam<ShipmentFilter>;
    fn get_all(
        pool: &State<DbPool>,
        param: &RequestParam<ShipmentFilter>,
    ) -> Result<ApiResponse<Data<Vec<Self::Item>>>, Box<dyn Error>> {
        controller_get_all::<Self::Item, ShipmentService, ShipmentFilter>(pool, param)
    }

    fn get_by_id(
        pool: &State<DbPool>,
        pid: i32,
    ) -> Result<ApiResponse<Self::Item>, Box<dyn Error>> {
        controller_get_by_id::<Self::Item, ShipmentService>(pool, pid)
    }

    fn add_single(
        pool: &State<DbPool>,
        obj: &mut PostShipment,
    ) -> Result<ApiResponse<Self::Item>, Box<dyn Error>> {
        controller_add_single::<Self::Item, ShipmentService, PostShipment>(pool, obj)
    }

    fn delete_by_id(
        pool: &State<DbPool>,
        pid: i32,
    ) -> Result<ApiResponse<Self::Item>, Box<dyn Error>> {
        controller_delete_by_id::<Self::Item, ShipmentService>(pool, pid)
    }

    fn update_by_id(
        pool: &State<DbPool>,
        pid: i32,
        obj: &PatchShipment,
    ) -> Result<ApiResponse<Self::Item>, Box<dyn Error>> {
        controller_update_by_id::<Self::Item, ShipmentService, PatchShipment>(pool, pid, obj)
    }
    fn filter(
        pool: &State<DbPool>,
        param: &RequestParam<ShipmentFilter>,
    ) -> Result<ApiResponse<Data<Vec<Self::Item>>>, Box<dyn std::error::Error>> {
        controller_filter::<Self::Item, ShipmentService, ShipmentFilter>(pool, param)
    }
}
