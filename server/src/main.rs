mod ws;

use ws::run_websocket_server;

#[tokio::main]
async fn main() {
  tracing_subscriber::fmt()
    .with_max_level(tracing::Level::INFO)
    .init();

  let result = run_websocket_server().await;

  if let Err(error) = result {
    tracing::error!("{error}");
  }
}
