use std::error::Error;

use crate::mappers::task_mapper::TaskMapper;
use crate::models::task::{PostTask, PatchTask, Task};
use crate::models::task_filter::TaskFilter;
use obj_traits::request::pagination_request_param::PaginationParam;
use obj_traits::request::request_param::RequestParam;
use obj_traits::response::data::Data;
use obj_traits::service::service_crud::{
    service_add_single, service_delete_by_id, service_filter, service_get_all, service_get_by_id,
    service_update_by_id, ServiceCRUD,
};

pub struct TaskService {}

impl ServiceCRUD for TaskService {
    type Item = Task;
    type PostItem = PostTask;
    type PatchItem = PatchTask;
    type Param = RequestParam<PaginationParam, TaskFilter>;
    fn get_all(
        param: &RequestParam<PaginationParam, TaskFilter>,
    ) -> Result<Data<Vec<Task>>, Box<dyn Error>> {
        service_get_all::<Task, TaskMapper, TaskFilter>(param)
    }
    fn get_by_id(pid: i32) -> Result<Task, Box<dyn Error>> {
        service_get_by_id::<Task, TaskMapper>(pid)
    }

    fn add_single(obj: &PostTask) -> Result<Task, Box<dyn Error>> {
        service_add_single::<Task, TaskMapper, PostTask>(obj)
    }

    fn delete_by_id(pid: i32) -> Result<Task, Box<dyn Error>> {
        service_delete_by_id::<Task, TaskMapper>(pid)
    }

    fn update_by_id(pid: i32, obj: &PatchTask) -> Result<Task, Box<dyn Error>> {
        service_update_by_id::<Task, TaskMapper, PatchTask>(pid, obj)
    }
    fn filter(
        param: &RequestParam<PaginationParam, TaskFilter>,
    ) -> Result<Data<Vec<Task>>, Box<dyn std::error::Error>> {
        service_filter::<Task, TaskMapper, TaskFilter>(param)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::task::PostTask;
    use crab_rocket_utils::time::get_e8_time;
    use obj_traits::request::pagination_request_param::PaginationParamTrait;

    #[test]
    fn test_insert_single_task() {
        let task = PostTask::new(
            "Get up late".to_string(),
            None,
            Some(get_e8_time()),
            Some(get_e8_time()),
            Some(1),
        );
        let _ = TaskService::add_single(&task);
    }

    #[test]
    fn test_get_all_tasks() {
        let param = RequestParam::new(PaginationParam::demo(), None);
        let all_tasks = TaskService::get_all(&param).unwrap();
        println!("{all_tasks}");
    }

    #[test]
    fn test_get_task_by_id() {
        let t_id = 3;
        let task = TaskService::get_by_id(t_id);
        println!("{task:?}");
    }

    #[test]
    fn test_update_task_by_id() {
        let t_id = 1;
        let task: PatchTask = PatchTask::new(
            "new title for put task".to_string(),
            "hello".to_string().into(),
            Some(4),
        );
        let updated_task = TaskService::update_by_id(t_id, &task);
        println!("updated_task: {updated_task:?}");
    }

    #[test]
    fn test_delete_task_by_id() {
        let deleted_task = TaskService::delete_by_id(4);
        println!("deleted_task: {deleted_task:?}");
    }
}
