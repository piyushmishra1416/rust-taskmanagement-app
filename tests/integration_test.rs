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
