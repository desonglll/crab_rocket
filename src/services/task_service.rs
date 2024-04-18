use crate::{
    models::task::{NewTask, PatchTask, Task},
    routes::models::task_param::TaskParam,
};

pub trait GetTask {
    fn insert_single_task(task: &NewTask) -> Result<Task, Box<dyn std::error::Error>>;
    fn get_all_tasks() -> Result<Vec<Task>, Box<dyn std::error::Error>>;
    fn get_task_by_id(t_id: i32) -> Result<Task, Box<dyn std::error::Error>>;
    fn update_task_by_id(t_id: i32, task: &PatchTask) -> Result<Task, Box<dyn std::error::Error>>;
    fn delete_task_by_id(t_id: i32) -> Result<Task, Box<dyn std::error::Error>>;
    fn insert_full_single_task(task: &Task) -> Result<Task, Box<dyn std::error::Error>>;
    fn filter_tasks_by_params(params: &TaskParam) -> Result<Vec<Task>, Box<dyn std::error::Error>>;
}
