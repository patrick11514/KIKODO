use std::fs;

use anyhow::Context;
use tokio::sync::mpsc;

pub struct Bot {
    pub sender: mpsc::Sender<()>,
    pub receiver: mpsc::Receiver<()>,
    pub octocrab: octocrab::Octocrab,
}

pub fn create_bot(args: crate::cli::Args) -> anyhow::Result<Bot> {
    let (sender, receiver) = mpsc::channel(100);

    let key =
        fs::read(args.github_private_key_path).context("Failed to read GitHub private key")?;
    let key = jsonwebtoken::EncodingKey::from_rsa_pem(key.as_slice())
        .context("Failed to parse GitHub private key")?;

    let octocrab = octocrab::Octocrab::builder()
        .app(
            args.github_app_id
                .parse::<u64>()
                .context("Failed to parse app id")?
                .into(),
            key,
        )
        .build()?;

    Ok(Bot {
        sender,
        receiver,
        octocrab,
    })
}

pub async fn bot_loop(mut bot: Bot) -> anyhow::Result<()> {
    loop {
        if let Some(value) = bot.receiver.recv().await {
            println!("Got value");
        }
    }

    Ok(())
}
