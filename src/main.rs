use actix_cors::Cors;
use actix_web::{http::header, middleware::Logger, App, HttpServer};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use soccer_meet_actix::routes::health_route::health_check_handler;

struct AppState {
    _db: Pool<Postgres>,
}

fn config(cfg: &mut actix_web::web::ServiceConfig) {
    // Configure your routes here
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    dotenv().ok();
    env_logger::init();

    let database_url: String = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool:Pool<Postgres> = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("Connected to Postgres is successful");
            pool
        }
        Err(e) => {
            eprintln!("Failed to connect to Postgres: {}", e);
            std::process::exit(1);
        }
    };

    let _ = HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT
            ])
            .supports_credentials();
    
        App::new()
            .app_data(actix_web::web::Data::new(AppState { db: pool.clone() }))
            .service(health_check_handler)
            .configure(config)
            .wrap(cors)
            .wrap(Logger::default())
    })
    .bind(&std::env::var("BASE_URL").expect("BASE_URL must be set"))?
    .run()
    .await;

    println!("server started successfully");
    Ok(())
}