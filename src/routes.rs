use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Importing models and database connection
use crate::db::Database;
use crate::models::{User, CreateUser};

// Define API routes and handlers
async fn hello() -> impl Responder {
    "Hello World!"
}

// Handling GET request to fetch all users
#[derive(Serialize)]
struct GetAllUsersResponse {
    users: Vec<User>,
}

async fn get_all_users(db: web::Data<Database>) -> impl Responder {
    let users = db.get_all_users().await;
    match users {
        Ok(users) => web::Json(GetAllUsersResponse { users }),
        Err(err) => {
            web::HttpResponse::InternalServerError().body(format!("Error fetching users: {}", err))
        }
    }
}

// Handling POST request to create a new user
#[derive(Deserialize)]
struct CreateUserRequest {
    name: String,
    email: String,
}

async fn create_user(
    db: web::Data<Database>,
    req: web::Json<CreateUserRequest>,
) -> impl Responder {
    let new_user = CreateUser {
        name: req.name.clone(),
        email: req.email.clone(),
    };
    match db.create_user(new_user).await {
        Ok(user) => web::Json(user),
        Err(err) => {
            web::HttpResponse::InternalServerError().body(format!("Error creating user: {}", err))
        }
    }
}

// Handling GET request to fetch a user by ID
async fn get_user_by_id(
    db: web::Data<Database>,
    id: web::Path<i32>,
) -> impl Responder {
    let user = db.get_user_by_id(*id).await;
    match user {
        Ok(Some(user)) => web::Json(user),
        Ok(None) => web::HttpResponse::NotFound().body("User not found"),
        Err(err) => {
            web::HttpResponse::InternalServerError().body(format!("Error fetching user: {}", err))
        }
    }
}

// Setting up API routes
pub async fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(hello))
        .route("/users", web::get().to(get_all_users))
        .route("/users", web::post().to(create_user))
        .route("/users/{id}", web::get().to(get_user_by_id));
}