use crab_rocket_utils::time::get_e8_time;
use diesel::{deserialize::Queryable, prelude::Insertable, Selectable};
use rocket::serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Default)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crab_rocket_schema::schema::post_table)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    post_id: i32,
    title: Option<String>,
    body: Option<String>,
    user_id: Option<i32>,
    status: Option<String>,
    created_at: Option<chrono::NaiveDateTime>,
    updated_at: Option<chrono::NaiveDateTime>,
    username: Option<String>,
}

impl Post {
    pub fn new(
        post_id: i32,
        title: Option<String>,
        body: Option<String>,
        user_id: Option<i32>,
        status: Option<String>,
        created_at: Option<chrono::NaiveDateTime>,
        updated_at: Option<chrono::NaiveDateTime>,
        username: Option<String>,
    ) -> Self {
        Self {
            post_id,
            title,
            body,
            user_id,
            status,
            created_at,
            updated_at,
            username,
        }
    }

    pub fn post_id(&self) -> i32 {
        self.post_id
    }

    pub fn title(&self) -> &Option<String> {
        &self.title
    }

    pub fn body(&self) -> &Option<String> {
        &self.body
    }

    pub fn user_id(&self) -> Option<i32> {
        self.user_id
    }

    pub fn status(&self) -> &Option<String> {
        &self.status
    }

    pub fn created_at(&self) -> Option<chrono::NaiveDateTime> {
        self.created_at
    }

    pub fn updated_at(&self) -> Option<chrono::NaiveDateTime> {
        self.updated_at
    }

    pub fn username(&self) -> &Option<String> {
        &self.username
    }

    pub fn set_post_id(&mut self, post_id: i32) {
        self.post_id = post_id;
    }

    pub fn set_title(&mut self, title: Option<String>) {
        self.title = title;
    }

    pub fn set_body(&mut self, body: Option<String>) {
        self.body = body;
    }

    pub fn set_user_id(&mut self, user_id: Option<i32>) {
        self.user_id = user_id;
    }

    pub fn set_status(&mut self, status: Option<String>) {
        self.status = status;
    }

    pub fn set_created_at(&mut self, created_at: Option<chrono::NaiveDateTime>) {
        self.created_at = created_at;
    }

    pub fn set_updated_at(&mut self, updated_at: Option<chrono::NaiveDateTime>) {
        self.updated_at = updated_at;
    }

    pub fn set_username(&mut self, username: Option<String>) {
        self.username = username;
    }
}

///示例数据
// {
//   "title": "Sample Post",
//   "body": "This is a sample post body.",
//   "user_id": 123,
//   "status": "Published",
//   "created_at": "2024-04-07T08:30:00",
//   "updated_at": "2024-04-07T08:30:00"
// }
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crab_rocket_schema::schema::post_table)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PostPost {
    title: Option<String>,
    body: Option<String>,
    user_id: Option<i32>,
    status: Option<String>,
    created_at: Option<chrono::NaiveDateTime>,
    updated_at: Option<chrono::NaiveDateTime>,
}

impl PostPost {
    pub fn new(
        title: Option<String>,
        body: Option<String>,
        user_id: Option<i32>,
        status: Option<String>,
        created_at: Option<chrono::NaiveDateTime>,
        updated_at: Option<chrono::NaiveDateTime>,
    ) -> Self {
        Self {
            title,
            body,
            user_id,
            status,
            created_at,
            updated_at,
        }
    }

    pub fn demo() -> Self {
        Self {
            title: Some(String::from("demo post")),
            body: Some(String::from("demo body")),
            user_id: Some(1),
            status: Some(String::from("demo status")),
            created_at: Some(get_e8_time()),
            updated_at: Some(get_e8_time()),
        }
    }

    pub fn title(&self) -> &Option<String> {
        &self.title
    }

    pub fn body(&self) -> &Option<String> {
        &self.body
    }

    pub fn user_id(&self) -> Option<i32> {
        self.user_id
    }

    pub fn status(&self) -> &Option<String> {
        &self.status
    }

    pub fn created_at(&self) -> Option<chrono::NaiveDateTime> {
        self.created_at
    }

    pub fn updated_at(&self) -> Option<chrono::NaiveDateTime> {
        self.updated_at
    }

    pub fn set_title(&mut self, title: Option<String>) {
        self.title = title;
    }

    pub fn set_body(&mut self, body: Option<String>) {
        self.body = body;
    }

    pub fn set_user_id(&mut self, user_id: Option<i32>) {
        self.user_id = user_id;
    }

    pub fn set_status(&mut self, status: Option<String>) {
        self.status = status;
    }

    pub fn set_created_at(&mut self, created_at: Option<chrono::NaiveDateTime>) {
        self.created_at = created_at;
    }

