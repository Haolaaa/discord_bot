use crate::error::BotError;
use tokio::signal::{
    ctrl_c,
    unix::{SignalKind, signal},
};

mod audio;
mod client;
mod commands;
mod consts;
mod error;
mod events;
mod guild_state;
mod queue;
mod telemetry;
pub mod utils;

#[tokio::main]
async fn main() -> Result<(), BotError> {
    let _guard = telemetry::init_log();

    if let Err(e) = dotenv::dotenv() {
        tracing::error!("Unable to find .env configuration file: {}", e);
    }

    let mut client = client::get().await?;

    let shard_manager = client.shard_manager.clone();
    let mut sigterm = signal(SignalKind::terminate())?;

    tokio::select! {
        result = client.start() => result?,
        _ = sigterm.recv() => {
            client::handle_shutdown(shard_manager, "Received SIGTERM").await;
            println!("Everything is shutdown. GoodBye!");
            std::process::exit(0)
        },
        _ = ctrl_c() => {
            client::handle_shutdown(shard_manager, "Interrupted").await;
            println!("Everything is shutdown. GoodBye!");
            std::process::exit(0)
        }
    }

    Ok(())
}
