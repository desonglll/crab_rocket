#[macro_use]
extern crate rocket;
use hello_rocket::routes::task_route;
use hello_rocket::utils::sqlite_tools;
use std::env::{self};

#[launch]
fn rocket() -> _ {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        //check whether the length of args is more than 1.
        let op = &args[1];
        if op == "init" {
            let _ = sqlite_tools::init_sqlite_db();
        }
    }

    rocket::build().mount(
        "/",
        routes![
            task_route::index,
            task_route::get_task_by_id,
            task_route::get_all_tasks,
            task_route::insert_single_task
        ],
    )
}