    pub fn set_updated_at(&mut self, updated_at: Option<chrono::NaiveDateTime>) {
        self.updated_at = updated_at;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crab_rocket_schema::schema::post_table)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PatchPost {
    title: Option<String>,
    body: Option<String>,
    user_id: Option<i32>,
    status: Option<String>,
    created_at: Option<chrono::NaiveDateTime>,
    updated_at: Option<chrono::NaiveDateTime>,
}

impl PatchPost {
    pub fn new(
        title: Option<String>,
        body: Option<String>,
        user_id: Option<i32>,
        status: Option<String>,
        created_at: Option<chrono::NaiveDateTime>,
        updated_at: Option<chrono::NaiveDateTime>,
    ) -> Self {
        Self {
            title,
            body,
            user_id,
            status,
            created_at,
            updated_at,
        }
    }

    pub fn title(&self) -> &Option<String> {
        &self.title
    }

    pub fn body(&self) -> &Option<String> {
        &self.body
    }

    pub fn user_id(&self) -> Option<i32> {
        self.user_id
    }

    pub fn status(&self) -> &Option<String> {
        &self.status
    }

    pub fn created_at(&self) -> Option<chrono::NaiveDateTime> {
        self.created_at
    }

    pub fn updated_at(&self) -> Option<chrono::NaiveDateTime> {
        self.updated_at
    }

    pub fn set_title(&mut self, title: Option<String>) {
        self.title = title;
    }

    pub fn set_body(&mut self, body: Option<String>) {
        self.body = body;
    }

    pub fn set_user_id(&mut self, user_id: Option<i32>) {
        self.user_id = user_id;
    }

    pub fn set_status(&mut self, status: Option<String>) {
        self.status = status;
    }

    pub fn set_created_at(&mut self, created_at: Option<chrono::NaiveDateTime>) {
        self.created_at = created_at;
    }

    pub fn set_updated_at(&mut self, updated_at: Option<chrono::NaiveDateTime>) {
        self.updated_at = updated_at;
    }
}

impl Into<PatchPost> for PostPost {
    fn into(self) -> PatchPost {
        PatchPost {
            title: self.title,
            body: self.body,
            user_id: self.user_id,
            status: self.status,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_post_new() {
        let post = Post::new(
            1,
            Some("Title".to_string()),
            Some("Body".to_string()),
            Some(123),
            Some("Published".to_string()),
            Some(
                chrono::NaiveDateTime::parse_from_str("2024-04-06 12:00:00", "%Y-%m-%d %H:%M:%S")
                    .unwrap(),
            ),
            Some(
                chrono::NaiveDateTime::parse_from_str("2024-04-06 12:00:00", "%Y-%m-%d %H:%M:%S")
                    .unwrap(),
            ),
            Some("mike".to_string()),
        );

        assert_eq!(post.post_id, 1);
        assert_eq!(post.title, Some("Title".to_string()));
        assert_eq!(post.body, Some("Body".to_string()));
        assert_eq!(post.user_id, Some(123));
        assert_eq!(post.status, Some("Published".to_string()));
        assert_eq!(post.created_at.unwrap().to_string(), "2024-04-06 12:00:00");
        assert_eq!(post.updated_at.unwrap().to_string(), "2024-04-06 12:00:00");
    }

    #[test]
    fn test_post_new_empty() {
        let post = Post::default();

        assert_eq!(post.post_id, 0);
        assert_eq!(post.title, None);
        assert_eq!(post.body, None);
        assert_eq!(post.user_id, None);
        assert_eq!(post.status, None);
        assert_eq!(post.created_at, None);
        assert_eq!(post.updated_at, None);
    }

    #[test]
    fn test_new_post_new() {
        let new_post = PostPost::new(
            Some("Title".to_string()),
            Some("Body".to_string()),
            Some(123),
            Some("Published".to_string()),
            Some(
                chrono::NaiveDateTime::parse_from_str("2024-04-06 12:00:00", "%Y-%m-%d %H:%M:%S")
                    .unwrap(),
            ),
            Some(
                chrono::NaiveDateTime::parse_from_str("2024-04-06 12:00:00", "%Y-%m-%d %H:%M:%S")
                    .unwrap(),
            ),
        );

        assert_eq!(new_post.title, Some("Title".to_string()));
        assert_eq!(new_post.body, Some("Body".to_string()));
        assert_eq!(new_post.user_id, Some(123));
        assert_eq!(new_post.status, Some("Published".to_string()));
        assert_eq!(new_post.created_at.unwrap().to_string(), "2024-04-06 12:00:00");
        assert_eq!(new_post.updated_at.unwrap().to_string(), "2024-04-06 12:00:00");
    }

    #[test]
    fn test_new_post_new_empty() {
        let new_post = PostPost::default();
        assert_eq!(new_post.title, None);
        assert_eq!(new_post.body, None);
        assert_eq!(new_post.user_id, None);
        assert_eq!(new_post.status, None);
        assert_eq!(new_post.created_at, None);
        assert_eq!(new_post.updated_at, None);
    }
}
