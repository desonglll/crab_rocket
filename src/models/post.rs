use diesel::{deserialize::Queryable, prelude::Insertable, Selectable};
use rocket::serde::{Deserialize, Serialize};

use crate::utils::time::get_e8_time;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    pub post_id: i32,
    pub title: Option<String>,
    pub body: Option<String>,
    pub user_id: Option<i32>,
    pub status: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
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
    ) -> Self {
        Post {
            post_id,
            title,
            body,
            user_id,
            status,
            created_at,
            updated_at,
        }
    }

    pub fn new_empty() -> Self {
        Post {
            post_id: 0,
            title: None,
            body: None,
            user_id: None,
            status: None,
            created_at: None,
            updated_at: None,
        }
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewPost {
    title: Option<String>,
    body: Option<String>,
    user_id: Option<i32>,
    status: Option<String>,
    created_at: Option<chrono::NaiveDateTime>,
    updated_at: Option<chrono::NaiveDateTime>,
}

impl NewPost {
    pub fn new(
        title: Option<String>,
        body: Option<String>,
        user_id: Option<i32>,
        status: Option<String>,
        created_at: Option<chrono::NaiveDateTime>,
        updated_at: Option<chrono::NaiveDateTime>,
    ) -> Self {
        NewPost {
            title,
            body,
            user_id,
            status,
            created_at,
            updated_at,
        }
    }

    pub fn new_empty() -> Self {
        NewPost {
            title: None,
            body: None,
            user_id: None,
            status: None,
            created_at: None,
            updated_at: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PatchPost {
    pub title: Option<String>,
    pub body: Option<String>,
    pub user_id: Option<i32>,
    pub status: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
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
        PatchPost {
            title,
            body,
            user_id,
            status,
            created_at,
            updated_at,
        }
    }

    pub fn new_empty() -> Self {
        PatchPost {
            title: None,
            body: None,
            user_id: None,
            status: None,
            created_at: Some(get_e8_time()),
            updated_at: Some(get_e8_time()),
        }
    }
}

impl Into<PatchPost> for NewPost {
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
        let post = Post::new_empty();

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
        let new_post = NewPost::new(
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
        assert_eq!(
            new_post.created_at.unwrap().to_string(),
            "2024-04-06 12:00:00"
        );
        assert_eq!(
            new_post.updated_at.unwrap().to_string(),
            "2024-04-06 12:00:00"
        );
    }

    #[test]
    fn test_new_post_new_empty() {
        let new_post = NewPost::new_empty();
        assert_eq!(new_post.title, None);
        assert_eq!(new_post.body, None);
        assert_eq!(new_post.user_id, None);
        assert_eq!(new_post.status, None);
        assert_eq!(new_post.created_at, None);
        assert_eq!(new_post.updated_at, None);
    }
}
