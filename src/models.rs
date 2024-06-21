use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct User{
   pub id: Uuid,
   pub username: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct NewUser {
   pub username: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Task {
   pub id: Uuid,
   pub title: String,
   pub description: String,
   pub status: TaskStatus,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct NewTask {
    pub title: String,
    pub description: String,
    pub due_date: Option<String>, // Optional field
    pub status: TaskStatus,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct UpdateTask {
    pub title: Option<String>,        
    pub description: Option<String>,  
    pub status: Option<TaskStatus>,   
}

#[derive(Serialize, Deserialize, Clone)]
pub enum TaskStatus {
   Todo,
   InProgress,
   Done,
}