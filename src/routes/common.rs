use rocket::{get, serde::json::Json};
use utoipa::OpenApi;

use crate::models::employee::{NewEmployee, PatchEmployee};
use crate::models::{employee::Employee, follow::Follow, post::Post, task::Task, user::User};
use crate::routes::*;

use self::models::employee_param::EmployeeParam;

#[derive(OpenApi)]
#[openapi(
paths(
// task routes
task_route::insert_single_task,
task_route::get_all_tasks,
task_route::get_task_by_id,
task_route::update_task_by_id,
task_route::delete_task_by_id,
task_route::put_task,
task_route::get_tasks_by_params,
// post routes
post_route::insert_single_post,
post_route::get_all_posts,
post_route::get_post_by_id,
post_route::update_post_by_id,
post_route::delete_post_by_id,
post_route::get_posts_by_params,
//user routes
user_route::insert_single_user,
user_route::get_all_users,
user_route::get_user_by_id,
user_route::update_user_by_id,
user_route::delete_user_by_id,
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
),
components(schemas(Task, User, Post, Follow, Employee, PatchEmployee, NewEmployee, EmployeeParam))
)]
struct ApiDoc;

#[get("/doc")]
pub fn doc() -> Json<serde_json::Value> {
    let response =
        serde_json::from_str(ApiDoc::openapi().to_pretty_json().unwrap().as_str()).unwrap();
    Json(response)
}
