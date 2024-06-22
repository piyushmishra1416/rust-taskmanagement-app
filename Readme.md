# Rust Project with Actix Web

## Overview
This project is a Rust application built using the Actix Web framework. It provides RESTful APIs for managing users and tasks.

## Architecture
The application follows a typical web server architecture using Actix Web, consisting of:

- **Main Module (`main.rs`)**: Entry point for the application where the Actix Web server is configured and started.
- **Routes (`routes.rs`)**: Defines API endpoints and handlers.
- **Models (`models.rs`)**: Defines data structures used in the application.
- **Database Module (`db.rs`)**: Simulates a simple in-memory database using Rust's `HashMap`.
- **Error Handling (`errors.rs`)**: Defines custom error types and handling.
- **Tests (`tests` folder)**: Contains integration tests for APIs using Actix Web's testing utilities.

## Dependencies
- `actix-web`: Actix Web framework for building web servers in Rust.
- `uuid`: For generating UUIDs for users and tasks.
- `serde` and `serde_json`: For serializing and deserializing data.
- `tokio`: Asynchronous runtime for Rust.

## Setup
### 1. Clone the Repository:

```bash
git clone https://github.com/your/repository.git
cd rust_project
```
### 2. Install Rust:
Ensure you have Rust installed. If not, follow instructions at [rust-lang.org](rust-lang.org).

### 3. Build and Run:

```bash
cargo build 
cargo run
```
### 4. Testing:

```bash
cargo test
```
## Usage
- **Create User:**
``` bash
curl -X POST http://127.0.0.1:3000/users -d '{"username": "user1"}' -H "Content-Type: application/json"
```
- **Create Task:**
``` bash
curl -X POST http://127.0.0.1:3000/users/{user_id}/tasks -d '{"title": "Task 1", "description": "Description 1", "due_date": "2024-07-01", "status": "ToDo"}' -H "Content-Type: application/json"

```
- **List Tasks:**
``` bash
curl http://127.0.0.1:3000/users/{user_id}/tasks
```
- **Get Task:**
``` bash
curl http://127.0.0.1:3000/users/{user_id}/tasks/{task_id}

```
- **Update Task:**
``` bash
curl -X PUT http://127.0.0.1:3000/users/{user_id}/tasks/{task_id} -d '{"title": "Updated Task", "description": "Updated Description", "due_date": "2024-07-02", "status": "InProgress"}' -H "Content-Type: application/json"

```
- **Delete Task:**
``` bash
curl -X DELETE http://127.0.0.1:3000/users/{user_id}/tasks/{task_id}
```

## Design Decisions

- **Actix Web**: Chosen for its performance, scalability, and asynchronous capabilities in handling web requests.
- **UUID**: Used for generating unique identifiers for users and tasks.
- **Serde**: For serialization and deserialization of JSON data.
- **In-memory Database**: Implemented using HashMap for simplicity in the example.
