use crate::{client::Context, error::BotError};

#[poise::command(slash_command, guild_only)]
pub async fn resume(ctx: Context<'_>) -> Result<(), BotError> {
    let guild_id = ctx
        .guild_id()
        .ok_or(BotError::Internal("Not in guild".into()))?;

    let state = ctx
        .data()
        .guild_states
        .get(&guild_id)
        .ok_or(BotError::NothingPlaying)?;

    let current = state
        .current_track
        .as_ref()
        .ok_or(BotError::NothingPlaying)?;
    current.handle.play()?;

    ctx.say("Resumed.").await?;

    Ok(())
}
