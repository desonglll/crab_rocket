#[macro_use]
extern crate rocket;

use crab_rocket::routes::{
    common, employee_route, follow_route, info_route, permission_route, post_route, role_route,
    task_route, upload_route, user_route,
};
use dotenvy::dotenv;
use std::env;

use rocket::http::Method;
use rocket::routes;
use rocket_cors::{AllowedHeaders, AllowedOrigins};

#[launch]
fn rocket() -> _ {
    // Clear environment variable before running.
    env::remove_var("DATABASE_URL");
    dotenv().ok();

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
        ]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .unwrap();
    rocket::build()
        .mount(
            "/",
            routes![
                common::doc,
                info_route::get_info,
                // task routes
                task_route::index,
                task_route::demo,
                task_route::insert_single_task,
                task_route::get_all_tasks,
                task_route::get_task_by_id,
                task_route::update_task_by_id,
                task_route::delete_task_by_id,
                task_route::put_task,
                task_route::get_tasks_by_params,
                task_route::options_task_filter,
                //user routes
                user_route::insert_single_user,
                user_route::get_all_users,
                user_route::get_user_by_id,
                user_route::update_user_by_id,
                user_route::delete_user_by_id,
                user_route::options_user_filter,
                // post routes
                post_route::insert_single_post,
                post_route::get_all_posts,
                post_route::get_post_by_id,
                post_route::update_post_by_id,
                post_route::delete_post_by_id,
                post_route::get_posts_by_params,
                post_route::options_post_filter,
                // follow routes
                follow_route::insert_single_follow,
                follow_route::get_all_follows,
                follow_route::get_follows_by_params,
                follow_route::delete_follow,
                // employee routes
                employee_route::insert_single_employee,
                employee_route::delete_employee_by_id,
                employee_route::get_employee_by_params,
                employee_route::update_employee_by_id,
                // upload
                upload_route::single_upload,
                // role routes
                role_route::insert_role,
                role_route::get_all_roles,
                role_route::delete_role_by_id,
                role_route::get_role_by_id,
                role_route::update_role_by_id,
                // permission routes
                permission_route::get_all_permissions
            ],
        )
        .attach(cors)
}
