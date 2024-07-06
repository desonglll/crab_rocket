use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct PaginationParam {
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}
pub trait PaginationParamTrait {
    fn new(limit: Option<i32>, offset: Option<i32>) -> Self;
    fn demo() -> Self;
    fn default() -> Self;
}
impl PaginationParamTrait for PaginationParam {
    fn new(limit: Option<i32>, offset: Option<i32>) -> Self {
        Self {
            limit,
            offset,
        }
    }
    fn demo() -> Self {
        Self {
            limit: Some(10),
            offset: Some(0),
        }
    }
    fn default() -> Self {
        Self {
            limit: Some(10),
            offset: Some(0),
        }
    }
}

#[derive(Serialize, Default, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Pagination {
    page: i32,
    per_page: i32,
    total_pages: i32,
    count: i32,
    next: Option<String>,
    previous: Option<String>,
}

impl Pagination {
    pub fn new(
        page: i32,
        per_page: i32,
        total_pages: i32,
        count: i32,
        next: Option<String>,
        previous: Option<String>,
    ) -> Self {
        Self {
            page,
            per_page,
            total_pages,
            count,
            next,
            previous,
        }
    }
}

impl std::fmt::Display for Pagination {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Format the pagination information into a string
        write!(f, "Page {} of {} | Showing {} items", self.page, self.total_pages, self.count)?;

        // Optionally add previous and next links
        if let Some(prev) = &self.previous {
            write!(f, " | Previous: {}", prev)?;
        }
        if let Some(next) = &self.next {
            write!(f, " | Next: {}", next)?;
        }

        Ok(())
    }
}
