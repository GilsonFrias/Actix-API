use actix_web::{web, HttpResponse};
use crate::models::user::{User, CreateUser};
use uuid::Uuid;

pub async fn create_user(user: web::Json<CreateUser>)  -> HttpResponse {
    let new_user = User {
        id: Uuid::new_v4(),
        username: user.username.clone(),
        email: user.email.clone()
    };
    
    println!("Created new user with id: {:?}", new_user.id);
    HttpResponse::Created().json(new_user)
}

pub async fn get_user(user_id: web::Path<Uuid>) -> HttpResponse {
    let mock_user = User {
        id: *user_id,
        username: String::from("mock_user"),
        email: String::from("foo@foo.com")
    };

    println!("User retrieved successfully");

    HttpResponse::Ok().json(mock_user)
}

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json("API up and running!")
}
