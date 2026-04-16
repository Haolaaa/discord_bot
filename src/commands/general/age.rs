use crate::{client::Context, error::BotError};
use poise::serenity_prelude::User;

#[poise::command(slash_command)]
pub async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<User>,
) -> Result<(), BotError> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!(
        "{}'s account was created at {}",
        u.name,
        u.created_at().format("%Y/%m/%d %H:%M:%S")
    );
    ctx.say(response).await?;

    Ok(())
}
