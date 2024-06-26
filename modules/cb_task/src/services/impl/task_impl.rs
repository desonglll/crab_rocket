use crate::mappers::task_mapper;
use crate::models::task::{NewTask, Task};
use crate::services::task_service;
use crab_rocket_schema::establish_pg_connection;

impl task_service::GetTask for Task {
    // GOOD:
    fn insert_single_task(task: &NewTask) -> Result<Task, Box<dyn std::error::Error>> {
        match establish_pg_connection() {
            Ok(mut conn) => {
                match task_mapper::insert_task(&mut conn, task) {
                    Ok(inserted_task) => Ok(inserted_task),
                    Err(e) => {
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

    // GOOD:
    fn get_all_tasks() -> Result<Vec<Task>, Box<dyn std::error::Error>> {
        match establish_pg_connection() {
            Ok(mut conn) => {
                match task_mapper::fetch_all_tasks(&mut conn) {
                    Ok(all_tasks) => {
                        if all_tasks.len() != 0 {
                            Ok(all_tasks)
                        } else {
                            println!("Empty query.");
                            Err(Box::new(diesel::result::Error::NotFound))
                        }
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

    // GOOD:
    fn get_task_by_id(t_id: i32) -> Result<Task, Box<dyn std::error::Error>> {
        match establish_pg_connection() {
            Ok(mut conn) => {
                match task_mapper::fetch_task_by_id(&mut conn, t_id) {
                    Ok(task) => Ok(task),
                    Err(e) => {
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

    // GOOD:
    fn update_task_by_id(
        t_id: i32,
        task: &crate::models::task::PatchTask,
    ) -> Result<Task, Box<dyn std::error::Error>> {
        match establish_pg_connection() {
            Ok(mut conn) => {
                match task_mapper::update_task_by_id(&mut conn, t_id, &task) {
                    Ok(task) => Ok(task),
                    Err(e) => {
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

    /// ## Service impl
    /// GOOD:
    fn delete_task_by_id(t_id: i32) -> Result<Task, Box<dyn std::error::Error>> {
        match establish_pg_connection() {
            Ok(mut conn) => {
                match task_mapper::delete_task_by_id(&mut conn, t_id) {
                    Ok(deleted_task) => Ok(deleted_task),
                    Err(e) => {
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

    // GOOD:
    fn insert_full_single_task(task: &Task) -> Result<Task, Box<dyn std::error::Error>> {
        match establish_pg_connection() {
            Ok(mut conn) => {
                match task_mapper::insert_full_single_task(&mut conn, task) {
                    Ok(inserted_full_task) => Ok(inserted_full_task),
                    Err(e) => {
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

    fn filter_tasks_by_params(
        params: &crate::routes::task_param::TaskParam,
    ) -> Result<Vec<Task>, Box<dyn std::error::Error>> {
        match establish_pg_connection() {
            Ok(mut conn) => {
                match task_mapper::fetch_tasks_by_params(&mut conn, params) {
                    Ok(filtered_tasks) => {
                        if filtered_tasks.len() != 0 {
                            Ok(filtered_tasks)
                        } else {
                            println!("Empty query.");
                            Err(Box::new(diesel::result::Error::NotFound))
                        }
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
}

#[cfg(test)]
mod tests {
    use self::task_service::GetTask;
    use super::*;
    use crate::{models::task::PatchTask, routes::task_param::TaskParam};
    use crab_rocket_utils::time::get_e8_time;

    #[test]
    fn test_insert_single_task() {
        let task = NewTask::new(
            "Get up late".to_string(),
            None,
            Some(get_e8_time()),
            Some(get_e8_time()),
            Some(4),
        );
        let _ = Task::insert_single_task(&task);
    }

    #[test]
    fn test_get_all_tasks() {
        let all_tasks = Task::get_all_tasks();
        println!("{all_tasks:?}");
    }

    #[test]
    fn test_get_task_by_id() {
        let t_id = 3;
        let task = Task::get_task_by_id(t_id);
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
        let updated_task = Task::update_task_by_id(t_id, &task);
        println!("updated_task: {updated_task:?}");
    }

    #[test]
    fn test_delete_task_by_id() {
        let deleted_task = Task::delete_task_by_id(4);
        println!("deleted_task: {deleted_task:?}");
    }

    #[test]
    fn test_insert_full_single_task() {
        let task = Task::new(2, "title1".to_string(), "content".to_string().into(), Some(4));
        let inserted_task = Task::insert_full_single_task(&task);
        println!("{inserted_task:?}");
    }

    #[test]
    fn test_get_tasks_by_params() {
        let params: TaskParam = TaskParam {
            user_id: Some(2),
            limit: Some(10),
            offset: Some(0),
        };
        let filtered_tasks = Task::filter_tasks_by_params(&params);
        println!("{filtered_tasks:?}");
    }
}
