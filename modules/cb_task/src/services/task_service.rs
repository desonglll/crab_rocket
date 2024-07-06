use crate::mappers::task_mapper::TaskMapper;
use crate::models::task::{NewTask, PatchTask, Task};
use crate::models::task_filter::TaskFilter;
use crab_rocket_schema::establish_pg_connection;
use obj_traits::mapper::mapper_crud::MapperCRUD;
use obj_traits::request::pagination_request_param::PaginationParam;
use obj_traits::request::request_param::RequestParam;
use obj_traits::response::data::Data;
use obj_traits::service::service_crud::ServiceCRUD;

pub struct TaskService {}

impl ServiceCRUD<Task, NewTask, PatchTask, RequestParam<PaginationParam, TaskFilter>>
    for TaskService
{
    fn get_all(
        param: &RequestParam<PaginationParam, TaskFilter>,
    ) -> Result<Data<Vec<Task>>, Box<dyn std::error::Error>> {
        match establish_pg_connection() {
            Ok(mut conn) => {
                match TaskMapper::get_all(&mut conn, param) {
                    Ok(data) => {
                        // if all_tasks.len() != 0 {
                        //     Ok(all_tasks)
                        // } else {
                        //     println!("Empty query.");
                        //     Err(Box::new(diesel::result::Error::NotFound))
                        // }
                        Ok(data)
                    }
                    Err(e) => {
                        // panic!("oWo! Please add task first!");

                        println!("{e:?}");
                        Err(Box::new(e))
                    }
                }
            }
            Err(e) => {
                println!("{e:?}");
                Err(Box::new(e))
            }
        }
    }

    fn get_by_id(pid: i32) -> Result<Task, Box<dyn std::error::Error>> {
        match establish_pg_connection() {
            Ok(mut conn) => match TaskMapper::get_by_id(&mut conn, pid) {
                Ok(task) => Ok(task),
                Err(e) => {
                    println!("{e:?}");
                    Err(Box::new(e))
                }
            },
            Err(e) => {
                println!("{e:?}");
                Err(Box::new(e))
            }
        }
    }

    fn add_single(obj: &NewTask) -> Result<Task, Box<dyn std::error::Error>> {
        match establish_pg_connection() {
            Ok(mut conn) => match TaskMapper::add_single(&mut conn, obj) {
                Ok(inserted_task) => Ok(inserted_task),
                Err(e) => {
                    println!("{e:?}");
                    Err(Box::new(e))
                }
            },
            Err(e) => {
                println!("{e:?}");
                Err(Box::new(e))
            }
        }
    }

    fn delete_by_id(pid: i32) -> Result<Task, Box<dyn std::error::Error>> {
        match establish_pg_connection() {
            Ok(mut conn) => match TaskMapper::delete_by_id(&mut conn, pid) {
                Ok(deleted_task) => Ok(deleted_task),
                Err(e) => {
                    println!("{e:?}");
                    Err(Box::new(e))
                }
            },
            Err(e) => {
                println!("{e:?}");
                Err(Box::new(e))
            }
        }
    }

    fn update_by_id(pid: i32, obj: &PatchTask) -> Result<Task, Box<dyn std::error::Error>> {
        match establish_pg_connection() {
            Ok(mut conn) => match TaskMapper::update_by_id(&mut conn, pid, &obj) {
                Ok(task) => Ok(task),
                Err(e) => {
                    println!("{e:?}");
                    Err(Box::new(e))
                }
            },
            Err(e) => {
                println!("{e:?}");
                Err(Box::new(e))
            }
        }
    }
    fn filter(
        param: &RequestParam<PaginationParam, TaskFilter>,
    ) -> Result<Data<Vec<Task>>, Box<dyn std::error::Error>> {
        match establish_pg_connection() {
            Ok(mut conn) => {
                match TaskMapper::filter(&mut conn, param) {
                    Ok(data) => Ok(data),
                    Err(e) => {
                        // panic!("oWo! Please add task first!");

                        println!("{e:?}");
                        Err(Box::new(e))
                    }
                }
            }
            Err(e) => {
                println!("{e:?}");
                Err(Box::new(e))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::task::NewTask;
    use crab_rocket_utils::time::get_e8_time;
    use obj_traits::request::pagination_request_param::PaginationParamTrait;

    #[test]
    fn test_insert_single_task() {
        let task = NewTask::new(
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
