use axum::{
    Json, Router,
    routing::{get, post},
    serve,
};
use tokio::net::TcpListener;

pub mod server;
mod structs;

pub async fn run_server(host: &str, port: u16) -> anyhow::Result<()> {
    let router: Router = Router::new()
        .route("/", get(slash))
        .route("/webhook", post(webhook));

    let listener = TcpListener::bind((host, port)).await?;
    serve(listener, router).await.unwrap();

    Ok(())
}

async fn slash() -> &'static str {
    "Hello, this is endpoint of KIKODO's 🐶 bot!"
}

async fn webhook(Json(event): Json<structs::WebhookEvent>) -> &'static str {
    tracing::info!("AHOJ");
    println!("Received a webhook!");
    println!("Body: {:?}", event);

    "This is the webhook endpoint!"
}
