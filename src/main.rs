use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
// Reqwest client for making HTTP requests.
use reqwest::Client;

// Equivalent of a type in TypeScript
#[derive(Deserialize, Serialize)]
struct StoicQuote {
    author: String,
    quote: String,
}

// Asynchronous function to fetch a stoicism quote from an external API.
async fn fetch_quote() -> Result<StoicQuote, reqwest::Error> {
    // Initialize HTTP client
    let client = Client::new();
    // Perform a GET request and await the response, converting it directly into the StoicQuote struct.
    let res = client.get("https://stoic.tekloon.net/stoic-quote")
        .send()
        .await?
        .json::<StoicQuote>()
        .await?;
    Ok(res)
}

// Asynchronous function that acts as a web handler; it fetches a quote and returns an HTTP response.
async fn quote() -> impl Responder {
    match fetch_quote().await {
        // Return HTTP 200 OK with the quote in JSON format if successful.
        Ok(quote) => HttpResponse::Ok().json(quote),
        // Return HTTP 500 Internal Server Error if the fetch operation fails.
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// Main function to run the web server.
#[tokio::main]
async fn main() -> std::io::Result<()> {

    let port = std::env::var("PORT").unwrap_or("8080".to_string());
    println!("Using port: {}", port);
    
    let address: String = format!("0.0.0.0:{}", port);

    // Set up and run an HTTP server
    HttpServer::new(|| {
        App::new()
            // Define a single route that responds to GET requests on the /quote path.
            .route("/quote", web::get().to(quote))
    })
    // Bind the server to the
    .bind(address)?
    // Start the server and await its completion.
    .run()
    .await
}
