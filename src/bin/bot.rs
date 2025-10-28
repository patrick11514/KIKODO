use clap::Parser;
use dotenv::dotenv;
use kikodo::{
    bot::{bot::create_bot, http::run_server},
    cli::Args,
};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    tracing::info!("Starting Kikodo bot...");

    let args = Args::parse();
    let http_server = tokio::spawn(async move { run_server(&args.host, args.port).await });

    let bot = create_bot(args);

    tokio::select! {
         server = http_server => {
            match server? {
                Ok(_) => {
                    tracing::warn!("HTTP server has stopped running.");
                },
                Err(e) =>  {
                    tracing::error!("HTTP server task failed: {}", e);
                },
            }
         }
    }

    Ok(())
}
