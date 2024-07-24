use std::error::Error;

use rocket::State;

use crab_rocket_schema::DbPool;
use obj_traits::request::request_param::RequestParam;
use obj_traits::response::data::Data;
use obj_traits::service::service_crud::{
    service_add_single, service_delete_by_id, service_filter, service_get_all, service_get_by_id,
    service_update_by_id, ServiceCRUD,
};

use crate::mappers::task_mapper::TaskMapper;
use crate::models::task::{PatchTask, PostTask, Task};
use crate::models::task_filter::TaskFilter;

pub struct TaskService {}

impl ServiceCRUD for TaskService {
    type Item = Task;
    type PostItem = PostTask;
    type PatchItem = PatchTask;
    type Param = RequestParam<TaskFilter>;
    fn get_all(
        pool: &State<DbPool>,
        param: &RequestParam<TaskFilter>,
    ) -> Result<Data<Vec<Task>>, Box<dyn Error>> {
        service_get_all::<Task, TaskMapper, TaskFilter>(pool, param)
    }
    fn get_by_id(pool: &State<DbPool>, pid: i32) -> Result<Task, Box<dyn Error>> {
        service_get_by_id::<Task, TaskMapper>(pool, pid)
    }

    fn add_single(pool: &State<DbPool>, obj: &PostTask) -> Result<Task, Box<dyn Error>> {
        service_add_single::<Task, TaskMapper, PostTask>(pool, obj)
    }

    fn delete_by_id(pool: &State<DbPool>, pid: i32) -> Result<Task, Box<dyn Error>> {
        service_delete_by_id::<Task, TaskMapper>(pool, pid)
    }

    fn update_by_id(
        pool: &State<DbPool>,
        pid: i32,
        obj: &PatchTask,
    ) -> Result<Task, Box<dyn Error>> {
        service_update_by_id::<Task, TaskMapper, PatchTask>(pool, pid, obj)
    }
    fn filter(
        pool: &State<DbPool>,
        param: &RequestParam<TaskFilter>,
    ) -> Result<Data<Vec<Task>>, Box<dyn std::error::Error>> {
        service_filter::<Task, TaskMapper, TaskFilter>(pool, param)
    }
}

#[cfg(test)]
mod test {
    use rocket::State;

    use crab_rocket_schema::{DbPool, establish_pool};
    use obj_traits::request::pagination_request_param::{PaginationParam, PaginationParamTrait};
    use obj_traits::request::request_param::RequestParam;
    use obj_traits::service::service_crud::ServiceCRUD;

    use crate::services::task_service::TaskService;

    #[test]
    fn test_insert_single_task() {
        use crate::models::task::PostTask;
        let task = PostTask::demo();
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match TaskService::add_single(pool, &task) {
            Ok(result) => println!("{result}"),
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn test_get_all_tasks() {
        let param = RequestParam::new(Some(PaginationParam::demo()), None);
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match TaskService::get_all(pool, &param) {
            Ok(res) => println!("{res}"),
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn test_get_task_by_id() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match TaskService::get_by_id(pool, 1) {
            Ok(res) => println!("{res:?}"),
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn test_delete_task_by_id() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match TaskService::delete_by_id(pool, 2) {
            Ok(res) => println!("{res:?}"),
            Err(e) => println!("{e:?}"),
        }
    }
}
