use crate::{client::BotData, error::BotError};

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
            let msg = if error.is_user_facing() {
                error.to_string()
            } else {
                "Something went wrong. Please try again".into()
            };

            ctx.send(poise::CreateReply::default().content(msg).ephemeral(true))
                .await
                .ok();
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
