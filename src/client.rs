use crate::{commands, error::BotError, events, guild_state::GuildPlayer};
use dashmap::DashMap;
use poise::{
    EditTracker, Framework, FrameworkOptions, PrefixFrameworkOptions,
    serenity_prelude::{self as serenity},
};
use reqwest::Client as HttpClient;
use songbird::{
    SerenityInit,
    driver::{DecodeConfig, DecodeMode},
};
use std::{sync::Arc, time::Duration};

pub type Context<'a> = poise::Context<'a, BotData, BotError>;

pub struct BotData {
    pub http_client: HttpClient,
    pub guild_states: DashMap<serenity::GuildId, GuildPlayer>,
}

impl Default for BotData {
    fn default() -> Self {
        Self {
            http_client: reqwest::Client::new(),
            guild_states: DashMap::new(),
        }
    }
}

impl BotData {
    pub fn new() -> Self {
        Self::default()
    }
}

async fn setup(ctx: &serenity::Context) -> Result<BotData, BotError> {
    poise::builtins::register_globally(ctx, &commands::all()).await?;
    tracing::info!("Registered global commands!");

    Ok(BotData::new())
}

pub async fn handle_shutdown(shard_manager: Arc<serenity::ShardManager>, reason: &str) {
    tracing::warn!("{reason}! Shutting down bot...");
    shard_manager.shutdown_all().await;
    println!("Everything is shutdown. GoodBye!")
}

pub async fn get() -> Result<serenity::Client, BotError> {
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged()
        | serenity::GatewayIntents::MESSAGE_CONTENT
        | serenity::GatewayIntents::GUILD_VOICE_STATES;

    let options = FrameworkOptions {
        commands: commands::all(),
        on_error: |error| Box::pin(events::error::handle(error)),
        command_check: Some(|ctx| {
            Box::pin(async move { Ok(ctx.author().id != ctx.framework().bot_id) })
        }),
        event_handler: |ctx, event, _framework, data| Box::pin(events::handle(ctx, event, data)),
        prefix_options: PrefixFrameworkOptions {
            prefix: Some("!".into()),
            edit_tracker: Some(Arc::new(EditTracker::for_timespan(Duration::from_secs(
                3600,
            )))),
            ..Default::default()
        },
        ..Default::default()
    };

    let songbird_config =
        songbird::Config::default().decode_mode(DecodeMode::Decode(DecodeConfig::default()));

    let framework = Framework::builder()
        .options(options)
        .setup(|ctx, _ready, _framework| Box::pin(setup(ctx)))
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .register_songbird_from_config(songbird_config)
        .await?;

    Ok(client)
}
