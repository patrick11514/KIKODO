use clap::Parser;

#[derive(Parser, Debug, Clone)]
pub struct Args {
    #[arg(long, env)]
    pub github_app_id: String,
    #[arg(long, env)]
    pub github_client_id: String,
    #[arg(long, env)]
    pub github_client_secret: String,
    #[arg(long, env)]
    pub github_private_key_path: String,
    #[arg(long, default_value_t = String::from("0.0.0.0"))]
    pub host: String,
    #[arg(long, default_value_t = 5173)]
    pub port: u16,
}
