mod handlers;
mod models;
use sqlx::postgres::PgPoolOptions;
use sqlx::Row;

use crate::handlers::user_handler::{create_user, health_check};

fn main() {
    //create_user();
    health_check();
    println!("Hello, world!");
}
