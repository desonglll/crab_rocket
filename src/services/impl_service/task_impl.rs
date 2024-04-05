use crate::establish_pg_connection;
use crate::mappers::task_mapper;
use crate::models::task::{NewTask, Task};
use crate::services::task_service;
impl task_service::GetTask for Task {
    fn insert_single_task(task: NewTask) -> Task {
        let mut conn = establish_pg_connection();
        match task_mapper::insert_task(&mut conn, &task) {
            Ok(inserted_task) => inserted_task,
            Err(e) => panic!("oWo! {e:?}"),
        }
    }
    fn get_all_tasks() -> Vec<Task> {
        let mut conn = establish_pg_connection();
        match task_mapper::fetch_all_tasks(&mut conn) {
            Ok(all_tasks) => all_tasks,
            Err(_) => panic!("oWo! Please add task first!"),
        }
    }
    fn get_task_by_id(t_id: i32) -> Task {
        let mut conn = establish_pg_connection();
        match task_mapper::fetch_task_by_id(&mut conn, t_id) {
            Ok(task) => task,
            Err(e) => panic!("oWo! Error! {e:?}"),
        }
    }
    fn update_task_by_id(t_id: i32, task: crate::models::task::PatchTask) -> Task {
        let mut conn = establish_pg_connection();
        match task_mapper::update_task_by_id(&mut conn, t_id, task) {
            Ok(task) => task,
            Err(e) => panic!("oWo! Error! {e:?}"),
        }
    }
    fn delete_task_by_id(t_id: i32) -> Task {
        let mut conn = establish_pg_connection();
        match task_mapper::delete_task_by_id(&mut conn, t_id) {
            Ok(deleted_task) => deleted_task,
            Err(e) => panic!("oWo! Error! {e:?}"),
        }
    }
    fn insert_full_single_task(task: Task) -> Task {
        let mut conn = establish_pg_connection();
        match task_mapper::insert_full_single_task(&mut conn, &task) {
            Ok(inserted_full_task) => inserted_full_task,
            Err(e) => panic!("oWo! {e:?}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{models::task::PatchTask, utils::time::get_e8_time};

    use self::task_service::GetTask;
    use super::*;

    #[test]
    fn test_insert_single_task() {
        let task = NewTask::new(
            "Get up late".to_string(),
            None,
            Some(get_e8_time()),
            Some(get_e8_time()),
        );
        let _ = Task::insert_single_task(task);
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
            Some(get_e8_time()),
        );
        let updated_task = Task::update_task_by_id(t_id, task);
        println!("updated_task: {updated_task:?}");
    }
    #[test]
    fn test_delete_task_by_id() {
        let deleted_task = Task::delete_task_by_id(4);
        println!("deleted_task: {deleted_task:?}");
    }
    #[test]
    fn test_insert_full_single_task() {
        let task = Task::new(2, "title1".to_string(), "content".to_string().into());
        let inserted_task = Task::insert_full_single_task(task);
        println!("{inserted_task:?}");
    }
}
