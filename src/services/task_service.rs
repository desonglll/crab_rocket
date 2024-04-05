use crate::models::task::{NewTask, PutTask, Task};

pub trait GetTask {
    fn insert_single_task(task: NewTask) -> Task;
    fn get_all_tasks() -> Vec<Task>;
    fn get_task_by_id(t_id: i32) -> Task;
    fn update_task_by_id(t_id: i32, task: PutTask) -> Task;
}
