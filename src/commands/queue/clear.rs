use crate::{client::Context, error::BotError};

/// Clear the queue (keeps current track playing)
#[poise::command(slash_command, prefix_command)]
pub async fn clear(ctx: Context<'_>) -> Result<(), BotError> {
    let guild_id = ctx.guild_id().unwrap_or_default();

    let count = {
        let mut state = ctx
            .data()
            .guild_states
            .get_mut(&guild_id)
            .ok_or(BotError::NothingPlaying)?;
        let count = state.queue.len();
        state.queue.clear();
        count
    };

    ctx.say(format!("Cleared {count} tracks from the queue"))
        .await?;

    Ok(())
}
