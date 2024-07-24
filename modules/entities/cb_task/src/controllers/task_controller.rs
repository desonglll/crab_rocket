use obj_traits::controller::controller_crud::ControllerCRUD;

use crate::models::task::{PatchTask, PostTask, Task};
use crate::models::task_filter::TaskFilter;
use crate::services::task_service::TaskService;

pub struct TaskController {}

impl ControllerCRUD for TaskController {
    type Item = Task;
    type PostItem = PostTask;
    type PatchItem = PatchTask;
    type Filter = TaskFilter;
    type Service = TaskService;
}
