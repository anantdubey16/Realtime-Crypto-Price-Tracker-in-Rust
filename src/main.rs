use warp::Filter;
use tokio::sync::broadcast;
use serde::{Deserialize, Serialize};
use futures_util::{StreamExt, SinkExt};
use warp::ws::Message;
use tokio::time::{self, Duration};

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
    // API key
    let api_key = "EDACCDA1-BCA9-4565-9F7B-32CEB779A524";

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

    // Start periodic price fetching
    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(5));
        let asset_id = "BTC"; // Replace with the desired asset ID
        loop {
            interval.tick().await;
            if let Ok(price) = fetch_price(&api_key, asset_id).await {
                let price_update = PriceUpdate {
                    asset_id: asset_id.to_string(),
                    price,
                };
                // Send price update through the broadcast channel
                if let Err(e) = tx.send(price_update.clone()) {
                    eprintln!("Price update: {:?}", e);
                }
            }
        }
    });

    // Start the server
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
