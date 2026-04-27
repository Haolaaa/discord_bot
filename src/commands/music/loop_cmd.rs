use crate::{client::Context, error::BotError, queue::LoopMode};

/// Toggle loop mode (Off → Single → Queue)
#[poise::command(slash_command, guild_only)]
pub async fn loop_cmd(
    ctx: Context<'_>,
    #[description = "Loop mode: off, single, queue"] mode: Option<String>,
) -> Result<(), BotError> {
    let guild_id = ctx
        .guild_id()
        .ok_or(BotError::Internal("Not in guild".into()))?;

    let new_mode = {
        let mut state = ctx
            .data()
            .guild_states
            .get_mut(&guild_id)
            .ok_or(BotError::NothingPlaying)?;

        let new_mode = match mode.as_deref() {
            Some("off") => LoopMode::Off,
            Some("single") => LoopMode::Single,
            Some("queue") => LoopMode::Queue,
            _ => state.queue.loop_mode.cycle(),
        };
        state.queue.loop_mode = new_mode;
        new_mode
    };

    ctx.say(format!("Loop mode: **{}**", new_mode.display_name()))
        .await?;

    Ok(())
}
