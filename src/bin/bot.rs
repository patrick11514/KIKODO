use clap::Parser;
use dotenv::dotenv;
use kikodo::{
    bot::{
        bot::{bot_loop, create_bot},
        http::run_server,
    },
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
    let bot = create_bot(args.clone())?;

    let http_server = tokio::spawn(async move { run_server(&args.host, args.port, &bot).await });
    let bot_loop = tokio::spawn(bot_loop(bot));

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
         },
         bot = bot_loop => {
            match bot? {
                Ok(_) => {
                    tracing::warn!("Bot loop has stopped running.");
                },
                Err(e) =>  {
                    tracing::error!("Bot loop task failed: {}", e);
                },
            }
         }
    }

    Ok(())
}
