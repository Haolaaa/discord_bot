use crate::{client::Context, error::BotError};
use tracing::instrument;

#[poise::command(slash_command, guild_only)]
#[instrument(skip(ctx), fields(guild = ?ctx.guild_id()))]
pub async fn leave(ctx: Context<'_>) -> Result<(), BotError> {
    let guild_id = ctx
        .guild_id()
        .ok_or(BotError::Internal("Not in guild".into()))?;
    let manager = songbird::get(ctx.serenity_context())
        .await
        .ok_or(BotError::Internal("Songbird not initialized".into()))?;

    manager.remove(guild_id).await?;

    ctx.data().guild_states.remove(&guild_id);

    tracing::info!(%guild_id, "Left voice channel");
    ctx.say("Left the voice channel.").await?;
    Ok(())
}
