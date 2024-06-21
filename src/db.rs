use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use crate::models::{User, Task};

pub type Db= Arc<Mutex<HashMap<Uuid, Vec<Task>>>>;
pub type UserDb = Arc<Mutex<HashMap<Uuid, User>>>;

pub fn init_db() -> (Db, UserDb) {
   let task_db = Arc::new(Mutex::new(HashMap::new()));
   let user_db = Arc::new(Mutex::new(HashMap::new()));
   (task_db, user_db)
} 