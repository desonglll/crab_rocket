use crate::mappers::task_mapper;
use crate::models::task::Task;
use crate::services::task_service;
impl task_service::GetTask for Task {
    fn get_task_by_id(id: u64) -> Option<Task> {
        let conn = task_mapper::get_sqlite_conn().unwrap();
        let task = task_mapper::get_task_by_id(&conn, id).unwrap();
        task
    }

    fn get_all_tasks() -> Option<Vec<Task>> {
        let conn = task_mapper::get_sqlite_conn().unwrap();
        let tasks: Option<Vec<Task>> = task_mapper::get_all_tasks(&conn).unwrap();
        tasks
    }

    fn insert_single_task(task: Task) -> Result<Task, ()> {
        let conn = task_mapper::get_sqlite_conn().unwrap();
        let task = task_mapper::insert_task(&conn, task).unwrap();
        Ok(task)
    }
}

#[cfg(test)]
mod tests {
    use self::task_service::GetTask;

    use super::*;
    #[test]
    fn test_get_all_tasks() {
        let tasks = Task::get_all_tasks();
        println!("{tasks:?}");
    }

    #[test]
    fn test_insert_task() {
        let task = Task::new(99, "test insert".to_string(), "content".to_string());
        let result = Task::insert_single_task(task).unwrap();
        println!("{result:?}");
    }
}
