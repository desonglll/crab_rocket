use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(crate = "rocket::serde")]
pub struct Info {
    post_count: i64,
    employee_count: i64,
    task_count: i64,
    user_count: i64,
}

impl Info {
    pub fn new(post_count: i64, employee_count: i64, task_count: i64, user_count: i64) -> Self {
        Self {
            post_count,
            employee_count,
            task_count,
            user_count,
        }
    }

    pub fn post_count(&self) -> i64 {
        self.post_count
    }

    pub fn employee_count(&self) -> i64 {
        self.employee_count
    }

    pub fn task_count(&self) -> i64 {
        self.task_count
    }

    pub fn user_count(&self) -> i64 {
        self.user_count
    }

    pub fn set_post_count(&mut self, post_count: i64) {
        self.post_count = post_count;
    }

    pub fn set_employee_count(&mut self, employee_count: i64) {
        self.employee_count = employee_count;
    }

    pub fn set_task_count(&mut self, task_count: i64) {
        self.task_count = task_count;
    }

    pub fn set_user_count(&mut self, user_count: i64) {
        self.user_count = user_count;
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
