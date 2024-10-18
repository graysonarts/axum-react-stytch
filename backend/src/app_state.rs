use std::sync::Arc;

use envconfig::Envconfig as _;
use stytch::consumer::client::Client;
use tokio::sync::Mutex;

use crate::infra::config::Config;

pub struct InnerAppState {
    pub config: Config,
    pub stytch: Client,
}
pub type AppState = Arc<Mutex<InnerAppState>>;

pub async fn init() -> Result<AppState, anyhow::Error> {
    let config = Config::init_from_env()?;
    let client = Client::new(&config.stytch_project_id, &config.stytch_secret)?;
    Ok(Arc::new(Mutex::new(InnerAppState {
        config,
        stytch: client,
    })))
}
