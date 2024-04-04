use crate::models::task::Task;

pub trait GetTask {
    fn get_task_by_id(id: u64) -> Option<Task>;
    fn get_all_tasks() -> Option<Vec<Task>>;
    fn insert_single_task(task: Task) -> Result<Task, ()>;
}
