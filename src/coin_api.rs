use reqwest::Client;
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
struct CoinAPIResponse {
    rate: f64,
}

pub async fn fetch_price(api_key: &str, asset_id: &str) -> Result<f64, Box<dyn Error>> {
    let client = Client::new();
    let url = format!("https://rest.coinapi.io/v1/exchangerate/{}/USD", asset_id);
    let resp = client
        .get(&url)
        .header("X-CoinAPI-Key", api_key)
        .send()
        .await?
        .json::<CoinAPIResponse>()
        .await?;
    
    Ok(resp.rate)
}
