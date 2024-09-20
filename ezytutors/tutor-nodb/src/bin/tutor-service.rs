/**
 * Application state init and register 
 */
use actix_web::{web, App, HttpServer};
use std::io;
use std::sync::Mutex;

#[path = "../handler.rs"]
mod handlers;
#[path = "../routes.rs"]
mod routes;
#[path = "../state.rs"]
mod state;

use routes::*;
use state::AppState;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    // Reset the application state 
    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm good. You've already asked me ".to_string(),
        visit_count: Mute::new(0),
    });

    // Define web application
    let app = move || {
        App::new()
            .app_data(shared_data.clone()) // Register application state to web application
            .configure(general_routes) // Construct route to web application
    };

    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await // 
}
