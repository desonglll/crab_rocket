use crab_rocket_inventory::routes::inventory_route::*;
use crab_rocket_order::routes::order_route::*;
use crab_rocket_shipment::routes::shipment_route::*;
use rocket::{routes, Route};

pub fn services_routes() -> Vec<Route> {
    routes![
        //inventory routes
        get_inventorys,
        filter_inventorys,
        get_inventory_by_id,
        insert_single_inventory,
        delete_inventory_by_id,
        update_inventory_by_id,
        options_inventory,
        //shipment routes
        get_shipments,
        filter_shipments,
        get_shipment_by_id,
        insert_single_shipment,
        delete_shipment_by_id,
        update_shipment_by_id,
        options_shipment,
        //order routes
        get_orders,
        filter_orders,
        get_order_by_id,
        insert_single_order,
        delete_order_by_id,
        update_order_by_id,
        options_order,
    ]
}
