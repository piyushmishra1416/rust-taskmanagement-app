use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone,Debug)]
pub struct User{
   pub id: Uuid,
   pub username: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct NewUser {
   pub username: String,
}

#[derive(Serialize, Deserialize, Clone,Debug)]
pub struct Task {
   pub id: Uuid,
   pub title: String,
   pub description: String,
   pub status: TaskStatus,
}
#[derive(Clone, Serialize, Deserialize,PartialEq,Debug)]
pub struct NewTask {
    pub title: String,
    pub description: String,
    pub due_date: Option<String>, // Optional field
    pub status: TaskStatus,
}

#[derive(Clone, Serialize, Deserialize,PartialEq,Debug)]
pub struct UpdateTask {
    pub title: Option<String>,        
    pub description: Option<String>,  
    pub status: Option<TaskStatus>,   
}

#[derive(Serialize, Deserialize, Clone,PartialEq,Debug)]
pub enum TaskStatus {
   Todo,
   InProgress,
   Done,
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_create_user() {
        let user = User {
            id: Uuid::new_v4(),
            username: String::from("testuser"),
        };
        assert_eq!(user.username, "testuser");
    }

    #[test]
    fn test_create_task() {
        let task = Task {
            id: Uuid::new_v4(),
            title: String::from("Test Task"),
            description: String::from("Test Description"),
            status: TaskStatus::Todo,
        };
        assert_eq!(task.title, "Test Task");
        assert_eq!(task.description, "Test Description");
        assert_eq!(task.status, TaskStatus::Todo);
    }

    #[test]
    fn test_update_task() {
        let mut task = Task {
            id: Uuid::new_v4(),
            title: String::from("Test Task"),
            description: String::from("Test Description"),
            status: TaskStatus::Todo,
        };

        let update = UpdateTask {
            title: Some(String::from("Updated Task")),
            description: Some(String::from("Updated Description")),
            status: Some(TaskStatus::InProgress),
        };

        if let Some(title) = update.title {
            task.title = title;
        }
        if let Some(description) = update.description {
            task.description = description;
        }
        if let Some(status) = update.status {
            task.status = status;
        }

        assert_eq!(task.title, "Updated Task");
        assert_eq!(task.description, "Updated Description");
        assert_eq!(task.status, TaskStatus::InProgress);
    }
}
