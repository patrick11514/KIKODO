use std::fs;

use tokio::sync::mpsc;

pub struct Bot {
    pub sender: mpsc::Sender<()>,
    pub receiver: mpsc::Receiver<()>,
    pub octocrab: octocrab::Octocrab,
}

pub fn create_bot(args: crate::cli::Args) -> Bot {
    let (sender, receiver) = mpsc::channel(100);

    let key = fs::read(args.github_private_key_path)?;
    let key = jsonwebtoken::EncodingKey::from_rsa_pem(key)?;

    let octocrab = octocrab::Octocrab::builder()
        .app(args.github_app_id, key)
        .build()?;

    Bot {
        sender,
        receiver,
        octocrab,
    }
}

pub async fn bot_loop(mut bot: Bot) {
    loop {
        if Some(value) = bot.receiver.recv().await {}
    }
}
