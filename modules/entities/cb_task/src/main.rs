#[macro_use]
extern crate rocket;

use crab_rocket_task::routes::task_route::*;
use crab_rocket_utils;
use dotenvy::dotenv;
use rocket::http::Method;
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use std::env;
#[launch]
fn rocket() -> _ {
    // Clear environment variable before running.
    env::remove_var("DATABASE_URL");
    // Load env
    // env_variables::load_env();
    dotenv().ok();

    crab_rocket_utils::run_preload();

    let allowed_origins = AllowedOrigins::All;
    // You can also deserialize this
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![
            Method::Get,
            Method::Post,
            Method::Patch,
            Method::Put,
            Method::Delete,
        ]
        .into_iter()
        .map(From::from)
        .collect(),
        allowed_headers: AllowedHeaders::some(&[
            "Content-Type",
            "Authorization",
            "Another-Header", // 允许的其他头部信息
            "Authorization",
            "Accept",
            "User-Agent",
            "X-Requested-With",
            "Referer",
        ]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .unwrap();

    rocket::build()
        .mount(
            "/api",
            routes![
                get_tasks,
                filter_tasks,
                get_task_by_id,
                insert_single_task,
                delete_task_by_id,
                update_task_by_id,
                options_task_filter
            ],
        )
        .attach(cors)
}