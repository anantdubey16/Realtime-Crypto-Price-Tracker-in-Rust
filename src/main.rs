use warp::Filter;
use tokio::sync::broadcast;
use serde::{Deserialize, Serialize};
use futures_util::{StreamExt, SinkExt};
use warp::ws::Message;
use tokio::time::{self, Duration};
use std::env;
use dotenv::dotenv;

mod coin_api;
use coin_api::fetch_price;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct PriceUpdate {
    asset_id: String,
    price: f64,
}

async fn handle_connection(
    ws: warp::ws::WebSocket,
    tx: broadcast::Sender<PriceUpdate>,
) {
    let (mut ws_tx, mut ws_rx) = ws.split();
    let mut rx = tx.subscribe();

    let ws_to_tx = async move {
        while let Ok(price_update) = rx.recv().await {
            let msg = Message::text(serde_json::to_string(&price_update).unwrap());
            if ws_tx.send(msg).await.is_err() {
                break;
            }
        }
    };

    let rx_to_ws = async move {
        while ws_rx.next().await.is_some() {
            // Handle incoming messages if needed
        }
    };

    tokio::select! {
        _ = ws_to_tx => {},
        _ = rx_to_ws => {},
    }
}

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenv().ok();

    // Read API key from environment variable
    let api_key = env::var("COIN_API_KEY").expect("COIN_API_KEY must be set");

    // Asset IDs to track
    let asset_ids = vec![
        "BTC", "ETH", "XRP", "BCH", "ADA",
        "LTC", "LINK", "XLM", "DOT", "SOL"
    ];

    // Broadcast channel for price updates
    let (tx, _) = broadcast::channel(100);

    // Clone sender for each request handler
    let tx_clone = tx.clone();

    // Route for WebSocket connections
    let ws_route = warp::path("ws")
        .and(warp::ws())
        .map(move |ws: warp::ws::Ws| {
            let tx_clone = tx_clone.clone(); // Clone tx_clone for the closure
            ws.on_upgrade(move |socket| handle_connection(socket, tx_clone.clone()))
        });

    // Combine routes
    let routes = ws_route;

    // Start periodic price fetching for each asset ID
    let price_fetch_tasks: Vec<_> = asset_ids.into_iter().map(|asset_id| {
        let api_key = api_key.to_string();
        let tx = tx.clone();
        tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_secs(5)); // Fetch every 5 seconds
            loop {
                interval.tick().await;
                match fetch_price(&api_key, &asset_id).await {
                    Ok(price) => {
                        let price_update = PriceUpdate {
                            asset_id: asset_id.to_string(),
                            price,
                        };
                        // Send price update through the broadcast channel
                        if let Err(e) = tx.send(price_update.clone()) {
                            eprintln!("Price update error: {:?}", e);
                        } else {
                            println!("Sent price update: {:?}", price_update);
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to fetch price for {}: {:?}", asset_id, e);
                    }
                }
            }
        })
    }).collect();

    // Spawn tasks to fetch prices concurrently
    for task in price_fetch_tasks {
        task.await.unwrap();
    }

    println!("Starting the server...");
    // Start the server
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
