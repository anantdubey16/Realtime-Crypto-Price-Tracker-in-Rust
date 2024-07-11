use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize)]
struct CoinApiResponse {
    rate: f64,
}

pub async fn fetch_price(api_key: &str, asset_id: &str) -> Result<f64, Error> {
    let url = format!(
        "https://rest.coinapi.io/v1/exchangerate/{}/USD",
        asset_id
    );
    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("X-CoinAPI-Key", api_key)
        .send()
        .await?;
    
    if response.status().is_success() {
        let coin_response = response.json::<CoinApiResponse>().await?;
        Ok(coin_response.rate)
    } else {
        Err(Error::from(response.error_for_status().unwrap_err()))
    }
}
