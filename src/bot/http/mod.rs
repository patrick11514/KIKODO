use axum::{
    Json, Router,
    http::StatusCode,
    routing::{get, post},
    serve,
};
use tokio::net::TcpListener;

mod structs;
use structs::WebhookEvent;

pub async fn run_server(host: &str, port: u16) -> anyhow::Result<()> {
    let router: Router = Router::new()
        .route("/", get(slash))
        .route("/webhook", post(webhook));

    tracing::info!("Starting HTTP server on {}:{}", host, port);

    let listener = TcpListener::bind((host, port)).await?;
    serve(listener, router).await.unwrap();

    Ok(())
}

async fn slash() -> &'static str {
    "Hello, this is endpoint of KIKODO's 🐶 bot!"
}

async fn webhook(Json(event): Json<WebhookEvent>) -> StatusCode {
    match &event {
        WebhookEvent::PullRequest(pr) => {
            tracing::span!(tracing::Level::INFO, "Pull Request", ?pr);
        }
    }

    tracing::info!("Received webhook event: {:?}", event);

    StatusCode::OK
}
