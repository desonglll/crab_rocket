#[macro_use]
extern crate rocket;
use hello_rocket::routes::{task_route, user_route};

use rocket::http::Method;
use rocket::routes;
use rocket_cors::{AllowedHeaders, AllowedOrigins};

#[launch]
fn rocket() -> _ {
    let allowed_origins = AllowedOrigins::All;

    // You can also deserialize this
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .unwrap();
    rocket::build()
        .mount(
            "/",
            routes![
                task_route::index,
                task_route::demo,
                task_route::insert_single_task,
                task_route::get_all_tasks,
                task_route::get_task_by_id,
                task_route::update_task_by_id,
                task_route::delete_task_by_id,
                task_route::put_task,
                user_route::insert_single_user,
                user_route::get_all_users,
                user_route::get_user_by_id,
                user_route::update_user_by_id,
                user_route::delete_user_by_id
            ],
        )
        .attach(cors)
}
