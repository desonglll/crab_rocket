use crate::models::task::{NewTask, PatchTask, Task};

pub trait GetTask {
    fn insert_single_task(task: NewTask) -> Result<Task, diesel::result::Error>;
    fn get_all_tasks() -> Result<Vec<Task>, diesel::result::Error>;
    fn get_task_by_id(t_id: i32) -> Result<Task, diesel::result::Error>;
    fn update_task_by_id(t_id: i32, task: PatchTask) -> Result<Task, diesel::result::Error>;
    fn delete_task_by_id(t_id: i32) -> Result<Task, diesel::result::Error>;
    fn insert_full_single_task(task: Task) -> Result<Task, diesel::result::Error>;
}
