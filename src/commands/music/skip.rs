use crate::{client::Context, error::BotError};

#[poise::command(slash_command, prefix_command)]
pub async fn skip(ctx: Context<'_>) -> Result<(), BotError> {
    let guild_id = ctx.guild_id().unwrap_or_default();

    {
        let state = ctx
            .data()
            .guild_states
            .get(&guild_id)
            .ok_or(BotError::NothingPlaying)?;
        if let Some(current) = &state.current_track {
            current.handle.stop()?;
        } else {
            return Err(BotError::NothingPlaying);
        }
    }

    ctx.say("Skipped.").await?;

    Ok(())
}
