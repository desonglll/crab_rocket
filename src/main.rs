#[macro_use]
extern crate rocket;
use hello_rocket::routes::task_route;

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![
            task_route::index,
            task_route::insert_single_task,
            task_route::get_all_tasks,
            task_route::get_task_by_id,
            task_route::update_task_by_id,
            task_route::delete_task_by_id,
            task_route::put_task
        ],
    )
}
