use crate::models::Greeting;
use actix_web::{get, HttpResponse, Responder};

#[get("/hello")]
pub async fn hello_world_route() -> impl Responder {
    HttpResponse::Ok().json(Greeting {
        name: "Daniel".to_string(),
        greeting: "Good afternoon".to_string(),
    })
}
