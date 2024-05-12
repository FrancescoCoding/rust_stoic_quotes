use reqwest::Client;
use serde::{Deserialize, Serialize};

// Equivalent of a type in TypeScript
#[derive(Deserialize, Serialize)]
pub struct StoicQuote {
    pub author: String,
    pub quote: String,
}

// Asynchronous function to fetch a stoicism quote from an external API.
pub async fn fetch_quote() -> Result<StoicQuote, reqwest::Error> {
    let client = Client::new();
    let res = client
        .get("https://stoic.tekloon.net/stoic-quote")
        .send()
        .await?
        .json::<StoicQuote>()
        .await?;
    Ok(res)
}
