use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
    pub validated: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug)]
pub struct PaginatedData<T> {
    pub data: Vec<T>,
    pub count: i64,
    pub total_pages: i64,
    pub page: i64,
    pub page_size: i64,
}
