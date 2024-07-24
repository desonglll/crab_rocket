use diesel::prelude::*;

//配合下面的 `tasks.filter()`
use crab_rocket_schema::schema::task_table::{self};
use crab_rocket_schema::schema::task_table::dsl;
use crab_rocket_utils::time::get_e8_time;
use obj_traits::mapper::mapper_crud::MapperCRUD;
use obj_traits::request::pagination_request_param::Pagination;
use obj_traits::request::request_param::RequestParam;
use obj_traits::response::data::Data;

use crate::models::task::{PatchTask, PostTask, Task};
use crate::models::task_filter::TaskFilter;

pub struct TaskMapper {}

impl MapperCRUD for TaskMapper {
    type Item = Task;
    type PostItem = PostTask;
    type PatchItem = PatchTask;
    type Param = RequestParam<TaskFilter>;
    fn get_all(
        conn: &mut PgConnection,
        param: &RequestParam<TaskFilter>,
    ) -> Result<Data<Vec<Task>>, diesel::result::Error> {
        // 当前页码（page）
        // 每页条目数（per_page）
        //
        // 总页数（total_pages）
        //
        // 公式
        //
        // 当前页的 offset: (page - 1) * per_page
        //
        // 下一页的 offset: page * per_page
        //
        // 上一页的 offset: (page - 2) * per_page （如果 page > 1）
        //
        // limit 始终为 per_page
        // 计算分页相关
        let pagination = param.pagination.unwrap_or_default().clone();
        let page = (pagination.offset.unwrap() / pagination.limit.unwrap()) + 1;
        let per_page = pagination.limit.unwrap();
        // 获取总记录数
        let total_count = dsl::task_table.count().get_result::<i64>(conn)? as i32;
        // 计算总页数
        let total_pages = (total_count + per_page - 1) / per_page;

        let previous_page_offset = (page - 2) * per_page;
        let next_page_offset = page * per_page;
        let pagination = Pagination::new(
            page,
            per_page,
            total_pages,
            total_count,
            Some(format!("?limit={}&offset={}", per_page, next_page_offset)),
            Some(format!("?limit={}&offset={}", per_page, previous_page_offset)),
        );

        // 分页查询
        let data = dsl::task_table
            .order(dsl::updated_at.desc())
            .limit(per_page as i64)
            .offset(((page - 1) * per_page) as i64)
            .load::<Task>(conn)?;
        let body = Data::new(data, pagination);
        println!("Getting tasks successfully.");
        Ok(body)
    }

    fn get_by_id(conn: &mut PgConnection, pid: i32) -> Result<Task, diesel::result::Error> {
        dsl::task_table.filter(task_table::task_id.eq(pid)).first(conn)
    }

    fn add_single(conn: &mut PgConnection, obj: &PostTask) -> Result<Task, diesel::result::Error> {
        match diesel::insert_into(task_table::table)
            .values(obj)
            .returning(Task::as_returning())
            .get_result(conn)
        {
            Ok(inserted_task) => Ok(inserted_task),
            Err(e) => {
                println!("{e:?}");
                Err(e)
            }
        }
    }

    fn delete_by_id(conn: &mut PgConnection, pid: i32) -> Result<Task, diesel::result::Error> {
        diesel::delete(dsl::task_table.filter(task_table::task_id.eq(pid))).get_result(conn)
    }

