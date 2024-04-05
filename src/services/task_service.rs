use crate::models::task::{NewTask, PatchTask, Task};

pub trait GetTask {
    fn insert_single_task(task: NewTask) -> Task;
    fn get_all_tasks() -> Vec<Task>;
    fn get_task_by_id(t_id: i32) -> Task;
    fn update_task_by_id(t_id: i32, task: PatchTask) -> Task;
    fn delete_task_by_id(t_id: i32) -> Task;
    fn insert_full_single_task(task: Task) -> Task;
}
