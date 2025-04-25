use common::error::{ServerError, ServerResult};
use futures_util::{future, StreamExt, TryStreamExt};
use tokio::net::{TcpListener, TcpStream};
use tracing::{error, info};

async fn accept_websocket(stream: TcpStream) -> ServerResult {
  let peer_addr = stream.peer_addr()?;
  let ws_stream = tokio_tungstenite::accept_async(stream).await?;
  info!("New websocket connection at {peer_addr}");

  let (write, read) = ws_stream.split();

  read
    .try_filter(|msg| future::ready(msg.is_text() || msg.is_binary()))
    .forward(write)
    .await?;

  Ok(())
}

async fn accept_connection(stream: TcpStream) {
  if let Err(err) = accept_websocket(stream).await {
    error!("{err}");
  }
}

pub async fn run_websocket_server() -> ServerResult {
  let addr = "127.0.0.1:8081";
  let tcp_stream = TcpListener::bind(addr).await.map_err(|err| {
    ServerError::Initialization(format!(
      "Failed to bind websocket listener to address {addr}: {err}"
    ))
  })?;

  info!("Running websocket server at {addr}");

  loop {
    match tcp_stream.accept().await {
      Ok((stream, _)) => {
        tokio::spawn(accept_connection(stream));
      }
      Err(err) => {
        error!("TCP stream accept failure: {err}");
      }
    }
  }
}