    fn update_by_id(
        conn: &mut PgConnection,
        pid: i32,
        obj: &PatchTask,
    ) -> Result<Task, diesel::result::Error> {
        diesel::update(dsl::task_table.filter(dsl::task_id.eq(pid)))
            .set((
                task_table::title.eq(&obj.title),
                task_table::content.eq(&obj.content),
                task_table::updated_at.eq(Some(get_e8_time())), //Update time
                task_table::user_id.eq(obj.user_id),
            ))
            .get_result(conn)
    }
    fn filter(
        conn: &mut PgConnection,
        param: &RequestParam<TaskFilter>,
    ) -> Result<Data<Vec<Task>>, diesel::result::Error> {
        // 当前页码（page）
        // 每页条目数（per_page）
        //
        // 总页数（total_pages）
        //
        // 公式
        //
        // 当前页的 offset: (page - 1) * per_page
        //
        // 下一页的 offset: page * per_page
        //
        // 上一页的 offset: (page - 2) * per_page （如果 page > 1）
        //
        // limit 始终为 per_page
        // 计算分页相关
        let pagination = param.pagination.unwrap_or_default().clone();

        let page = (pagination.offset.unwrap() / pagination.limit.unwrap()) + 1;
        let per_page = pagination.limit.unwrap();
        // 获取总记录数
        let total_count = dsl::task_table.count().get_result::<i64>(conn)? as i32;
        // 计算总页数
        let total_pages = (total_count + per_page - 1) / per_page;

        let previous_page_offset = (page - 2) * per_page;
        let next_page_offset = page * per_page;
        let pagination = Pagination::new(
            page,
            per_page,
            total_pages,
            total_count,
            Some(format!("?limit={}&offset={}", per_page, next_page_offset)),
            Some(format!("?limit={}&offset={}", per_page, previous_page_offset)),
        );

        let mut query = dsl::task_table.into_boxed();

        // 分页查询
        query = query
            .order(dsl::created_at.desc())
            .limit(per_page as i64)
            .offset(((page - 1) * per_page) as i64);

        let filter = &param.filter;
        if let Some(f) = filter {
            if let Some(id) = &f.id {
                query = query.filter(dsl::task_id.eq(id));
            }
            if let Some(title) = &f.title {
                query = query.filter(dsl::title.like(format!("%{}%", title)));
            }
            if let Some(content) = &f.content {
                query = query.filter(dsl::content.like(format!("%{}%", content)));
            }
            if let Some(created_at_min) = &f.created_at_min {
                query = query.filter(dsl::created_at.ge(created_at_min));
            }
            if let Some(created_at_max) = &f.created_at_max {
                query = query.filter(dsl::created_at.le(created_at_max));
            }
            if let Some(updated_at_min) = &f.updated_at_min {
                query = query.filter(dsl::updated_at.ge(updated_at_min));
            }
            if let Some(updated_at_max) = &f.updated_at_max {
                query = query.filter(dsl::updated_at.le(updated_at_max));
            }
            if let Some(user_id) = &f.user_id {
                query = query.filter(dsl::user_id.eq(user_id));
            }
        }
        let data = query.load::<Task>(conn)?;
        let body = Data::new(data, pagination);
        Ok(body)
    }
}

#[cfg(test)]
mod tests {
    use rocket::State;

    use crab_rocket_schema::{DbPool, establish_pg_connection, establish_pool};
    use obj_traits::request::pagination_request_param::{PaginationParam, PaginationParamTrait};
    use obj_traits::request::request_param::RequestParam;

    use crate::models::task::{PatchTask, PostTask};

    use super::*;

    #[test]
    fn test_add_single_task() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        let mut conn = establish_pg_connection(pool).expect("Failed to establish connection");

        let task = PostTask::new(
            "Test Task Title".to_string(),
            Some("Test Task Content".to_string()),
            None,
            None,
            Some(1),
        );

        let result = TaskMapper::add_single(&mut conn, &task);
        assert!(result.is_ok());
        let inserted_task = result.unwrap();
        assert_eq!(inserted_task.title, "Test Task Title");
    }

    #[test]
    fn test_get_all_tasks() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        let mut conn = establish_pg_connection(pool).expect("Failed to establish connection");

        let param = RequestParam::new(Some(PaginationParam::demo()), None);

        let result = TaskMapper::get_all(&mut conn, &param);
        assert!(result.is_ok());
        let data = result.unwrap();
        assert!(data.data().len() > 0); // Assuming there are tasks in the database
    }

    #[test]
    fn test_get_task_by_id() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        let mut conn = establish_pg_connection(pool).expect("Failed to establish connection");

        // Assuming a task with ID 1 exists in the database
        let task_id = 1;
        let result = TaskMapper::get_by_id(&mut conn, task_id);
        assert!(result.is_ok());
        let task = result.unwrap();
        assert_eq!(task.task_id, task_id);
    }

    #[test]
    fn test_update_task_by_id() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        let mut conn = establish_pg_connection(pool).expect("Failed to establish connection");

        let patch_task = PatchTask::new(
            "Updated Title".to_string(),
            Some("Updated Content".to_string()),
            Some(1),
        );

        // Assuming a task with ID 1 exists in the database
        let task_id = 1;
        let result = TaskMapper::update_by_id(&mut conn, task_id, &patch_task);
        println!("{:#?}", result);
        assert!(result.is_ok());
        let updated_task = result.unwrap();
        assert_eq!(updated_task.title, "Updated Title");
    }

    #[test]
    fn test_delete_task_by_id() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        let mut conn = establish_pg_connection(pool).expect("Failed to establish connection");

        // Add a task to delete
        let task = PostTask::new(
            "Task to Delete".to_string(),
            Some("Content to Delete".to_string()),
            None,
            None,
            Some(1),
        );
        let inserted_task =
            TaskMapper::add_single(&mut conn, &task).expect("Failed to insert task");
        let task_id = inserted_task.task_id;

        // Delete the task
        let result = TaskMapper::delete_by_id(&mut conn, task_id);
        assert!(result.is_ok());
        let deleted_task = result.unwrap();
        assert_eq!(deleted_task.task_id, task_id);

        // Verify deletion
        let result = TaskMapper::get_by_id(&mut conn, task_id);
        assert!(result.is_err());
    }
}
