use crate::models::task::{NewTask, PatchTask, Task};
use crab_rocket_schema::schema::tasks::dsl; //配合下面的 `tasks.filter()`
use crab_rocket_schema::schema::tasks::{self};
use crab_rocket_utils::time::get_e8_time;
use diesel::prelude::*;
use obj_traits::{Data, PaginationRequestParam, RequestParam};
use obj_traits::{MapperCRUD, Pagination};

pub struct TaskMapper {}

impl MapperCRUD<Task, NewTask, PatchTask, RequestParam<PaginationRequestParam>> for TaskMapper {
    fn get_all(
        conn: &mut PgConnection,
        param: &RequestParam<PaginationRequestParam>,
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
        let page = (param.pagination.offset.unwrap() / param.pagination.limit.unwrap()) + 1;
        let per_page = param.pagination.limit.unwrap();
        // 获取总记录数
        let total_count = dsl::tasks.count().get_result::<i64>(conn)? as i32;
        // 计算总页数
        let total_pages = (total_count + per_page - 1) / per_page;

        let previous_page_offset = (page - 2) * per_page;
        let next_page_offset = page * per_page;
        let pagination =
            Pagination::new(page, per_page, total_pages, total_count, Some(format!("?limit={}&offset={}", per_page, next_page_offset)), Some(format!("?limit={}&offset={}", per_page, previous_page_offset)));

        // 分页查询
        let data = dsl::tasks
            .order(dsl::updated_at.desc())
            .limit(per_page as i64)
            .offset(((page - 1) * per_page) as i64)
            .load::<Task>(conn)?;
        let body = Data::new(data, pagination);
        Ok(body)
    }

    fn get_by_id(conn: &mut PgConnection, pid: i32) -> Result<Task, diesel::result::Error> {
        dsl::tasks.filter(tasks::id.eq(pid)).first(conn)
    }

    fn add_single(conn: &mut PgConnection, obj: &NewTask) -> Result<Task, diesel::result::Error> {
        match diesel::insert_into(tasks::table)
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
        diesel::delete(dsl::tasks.filter(tasks::id.eq(pid))).get_result(conn)
    }

    fn update_by_id(
        conn: &mut PgConnection,
        pid: i32,
        obj: &PatchTask,
    ) -> Result<Task, diesel::result::Error> {
        diesel::update(dsl::tasks.filter(dsl::id.eq(pid)))
            .set((
                tasks::title.eq(obj.title()),
                tasks::content.eq(obj.content()),
                tasks::updated_at.eq(Some(get_e8_time())), //Update time
                tasks::user_id.eq(obj.user_id()),
            ))
            .get_result(conn)
    }
}

#[cfg(test)]
mod tests {
    use rocket::yansi::Paint;
    use super::TaskMapper;
    use crate::models::task::{NewTask, PatchTask};
    use crab_rocket_schema::establish_pg_connection;
    use obj_traits::{MapperCRUD, PaginationRequestParam, RequestParam};

    #[test]
    fn test_insert_task() {
        match establish_pg_connection() {
            Ok(mut conn) => {
                let task = NewTask::new(
                    "title".to_string().into(),
                    "new content".to_string().into(),
                    Some(chrono::Local::now().naive_utc()),
                    Some(chrono::Local::now().naive_utc()),
                    Some(4),
                );
                let _ = super::TaskMapper::add_single(&mut conn, &task);
            }
            Err(_) => println!("establish_pg_connection error"),
        }
    }

    #[test]
    fn test_fetch_all_tasks() {
        match establish_pg_connection() {
            Ok(mut conn) => {
                let param = RequestParam::new(PaginationRequestParam::default());
                let all_tasks = super::TaskMapper::get_all(&mut conn, &param).unwrap();
                let json_string = serde_json::to_string_pretty(&all_tasks).unwrap();
                println!("{json_string:?}");
            }
            Err(_) => println!("establish_pg_connection error"),
        }
    }

    #[test]
    fn test_fetch_task_by_id() {
        match establish_pg_connection() {
            Ok(mut conn) => {
                let t_id = 3;
                match super::TaskMapper::get_by_id(&mut conn, t_id) {
                    Ok(task) => println!("{task:?}"),
                    Err(_) => println!("err"),
                }
            }
            Err(_) => println!("establish_pg_connection error"),
        }
    }

    #[test]
    fn test_update_task_by_id() {
        match establish_pg_connection() {
            Ok(mut conn) => {
                let t_id = 1;
                let patch_task: PatchTask = PatchTask::new(
                    "title for put 1".to_string(),
                    "new content for put".to_string().into(),
                    Some(4),
                );
                match super::TaskMapper::update_by_id(&mut conn, t_id, &patch_task) {
                    Ok(res) => println!("{res:?}"),
                    Err(_) => println!("Err"),
                }
            }
            Err(_) => println!("establish_pg_connection error"),
        }
    }

    #[test]
    fn test_delete_task_by_id() {
        match establish_pg_connection() {
            Ok(mut conn) => {
                let t_id = 2;
                match super::TaskMapper::delete_by_id(&mut conn, t_id) {
                    Ok(res) => println!("{res:?}"),
                    Err(_) => println!("Err"),
                }
            }
            Err(_) => println!("establish_pg_connection error"),
        }
    }

    #[test]
    fn test_fetch_tasks_by_params() {
        match establish_pg_connection() {
            Ok(mut conn) => {
                let param: PaginationRequestParam = PaginationRequestParam {
                    limit: Some(10),
                    offset: Some(0),
                };
                match TaskMapper::get_by_param(&mut conn, &param) {
                    Ok(filtered_tasks) => println!("{filtered_tasks:?}"),
                    Err(_) => println!("Err"),
                }
            }
            Err(_) => println!("establish_pg_connection error"),
        }
    }
}
