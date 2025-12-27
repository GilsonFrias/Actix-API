mod handlers;
mod models;
mod routes;

use actix_web::{App, HttpServer};
use actix_web::http::{StatusCode};
use sqlx::postgres::PgPoolOptions;
use sqlx::Row;
use dotenv::dotenv;
use std::env;
use log::{info, error, debug};

use routes::user_routes::init;
use crate::handlers::user_handler::{create_user, health_check};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    dotenv().ok();
    //Get environment variables values
    //TODO: env variables are not being read, fix it
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    info!("Lifting service to run at http://{:?}:{:?}", host, port);
   

    let health = health_check().await;
    info!("Health: {:?}", health.status());
    info!("Health Status: {:?}", StatusCode::OK);
    match health.status() {
        StatusCode::OK => {
            info!("RS is Ok");
        },
        StatusCode::BAD_REQUEST => {
            info!("Bad RS");
        },
        _ => {
            info!("Unkown error");
        }
    }
    println!("Hello, world!");
    info!("started app");

    HttpServer::new(|| {
        App::new()
            .configure(init)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
   
    /*
    */
}
