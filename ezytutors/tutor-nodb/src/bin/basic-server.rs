/**
 * 1. Router
 * 2. Handler
 * 3. Register Router & Handler to application
 * 4. Construct HTTP server, execute server
*/
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::io;

// 1. Route
// Pass HTTP GET request to health_check_handler
pub fn general_routes(cfg: &mut web::ServiceConfig) {
    /**
     * 1. /health -> route
     * 2. web::get() -> HTTP method
     * 3. .to(health_check_handler) -> Request Handler Method
     */
    cfg.route("/health", web::get().to(health_check_handler));

}

// 2. Handler
// create HTTP Request
pub async fn health_check_handler() -> impl Responder {
    HttpResponse::Ok().json("Hello. EzyTutors is alive and kicking")
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    // Create an Actix web application instance and register to the path
    let app = move || App::new().configure(general_routes);
    // Init the webserver, load app, and binding to socket and execute server. 
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await 
}