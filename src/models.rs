use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct User{
   pub id: Uuid,
   pub username: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Task {
   pub id: Uuid,
   pub title: String,
   pub description: String,
   pub status: TaskStatus,
}
#[derive(Serialize, Deserialize, Clone)]
pub enum TaskStatus {
   Todo,
   InProgress,
   Done,
}