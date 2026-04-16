use crate::{client::Context, commands::voice::get_user_voice_channel, error::BotError};
use poise::serenity_prelude;
use tracing::instrument;

#[poise::command(slash_command, guild_only)]
#[instrument(skip(ctx), fields(guild = ?ctx.guild_id()))]
pub async fn join(
    ctx: Context<'_>,
    #[description = "Voice channel to join (defaults to yours)"] channel: Option<
        serenity_prelude::Channel,
    >,
) -> Result<(), BotError> {
    let guild_id = ctx
        .guild_id()
        .ok_or(BotError::Internal("Not in guild".into()))?;

    let channel_id = match channel {
        Some(ch) => ch.id(),
        None => get_user_voice_channel(ctx)?,
    };

    let manager = songbird::get(ctx.serenity_context())
        .await
        .ok_or(BotError::Internal("Songbird not initialized".into()))?;

    if let Err(e) = manager.join(guild_id, channel_id).await {
        tracing::error!("join error: {:?}", e);
        return Err(BotError::SongbirdJoin(e));
    }

    tracing::info!(%guild_id, %channel_id, "Joined voice channel");
    ctx.say(format!("Joined <#{channel_id}>")).await?;

    Ok(())
}
