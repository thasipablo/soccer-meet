use actix_web::{get, HttpResponse, Responder};

#[get("/api/health_checker")]
pub async fn health_check_handler() -> impl Responder {
    const MESSAGE: &str = "Build Simple RESTful API with Rust, Actix-web, and PostgreSQL";
    HttpResponse::Ok().json(json!({ "message": MESSAGE }))
}