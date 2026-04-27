use crate::{client::Context, error::BotError};
use poise::{
    CreateReply,
    serenity_prelude::{CreateEmbed, User},
};

/// Your profile picture
#[poise::command(context_menu_command = "Get profile picture", slash_command)]
pub async fn pfp(ctx: Context<'_>, user: User) -> Result<(), BotError> {
    let url = user
        .avatar_url()
        .unwrap_or_else(|| user.default_avatar_url());
    let embed = CreateEmbed::new().title(user.name).url(&url).image(&url);
    let reply = CreateReply::default().embed(embed);
    ctx.send(reply).await?;

    Ok(())
}
