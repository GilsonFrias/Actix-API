mod handlers;
mod models;

use crate::handlers::user_handler::{create_user, health_check};

fn main() {
    //create_user();
    health_check();
    println!("Hello, world!");
}
