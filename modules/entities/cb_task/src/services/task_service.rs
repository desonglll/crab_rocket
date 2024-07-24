use crate::mappers::task_mapper::TaskMapper;
use crate::models::task::{PatchTask, PostTask, Task};
use crate::models::task_filter::TaskFilter;

use obj_traits::service::service_crud::ServiceCRUD;

pub struct TaskService {}

impl ServiceCRUD for TaskService {
    type Item = Task;
    type PostItem = PostTask;
    type PatchItem = PatchTask;
    type Filter = TaskFilter;
    type Mapper = TaskMapper;
}

#[cfg(test)]
mod test {
    use crate::services::task_service::TaskService;
    use crab_rocket_schema::{establish_pool, DbPool};
    use obj_traits::request::pagination_request_param::{PaginationParam, PaginationParamTrait};
    use obj_traits::request::request_param::RequestParam;
    use obj_traits::service::service_crud::ServiceCRUD;
    use rocket::State;

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
