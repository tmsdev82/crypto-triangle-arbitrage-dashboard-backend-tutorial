use crate::{
    config::AppConfig,
    models::{self, DepthStreamWrapper},
    Clients,
};
use log::{debug, error, info};
use std::collections::HashMap;
use tokio::time::{Duration, Instant};

use tungstenite::{client::AutoStream, WebSocket};
use warp::ws::Message;

pub async fn main_worker(clients: Clients, config: AppConfig, mut socket: WebSocket<AutoStream>) {
    let mut pairs_data: HashMap<String, DepthStreamWrapper> = HashMap::new();
    let mut interval_timer = Instant::now();
    loop {
        // tokio::time::sleep(Duration::from_millis(100)).await;

        let connected_client_count = clients.lock().await.len();
        if connected_client_count == 0 {
            tokio::time::sleep(Duration::from_millis(100)).await;
            debug!("No clients connected, skip sending data");
            // continue;
        }

        let msg = socket.read_message().expect("Error reading message");
        let msg = match msg {
            tungstenite::Message::Text(s) => s,
            tungstenite::Message::Ping(p) => {
                info!("Ping message received! {:?}", p);
                // send_pong(&mut socket, p);
                continue;
            }
            tungstenite::Message::Pong(p) => {
                info!("Pong received: {:?}", p);
                continue;
            }
            _ => {
                error!("Error getting text: {:?}", msg);
                continue;
            }
        };

        info!("msg: {}", msg);
        let parsed: models::DepthStreamWrapper = serde_json::from_str(&msg).expect("Can't parse");

        let pair_key = parsed.stream.split_once("@").unwrap().0;
        pairs_data.insert(pair_key.to_string(), parsed);

        if interval_timer.elapsed().as_millis() < 105 {
            // debug!("skip processing");
            continue;
        }

        let data_copy = pairs_data.clone();
        let triangles = config.triangles.to_vec();
        let cclients = clients.clone();
        tokio::task::spawn(async move {
            for triangle_config in triangles.iter() {
                process_triangle_data(
                    &data_copy,
                    &triangle_config.pairs[0],
                    &triangle_config.pairs[1],
                    &triangle_config.pairs[2],
                    [
                        &triangle_config.parts[0],
                        &triangle_config.parts[1],
                        &triangle_config.parts[2],
                    ],
                    cclients.clone(),
                )
                .await;
            }
        });

        interval_timer = Instant::now();
    }
}

async fn process_triangle_data(
    pairs_data: &HashMap<String, DepthStreamWrapper>,
    start_pair: &str,
    mid_pair: &str,
    end_pair: &str,
    triangle: [&str; 3],
    clients: Clients,
) {
    info!(
        "processing triangle {:?}: {}->{}->{}",
        triangle, start_pair, mid_pair, end_pair
    );

    let data = (
        pairs_data.get(start_pair),
        pairs_data.get(mid_pair),
        pairs_data.get(end_pair),
    );

    let (start_pair_data, mid_pair_data, end_pair_data) = match data {
        (Some(s), Some(m), Some(e)) => (s, m, e),
        _ => {
            info!(
                "{:?} One or more of the pairs were not found, skipping",
                (start_pair, mid_pair, end_pair)
            );
            return;
        }
    };

    let mut profits: Vec<f64> = Vec::new();

    for i in 0..start_pair_data.data.asks.len() {
        let mut triangle_profit = calc_triangle_step(
            1.0,
            start_pair_data.data.asks[i].price,
            start_pair_data.data.bids[i].price,
            start_pair,
            triangle[0],
        );
        triangle_profit = calc_triangle_step(
            triangle_profit,
            mid_pair_data.data.asks[i].price,
            mid_pair_data.data.bids[i].price,
            mid_pair,
            triangle[1],
        );
        triangle_profit = calc_triangle_step(
            triangle_profit,
            end_pair_data.data.asks[i].price,
            end_pair_data.data.bids[i].price,
            end_pair,
            triangle[2],
        );

        let norm_profit = triangle_profit - 1.0;
        profits.push(norm_profit);
        if norm_profit > 0.0 {
            info!(target: "profit", "{:?} positive profit: {:.5}% ({} {})", triangle, (norm_profit*100.0), norm_profit, triangle[0]);
        }
    }

    info!("{:?} potential profits: {:?}", triangle, profits);
    let triangle_data = models::TriangleArbitrageData {
        start_pair_data: start_pair_data.clone(),
        mid_pair_data: mid_pair_data.clone(),
        end_pair_data: end_pair_data.clone(),
        profits,
        triangle: [
            triangle[0].to_string(),
            triangle[1].to_string(),
            triangle[2].to_string(),
        ],
    };

    // clients.lock().await.iter().for_each(|(_, client)| {
    //     if let Some(sender) = &client.sender {
    //         let _ = sender.send(Ok(Message::text(
    //             serde_json::to_string(&triangle_data).unwrap(),
    //         )));
    //     }
    // });
}

fn calc_triangle_step(
    trade_amount: f64,
    ask_price: f64,
    bid_price: f64,
    pair_name: &str,
    triangle_part: &str,
) -> f64 {
    // subtract trading fee
    let trade_amount = trade_amount - ((trade_amount / 100.0) * 0.075);
    // Compare first part of the part to the part of the triangle
    // to determine on what side of the trade we should be
    if pair_name[..triangle_part.len()] == *triangle_part {
        // sell side
        trade_amount * bid_price
    } else {
        // buy side
        trade_amount / ask_price
    }
}
