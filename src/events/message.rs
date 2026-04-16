use crate::{client::BotData, error::BotError};
use poise::serenity_prelude::{Context, Message};

pub async fn handle(ctx: &Context, msg: &Message, data: &BotData) -> Result<(), BotError> {
    if should_echo(ctx, msg, data).await? {
        msg.reply(ctx, &msg.content).await?;
    }

    Ok(())
}

async fn should_echo(ctx: &Context, msg: &Message, _data: &BotData) -> Result<bool, BotError> {
    if (msg.author.bot && msg.webhook_id.is_none())
        || msg.author.id == ctx.cache.as_ref().current_user().id
    {
        tracing::debug!("Not repeating another bot");
        return Ok(false);
    }

    // let gid = msg
    //     .guild_id
    //     .ok_or_else(|| format!("Couldn't get GuildId from {}!", msg.id))?;

    let content = &msg.content;

    Ok(content.to_ascii_lowercase() == "ghal" || content.to_ascii_lowercase() == "saya")
}
