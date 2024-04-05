use crate::models::task::{NewTask, PatchTask, Task};
use crate::schema::tasks::dsl::*; //配合下面的 `tasks.filter()`
use crate::schema::tasks::{self};
use crate::utils::time::get_e8_time;
use diesel::prelude::*;

/// Inserts a task into the database.
pub fn insert_task(conn: &mut PgConnection, task: &NewTask) -> Result<Task, diesel::result::Error> {
    let inserted_task = diesel::insert_into(tasks::table)
        .values(task)
        .returning(Task::as_returning())
        .get_result(conn)
        .expect("Error saving new task");
    Ok(inserted_task)
}

pub fn insert_full_single_task(
    conn: &mut PgConnection,
    task: &Task,
) -> Result<Task, diesel::result::Error> {
    let inserted_task = diesel::insert_into(tasks::table)
        .values(task)
        .returning(Task::as_returning())
        .get_result(conn)
        .expect("Error saving new task");
    Ok(inserted_task)
}

/// Fetch all tasks
pub fn fetch_all_tasks(conn: &mut PgConnection) -> Result<Vec<Task>, diesel::result::Error> {
    // sort
    // https://docs.diesel.rs/master/diesel/query_dsl/trait.QueryDsl.html#method.order_by
    tasks::table.order(tasks::id.asc()).load::<Task>(conn)
}

/// ## Fetch task by id
/// 在 Diesel 中，通常使用 table
/// 方法来指定查询的表，然后在其基础上进行进一步的过滤、排序等操作
/// 但是在你的情况下，你已经在 Task 结构体上使用了 #[diesel(table_name =
/// "tasks")] 注解， 指定了表名为 "tasks"，因此不需要使用 table 方法了。
/// 当你使用 #[derive(Queryable)] 或 #[derive(Insertable)] 等宏来为结构体生成
/// Diesel 相关的代码时， 它们会自动为结构体实现相应的方法，并且使用 table
/// 方法来指定表名。 因此，你在使用查询时直接使用 tasks 而不是 tasks.table
/// 就可以了。 tasks::table.filter(tasks::id.eq(id)).first(conn)
pub fn fetch_task_by_id(conn: &mut PgConnection, t_id: i32) -> Result<Task, diesel::result::Error> {
    tasks.filter(tasks::id.eq(t_id)).first(conn)
}

pub fn update_task_by_id(
    conn: &mut PgConnection,
    t_id: i32,
    task: PatchTask,
) -> Result<Task, diesel::result::Error> {
    diesel::update(tasks.filter(id.eq(t_id)))
        .set((
            tasks::title.eq(task.title),
            tasks::content.eq(task.content),
            tasks::updated_at.eq(Some(get_e8_time())), //Update time
        ))
        .get_result(conn)
}

pub fn delete_task_by_id(
    conn: &mut PgConnection,
    t_id: i32,
) -> Result<Task, diesel::result::Error> {
    diesel::delete(tasks.filter(tasks::id.eq(t_id))).get_result(conn)
}
pub fn check_exist_task_by_id(conn: &mut PgConnection, t_id: i32) -> bool {
    match tasks.filter(tasks::id.eq(t_id)).first::<Task>(conn) {
        Ok(_) => true,
        Err(_) => false,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::establish_pg_connection;

    #[test]
    fn test_insert_task() {
        let mut conn = establish_pg_connection();
        let task = NewTask::new(
            "title".to_string().into(),
            "new content".to_string().into(),
            Some(chrono::Local::now().naive_utc()),
            Some(chrono::Local::now().naive_utc()),
        );
        let _ = insert_task(&mut conn, &task);
    }

    #[test]
    fn test_fetch_all_tasks() {
        let mut conn = establish_pg_connection();
        let all_tasks = fetch_all_tasks(&mut conn).unwrap();
        let json_string = serde_json::to_string_pretty(&all_tasks).unwrap();
        println!("{json_string:?}");
    }
    #[test]
    fn test_fetch_task_by_id() {
        let mut conn = establish_pg_connection();
        let t_id = 3;
        let task = fetch_task_by_id(&mut conn, t_id).unwrap();
        println!("{task:?}");
    }

    #[test]
    fn test_update_task_by_id() {
        let mut conn = establish_pg_connection();
        let t_id = 1;
        let put_task: PatchTask = PatchTask::new(
            "title for put 1".to_string(),
            "new content for put".to_string().into(),
            Some(chrono::Local::now().naive_utc()),
        );
        let _ = update_task_by_id(&mut conn, t_id, put_task);
    }
    #[test]
    fn test_delete_task_by_id() {
        let mut conn = establish_pg_connection();
        let t_id = 2;
        let deleted_task = delete_task_by_id(&mut conn, t_id).unwrap();
        println!("deleted_task: {deleted_task:?}");
    }

    #[test]
    fn test_insert_full_single_task() {
        let mut conn = establish_pg_connection();
        let task = Task::new(1, "title".to_string(), "content".to_string().into());
        let inserted_task = insert_full_single_task(&mut conn, &task).unwrap();
        println!("{inserted_task:?}");
    }
}
