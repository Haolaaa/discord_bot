pub mod error;
pub mod message;

use crate::{client::BotData, consts, error::BotError};
use poise::serenity_prelude::{Context, CreateBotAuthParameters, FullEvent};

pub async fn handle(ctx: &Context, event: &FullEvent, data: &BotData) -> Result<(), BotError> {
    match event {
        FullEvent::Ready { data_about_bot } => {
            tracing::info!("Logged in as {}!", data_about_bot.user.name);

            if let Ok(invite_link) = CreateBotAuthParameters::new().auto_client_id(ctx).await {
                let link = invite_link
                    .scopes(consts::bot_scopes())
                    .permissions(*consts::bot_permissions())
                    .build();
                tracing::info!("Invite me to your server at {link}");
            } else {
                tracing::debug!("Not displaying invite_link since we couldn't find our client ID");
            }
        }
        FullEvent::Message { new_message } => {
            message::handle(ctx, new_message, data).await?;
        }
        _ => {}
    }

    Ok(())
}
