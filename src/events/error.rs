use crate::{client::BotData, consts::Colors, error::BotError};
use poise::{
    CreateReply,
    serenity_prelude::{CreateEmbed, Timestamp},
};

pub async fn handle(error: poise::FrameworkError<'_, BotData, BotError>) {
    match error {
        poise::FrameworkError::Setup {
            error, framework, ..
        } => {
            tracing::error!("Error setting up client! Bailing out");
            framework.shard_manager().shutdown_all().await;

            panic!("{error}");
        }
        poise::FrameworkError::Command { error, ctx, .. } => {
            tracing::error!("Error in command {}:\n{error}", ctx.command().name);

            let embed = CreateEmbed::default()
                .title("Something went wrong!")
                .description(format!("Oopsie... {}", error))
                .timestamp(Timestamp::now())
                .color(Colors::Orange);

            let reply = CreateReply::default().embed(embed);

            ctx.send(reply).await.ok();
        }
        poise::FrameworkError::EventHandler {
            error,
            ctx: _,
            event,
            framework: _,
            ..
        } => {
            tracing::error!(
                "Error while handling event {}:\n{error}",
                event.snake_case_name()
            );
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                tracing::error!("Unhandled error occurred:\n{e:#?}");
            }
        }
    }
}
