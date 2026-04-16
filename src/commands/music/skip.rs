use crate::{client::Context, error::BotError};

#[poise::command(slash_command, prefix_command)]
pub async fn skip(ctx: Context<'_>) -> Result<(), BotError> {
    let guild_id = ctx.guild_id().unwrap_or_default();

    let manager = songbird::get(ctx.serenity_context())
        .await
        .ok_or_else(|| BotError::Internal("Songbird not initialized".into()))?;

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        todo!()
    }

    Ok(())
}
