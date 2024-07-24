#[macro_use]
extern crate rocket;

use auth::auth_routes;
use crab_rocket::env_variables;
use crab_rocket_schema::establish_pool;
use crab_rocket_utils;
use dotenvy::dotenv;
use entities::entities_routes;
use rocket::{http::Method, Route};
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use schemas::schemas_routes;
use services::services_routes;
use std::env;

#[launch]
fn rocket() -> _ {
    // Clear environment variable before running.
    env::remove_var("DATABASE_URL");
    // Load env
    env_variables::load_env();
    dotenv().ok();
    let pool = establish_pool();

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

    let mut routes = Vec::<Route>::new();

    // routes.extend(doc_routes.clone());
    routes.extend(entities_routes().clone());
    routes.extend(services_routes().clone());
    routes.extend(schemas_routes().clone());
    routes.extend(auth_routes().clone());

    rocket::build().mount("/api", routes).attach(cors).manage(pool)
}
