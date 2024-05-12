use actix_web::{web, HttpResponse, Responder};
use crate::models::{StoicQuote, fetch_quote};
use sqlx::PgPool;

// Function to configure routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/quote", web::get().to(quote))
       .route("/add_quote", web::post().to(add_quote))
       .route("/get_quotes", web::get().to(get_quotes))
       .route("/get_random_quote", web::get().to(get_random_quote));
}

// Asynchronous function that acts as a web handler; it fetches a quote and returns an HTTP response.
async fn quote() -> impl Responder {
    match fetch_quote().await {
        Ok(quote) => HttpResponse::Ok().json(quote),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// Async function to add a quote to the database
async fn add_quote(pool: web::Data<PgPool>, quote: web::Json<StoicQuote>) -> impl Responder {
    let result = sqlx::query!(
        "INSERT INTO stoic_quotes (author, quote) VALUES ($1, $2)",
        quote.author, quote.quote
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Created().finish(),
        Err(e) => {
            println!("Failed to insert quote: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

// Async function to retrieve all quotes from the database
async fn get_quotes(pool: web::Data<PgPool>) -> impl Responder {
    let result = sqlx::query_as!(StoicQuote,
        "SELECT author, quote FROM stoic_quotes"
    )
    .fetch_all(pool.get_ref())
    .await;

    match result {
        Ok(quotes) => HttpResponse::Ok().json(quotes),
        Err(e) => {
            println!("Failed to fetch quotes: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

// Async function to get a random quote from the database
async fn get_random_quote(pool: web::Data<PgPool>) -> impl Responder {
    let result = sqlx::query_as!(StoicQuote,
        "SELECT author, quote FROM stoic_quotes ORDER BY RANDOM() LIMIT 1"
    )
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(quote) => HttpResponse::Ok().json(quote),
        Err(e) => {
            println!("Failed to fetch random quote: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
