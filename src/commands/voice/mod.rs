pub mod join;
pub mod leave;

use crate::{client::Context, error::BotError};
use poise::serenity_prelude;

pub fn get_user_voice_channel(ctx: Context<'_>) -> Result<serenity_prelude::ChannelId, BotError> {
    let guild = ctx
        .guild()
        .ok_or(BotError::Internal("Not in guild".into()))?;

    let channel_id = guild
        .voice_states
        .get(&ctx.author().id)
        .and_then(|vs| vs.channel_id)
        .ok_or(BotError::NotInVoiceChannel)?;
    Ok(channel_id)
}
