# Rust Crypto Tracker

Rust Crypto Tracker is a real-time cryptocurrency price monitoring server built with Rust. This project uses the CoinAPI to fetch prices for multiple cryptocurrencies and broadcasts these updates to connected clients via WebSockets.

![image](https://github.com/anantdubey16/Realtime-Crypto-Price-Tracker-in-Rust/assets/81023294/8830b2c0-7f0b-42cb-9640-4122df1eab21)


## Features

- Real-time cryptocurrency price updates
- Supports multiple cryptocurrencies
- WebSocket server to push updates to clients
- Asynchronous and concurrent operations with Tokio
- Uses Warp for the WebSocket server

## Prerequisites

- Rust (latest stable version)
- CoinAPI key (get it from [CoinAPI](https://docs.coinapi.io/))

## Project Structure


## Setup and Installation

1. **Clone the repository**:
    ```sh
    git clone https://github.com/anantdubey16/rust_crypto_tracker.git
    cd rust_crypto_tracker
    ```

2. **Set your CoinAPI key**:
    Replace `your_api_key` in `src/main.rs` with your actual CoinAPI key.
    ```rust
    let api_key = "your_api_key";
    ```

3. **Build the project**:
    ```sh
    cargo build
    ```

4. **Run the server**:
    ```sh
    cargo run
    ```

5. **Connect to WebSocket server**:
    Open a WebSocket client and connect to `ws://127.0.0.1:3030/ws` to receive real-time price updates.

## Code Explanation

### `src/main.rs`

This file contains the main logic of the server:
- **Imports and dependencies**: Includes necessary libraries and modules.
- **PriceUpdate struct**: Defines the structure for price updates.
- **handle_connection function**: Manages WebSocket connections and message broadcasting.
- **main function**: Initializes the server, sets up WebSocket routes, and spawns tasks for fetching prices.

### `src/coin_api.rs`

This file handles communication with the CoinAPI:
- **CoinApiResponse struct**: Deserializes API responses.
- **fetch_price function**: Fetches prices from CoinAPI for given asset IDs.

## Usage

- The server fetches prices for multiple cryptocurrencies (BTC, ETH, XRP, BCH, ADA, LTC, LINK, XLM, DOT, SOL) every 5 seconds.
- Clients connected to the WebSocket server will receive real-time updates in JSON format:
  ```json
  {
      "asset_id": "BTC",
      "price": 45000.0
  }
