use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;

use crate::models::{Task, User, TaskStatus, NewUser, NewTask, UpdateTask};
use crate::db::{Db, UserDb};
use crate::errors::AppError;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("", web::post().to(create_user))
            .route("/{user_id}/tasks", web::post().to(create_task))
            .route("/{user_id}/tasks", web::get().to(list_tasks))
            .route("/{user_id}/tasks/{task_id}", web::get().to(get_task))
            .route("/{user_id}/tasks/{task_id}", web::put().to(update_task))
            .route("/{user_id}/tasks/{task_id}", web::delete().to(delete_task)),
    );
}

async fn create_user(
    user_db: web::Data<UserDb>,
    payload: web::Json<NewUser>,
) -> Result<impl Responder, AppError> {
    let mut users = user_db.lock().unwrap();
    let user = User {
        id: Uuid::new_v4(),
        username: payload.username.clone(),
    };
    users.insert(user.id, user.clone());
    Ok(HttpResponse::Ok().json(user))
}

async fn create_task(
    db: web::Data<Db>,
    user_db: web::Data<UserDb>,
    path: web::Path<Uuid>,
    payload: web::Json<NewTask>,
) -> Result<impl Responder, AppError> {
    let user_id = path.into_inner(); // Extract user_id from the path
    let users = user_db.lock().unwrap();
    if !users.contains_key(&user_id) {
        return Err(AppError::UserNotFound);
    }
    drop(users);
    
    if payload.title.trim().is_empty() {
        return Err(AppError::InvalidInput); // Use InvalidInput error
    }

    let mut tasks = db.lock().unwrap();
    let user_tasks = tasks.entry(user_id).or_insert_with(Vec::new);
    let task = Task {
        id: Uuid::new_v4(),
        title: payload.title.clone(),
        description: payload.description.clone(),
        status: TaskStatus::Todo,
    };
    user_tasks.push(task.clone());
    Ok(HttpResponse::Ok().json(task))
}

async fn list_tasks(
    db: web::Data<Db>,
    path: web::Path<Uuid>,
) -> Result<impl Responder, AppError> {
    let user_id = path.into_inner(); // Extract user_id from the path
    let tasks = db.lock().unwrap();
    let user_tasks = tasks.get(&user_id).ok_or(AppError::UserNotFound)?;
    Ok(HttpResponse::Ok().json(user_tasks.clone()))
}

async fn get_task(
    db: web::Data<Db>,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<impl Responder, AppError> {
    let (user_id, task_id) = path.into_inner(); // Extract user_id and task_id from the path
    let tasks = db.lock().unwrap();
    let user_tasks = tasks.get(&user_id).ok_or(AppError::UserNotFound)?;
    let task = user_tasks.iter().find(|task| task.id == task_id).ok_or(AppError::TaskNotFound)?;
    Ok(HttpResponse::Ok().json(task.clone()))
}

async fn update_task(
    db: web::Data<Db>,
    path: web::Path<(Uuid, Uuid)>,
    payload: web::Json<UpdateTask>, // Use UpdateTask instead of Task
) -> Result<impl Responder, AppError> {
    let (user_id, task_id) = path.into_inner(); // Extract user_id and task_id from the path
    let mut tasks = db.lock().unwrap();
    let user_tasks = tasks.get_mut(&user_id).ok_or(AppError::UserNotFound)?;
    let task = user_tasks.iter_mut().find(|task| task.id == task_id).ok_or(AppError::TaskNotFound)?;

    // Update the task with the provided fields
    if let Some(title) = &payload.title {
        task.title = title.clone();
    }
    if let Some(description) = &payload.description {
        task.description = description.clone();
    }
    if let Some(status) = &payload.status {
        task.status = status.clone();
    }

    Ok(HttpResponse::Ok().json(task.clone()))
}
async fn delete_task(
    db: web::Data<Db>,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<impl Responder, AppError> {
    let (user_id, task_id) = path.into_inner(); // Extract user_id and task_id from the path
    let mut tasks = db.lock().unwrap();
    let user_tasks = tasks.get_mut(&user_id).ok_or(AppError::UserNotFound)?;
    if let Some(index) = user_tasks.iter().position(|task| task.id == task_id) {
        user_tasks.remove(index);
        Ok(HttpResponse::Ok().json(()))
    } else {
        Err(AppError::TaskNotFound)
    }
}
