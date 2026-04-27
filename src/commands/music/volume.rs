use crate::{client::Context, error::BotError};

/// Set playback volume (0-200)
#[poise::command(slash_command, guild_only)]
pub async fn volume(
    ctx: Context<'_>,
    #[description = "Volume level (0-200, default 50)"]
    #[min = 0]
    #[max = 200]
    level: u32,
) -> Result<(), BotError> {
    let guild_id = ctx
        .guild_id()
        .ok_or(BotError::Internal("Not in guild".into()))?;

    let vol = level.min(200) as f32 / 100.0;

    let mut state = ctx
        .data()
        .guild_states
        .get_mut(&guild_id)
        .ok_or(BotError::NothingPlaying)?;

    state.volume = vol;
    if let Some(current) = &state.current_track {
        current.handle.set_volume(vol)?;
    }
    drop(state);

    ctx.say(format!("Volume set to **{level}%**")).await?;
    Ok(())
}
