use crate::{client::Context, error::BotError};

/// Shuffle the queue
#[poise::command(slash_command, prefix_command)]
pub async fn shuffle(ctx: Context<'_>) -> Result<(), BotError> {
    let guild_id = ctx.guild_id().unwrap_or_default();

    let count = {
        let mut state = ctx
            .data()
            .guild_states
            .get_mut(&guild_id)
            .ok_or(BotError::QueueEmpty)?;
        state.queue.shuffle();
        state.queue.len()
    };

    ctx.say(format!("Shuffled {count} tracks.")).await?;

    Ok(())
}
