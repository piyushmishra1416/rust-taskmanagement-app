use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use std::vec;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
struct User{
    id: Uuid,
    tasks: Vec<Task>
}

#[derive(Serialize, Deserialize, Clone)]

struct Task {
    id : Uuid,
    description: String,
    status: String,
}
#[derive(Deserialize)]
struct NewUser{}

struct AppState {
    users: Mutex<HashMap<Uuid, User>>,
}


async fn create_user(state: web::Data<AppState>, _user: web::Json<NewUser>) -> impl Responder{
    let mut users = state.users.lock().unwrap();
    let new_user_id = Uuid::new_v4();
    users.insert(new_user_id, User{
        id: new_user_id,
        tasks: vec![],
    });
    HttpResponse::Created().json(users[&new_user_id].clone())

}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize shared application state
    let state = web::Data::new(AppState {
        users: Mutex::new(HashMap::new()),
    });

    // Start HTTP server and configure routes
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .route("/users", web::post().to(create_user))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}


