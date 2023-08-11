use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;
use log::{error, info};
use std::env;

mod handlers;
mod utils;
mod validations;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let environment = env::var("ENV").unwrap_or_else(|_| "local".to_string());
    let host_var = format!("HOST_{}", environment.to_uppercase());
    let port_var = format!("PORT_{}", environment.to_uppercase());

    // Retrieve the host from the environment variable
    let host = env::var(&host_var).unwrap_or_else(|_| "127.0.0.1".to_string()); // Default to localhost

    // Retrieve the port from the environment variable
    let port: u16 = env::var(&port_var)
        .unwrap_or_else(|_| "8080".to_string()) // Default to port 8080
        .parse()
        .expect("PORT must be a valid number");

    info!("Starting server at http://{}:{}", host, port);

    let client = utils::stripe::initialise_client().await;
    info!("Initialised Stripe Client");

    // Start the Actix Web server
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .max_age(3600),
            )
            .service(
                web::scope("/api/payments")
                    .app_data(client.clone())
                    .route(
                        "/create_customer",
                        web::post().to(handlers::create_customer::create_customer),
                    )
                    .route(
                        "/create_subscription",
                        web::post().to(handlers::create_product::create_product),
                    ),
            )
    })
    .bind((host, port)) // Bind to the host and port determined by the environment
    .map_err(|e| {
        error!("Failed to bind server: {}", e);
        e
    })?
    .run()
    .await
}
