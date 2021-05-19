// This program connects to node1 on 172.18.0.11:9042 and prints outs all logs
// Should be run using RUST_LOG=trace cargo run

use scylla::{Session, SessionBuilder};
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let uri = "172.18.0.11:9042";
    info!("Connecting to {}", uri);

    let _session: Session = SessionBuilder::new().known_node(uri).build().await.unwrap();

    tokio::time::sleep(std::time::Duration::from_secs(100000000)).await;
}