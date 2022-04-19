mod config;
mod hello_route;
mod json_func;
mod models;

extern crate env_logger;

use crate::config::AppConfig;
use crate::hello_route::hello_world_route;
use crate::json_func::{json_add_name, json_clear, json_get};
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let config = AppConfig::from_env();
    println!(
        "Server is starting at http://{}:{}",
        config.server_config.host, config.server_config.port
    );

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(hello_world_route)
            .service(json_get)
            .service(json_add_name)
            .service(json_clear)
    })
    .bind((config.server_config.host, config.server_config.port))?
    .run()
    .await
}
