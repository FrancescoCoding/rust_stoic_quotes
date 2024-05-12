use actix_web::{web, App, HttpServer};
use sqlx::postgres::PgPoolOptions;
mod handlers;
mod models;

// Main function to run the web server.
#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string()); // Default to port 8080 if not set
    let address = format!("0.0.0.0:{}", port);

    // DATABASE_URL must be set in the environment variables
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // Set up a connection pool to the PostgreSQL database
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool.");

    // Configure and run the HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone())) // Share the connection pool across threads
            .configure(handlers::configure_routes) // Include the route configuration
    })
    .bind(address)? // Bind server to the specified address
    .run()
    .await
}
