use crate::{client::Context, error::BotError};

#[poise::command(slash_command, guild_only)]
pub async fn stop(ctx: Context<'_>) -> Result<(), BotError> {
    let guild_id = ctx
        .guild_id()
        .ok_or(BotError::Internal("Not in guild".into()))?;

    let manager = songbird::get(ctx.serenity_context())
        .await
        .ok_or(BotError::Internal("Songbird not initialized".into()))?;

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;
        handler.stop();
    }

    manager.remove(guild_id).await?;
    ctx.data().guild_states.remove(&guild_id);

    ctx.say("Stopped and cleared queue.").await?;

    Ok(())
}
