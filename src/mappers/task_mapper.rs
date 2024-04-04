use crate::models::task::Task;
use rusqlite::{params, Connection, Result};

const GET_TASK_BY_ID_STATEMENT: &str = "SELECT id, title, content FROM tasks WHERE id = ?1";

const INSERT_TASK_STATEMENT: &str = "INSERT INTO tasks (title, content) VALUES (?1, ?2)";

pub fn get_sqlite_conn() -> Result<Connection> {
    let conn = Connection::open("tasks.sqlite")?;
    Ok(conn)
}

/// ## Insert task
pub fn insert_task(conn: &Connection, task: Task) -> Result<Task> {
    conn.execute(
        INSERT_TASK_STATEMENT,
        (&task.get_title(), &task.get_content()),
    )?;
    Ok(task)
}

/// ## Get task by id
pub fn get_task_by_id(conn: &Connection, id: u64) -> Result<Option<Task>> {
    let mut stmt = conn.prepare(GET_TASK_BY_ID_STATEMENT)?;
    let mut rows = stmt.query(params![id])?;
    if let Some(row) = rows.next()? {
        Ok(Some(Task::new(row.get(0)?, row.get(1)?, row.get(2)?)))
    } else {
        println!("No such {id}");
        Ok(None)
    }
}

pub fn get_all_tasks(conn: &Connection) -> Result<Option<Vec<Task>>> {
    let mut stmt = conn.prepare("SELECT id, title, content FROM tasks")?;
    let task_iter = stmt.query_map([], |row| {
        Ok(Task::new(row.get(0)?, row.get(1)?, row.get(2)?))
    })?;

    // 将任务迭代器转换为向量
    let mut tasks = Vec::new();
    for task_result in task_iter {
        tasks.push(task_result?);
    }

    // 检查是否有任务，如果有则返回 Some(tasks)，否则返回 None
    if tasks.is_empty() {
        Ok(None)
    } else {
        Ok(Some(tasks))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_task() {
        let task = Task::new(0, "second".to_string(), "hello world".to_string());
        let conn = get_sqlite_conn().unwrap();
        let _ = insert_task(&conn, task);
    }

    #[test]
    fn test_get_task_by_id() {
        let conn = get_sqlite_conn().unwrap();
        let task = get_task_by_id(&conn, 2).unwrap().unwrap();
        println!("task: {task:?}");
    }

    #[test]
    fn test_get_all_tasks() {
        let conn = get_sqlite_conn().unwrap();
        let tasks = get_all_tasks(&conn).unwrap().unwrap();
        println!("{tasks:?}");
    }
}
