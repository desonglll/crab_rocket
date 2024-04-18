use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Info {
    pub post_count: i64,
    pub employee_count: i64,
    pub task_count: i64,
    pub user_count: i64,
}

impl Info {
    pub fn new_empty() -> Self {
        Info {
            post_count: 0,
            employee_count: 0,
            task_count: 0,
            user_count: 0,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::models::info::Info;

    #[test]
    pub fn test_info() {
        let info = Info {
            post_count: 12,
            employee_count: 0,
            task_count: 0,
            user_count: 0,
        };

        let json_string = serde_json::to_string(&info).unwrap();
        println!("{json_string:?}");
    }
}
