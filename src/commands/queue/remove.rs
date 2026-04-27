use crate::{client::Context, error::BotError};

/// Remove a track from the queue by position
#[poise::command(slash_command, prefix_command)]
pub async fn remove(
    ctx: Context<'_>,
    #[description = "Track position in queue (1-based)"] position: u32,
) -> Result<(), BotError> {
    let guild_id = ctx.guild_id().unwrap_or_default();

    let idx = position as usize;
    if idx == 0 {
        return Err(BotError::InvalidIndex(0));
    }

    let mut state = ctx
        .data()
        .guild_states
        .get_mut(&guild_id)
        .ok_or(BotError::NothingPlaying)?;

    let removed = state
        .queue
        .remove(idx - 1)
        .ok_or(BotError::InvalidIndex(idx))?;

    let title = removed.metadata.title.clone();
    drop(state);

    ctx.say(format!("Removed: **{title}**")).await?;

    Ok(())
}
