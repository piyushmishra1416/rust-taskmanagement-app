use actix_web::{web, http::StatusCode, test, App};
use rust_project::routes::configure;
use rust_project::db::init_db;
use serde_json::json;

#[actix_rt::test]
async fn test_create_user() {
    let (task_db, user_db) = init_db();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(task_db.clone()))
            .app_data(web::Data::new(user_db.clone()))
            .configure(configure),
    )
    .await;

    let new_user = json!({
        "username": "testuser"
    });

    let req = test::TestRequest::post()
        .uri("/users")
        .set_json(&new_user)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let user: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(user["username"], "testuser");
}

#[actix_rt::test]
async fn test_create_task() {
    let (task_db, user_db) = init_db();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(task_db.clone()))
            .app_data(web::Data::new(user_db.clone()))
            .configure(configure),
    )
    .await;

    // First, create a user
    let new_user = json!({
        "username": "testuser"
    });

    let req = test::TestRequest::post()
        .uri("/users")
        .set_json(&new_user)
        .to_request();

    let resp = test::call_service(&app, req).await;
    let user: serde_json::Value = test::read_body_json(resp).await;
    let user_id = user["id"].as_str().unwrap();

    // Now, create a task for the user
    let new_task = json!({
        "title": "Test Task",
        "description": "Test Description",
        "status": "Todo"
    });

    let req = test::TestRequest::post()
        .uri(&format!("/users/{}/tasks", user_id))
        .set_json(&new_task)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let task: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(task["title"], "Test Task");
}

#[actix_rt::test]
async fn test_list_tasks() {
    let (task_db, user_db) = init_db();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(task_db.clone()))
            .app_data(web::Data::new(user_db.clone()))
            .configure(configure),
    )
    .await;

    // First, create a user
    let new_user = json!({
        "username": "testuser"
    });

    let req = test::TestRequest::post()
        .uri("/users")
        .set_json(&new_user)
        .to_request();

    let resp = test::call_service(&app, req).await;
    let user: serde_json::Value = test::read_body_json(resp).await;
    let user_id = user["id"].as_str().unwrap();

    // Create a task for the user
    let new_task = json!({
        "title": "Test Task",
        "description": "Test Description",
        "status": "Todo"
    });

    let req = test::TestRequest::post()
        .uri(&format!("/users/{}/tasks", user_id))
        .set_json(&new_task)
        .to_request();

    test::call_service(&app, req).await;

    // List tasks for the user
    let req = test::TestRequest::get()
        .uri(&format!("/users/{}/tasks", user_id))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let tasks: serde_json::Value = test::read_body_json(resp).await;
    assert!(tasks.as_array().unwrap().len() > 0);
}

#[actix_rt::test]
async fn test_get_task() {
    let (task_db, user_db) = init_db();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(task_db.clone()))
            .app_data(web::Data::new(user_db.clone()))
            .configure(configure),
    )
    .await;

    // First, create a user
    let new_user = json!({
        "username": "testuser"
    });

    let req = test::TestRequest::post()
        .uri("/users")
        .set_json(&new_user)
        .to_request();

    let resp = test::call_service(&app, req).await;
    let user: serde_json::Value = test::read_body_json(resp).await;
    let user_id = user["id"].as_str().unwrap();

    // Create a task for the user
    let new_task = json!({
        "title": "Test Task",
        "description": "Test Description",
        "status": "Todo"
    });

    let req = test::TestRequest::post()
        .uri(&format!("/users/{}/tasks", user_id))
        .set_json(&new_task)
        .to_request();

    let resp = test::call_service(&app, req).await;
    let task: serde_json::Value = test::read_body_json(resp).await;
    let task_id = task["id"].as_str().unwrap();

    // Get the task
    let req = test::TestRequest::get()
        .uri(&format!("/users/{}/tasks/{}", user_id, task_id))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let fetched_task: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(fetched_task["title"], "Test Task");
}

#[actix_rt::test]
async fn test_update_task() {
    let (task_db, user_db) = init_db();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(task_db.clone()))
            .app_data(web::Data::new(user_db.clone()))
            .configure(configure),
    )
    .await;

    // First, create a user
    let new_user = json!({
        "username": "testuser"
    });

    let req = test::TestRequest::post()
        .uri("/users")
        .set_json(&new_user)
        .to_request();

    let resp = test::call_service(&app, req).await;
    let user: serde_json::Value = test::read_body_json(resp).await;
    let user_id = user["id"].as_str().unwrap();

    // Create a task for the user
    let new_task = json!({
        "title": "Test Task",
        "description": "Test Description",
        "status": "Todo"
    });

    let req = test::TestRequest::post()
        .uri(&format!("/users/{}/tasks", user_id))
        .set_json(&new_task)
        .to_request();

    let resp = test::call_service(&app, req).await;
    let task: serde_json::Value = test::read_body_json(resp).await;
    let task_id = task["id"].as_str().unwrap();

    // Update the task
    let updated_task = json!({
        "title": "Updated Task",
        "description": "Updated Description",
        "status": "InProgress"
    });

    let req = test::TestRequest::put()
        .uri(&format!("/users/{}/tasks/{}", user_id, task_id))
        .set_json(&updated_task)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let fetched_task: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(fetched_task["title"], "Updated Task");
    assert_eq!(fetched_task["description"], "Updated Description");
    assert_eq!(fetched_task["status"], "InProgress");
}

#[actix_rt::test]
async fn test_delete_task() {
    let (task_db, user_db) = init_db();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(task_db.clone()))
            .app_data(web::Data::new(user_db.clone()))
            .configure(configure),
    )
    .await;

    // First, create a user
    let new_user = json!({
        "username": "testuser"
    });

    let req = test::TestRequest::post()
        .uri("/users")
        .set_json(&new_user)
        .to_request();

    let resp = test::call_service(&app, req).await;
    let user: serde_json::Value = test::read_body_json(resp).await;
    let user_id = user["id"].as_str().unwrap();

    // Create a task for the user
    let new_task = json!({
        "title": "Test Task",
        "description": "Test Description",
        "status": "Todo"
    });

    let req = test::TestRequest::post()
        .uri(&format!("/users/{}/tasks", user_id))
        .set_json(&new_task)
        .to_request();

    let resp = test::call_service(&app, req).await;
    let task: serde_json::Value = test::read_body_json(resp).await;
    let task_id = task["id"].as_str().unwrap();

    // Delete the task
    let req = test::TestRequest::delete()
        .uri(&format!("/users/{}/tasks/{}", user_id, task_id))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    // Verify the task has been deleted
    let req = test::TestRequest::get()
        .uri(&format!("/users/{}/tasks/{}", user_id, task_id))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}
