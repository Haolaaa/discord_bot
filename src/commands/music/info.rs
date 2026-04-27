use crate::{client::Context, error::BotError};
use poise::serenity_prelude;

/// Show what's currently playing
#[poise::command(slash_command, guild_only)]
pub async fn nowplaying(ctx: Context<'_>) -> Result<(), BotError> {
    let guild_id = ctx
        .guild_id()
        .ok_or(BotError::Internal("Not in guild".into()))?;

    let (title, progress, next_info) = {
        let state = ctx
            .data()
            .guild_states
            .get(&guild_id)
            .ok_or(BotError::NothingPlaying)?;

        let current = state
            .current_track
            .as_ref()
            .ok_or(BotError::NothingPlaying)?;

        let elapsed = current.started_at.elapsed().as_secs();
        let elapsed_fmt = format!("{}:{:02}", elapsed / 60, elapsed % 60);

        let duration_str = current.metadata.duration.as_deref().unwrap_or("?:??");

        let progress = if let Some(total_secs) = current.metadata.duration_secs {
            let ratio = (elapsed as f64 / total_secs as f64).min(1.0);
            let filled = (ratio * 20.0) as usize;
            let bar = "=".repeat(filled) + ">" + &"-".repeat(20 - filled);
            format!("[{bar}] {elapsed_fmt} / {duration_str}")
        } else {
            format!("{elapsed_fmt} / {duration_str}")
        };

        let next_info = state
            .queue
            .peek_next()
            .map(|t| format!("**Next up:** {}", t.metadata.title))
            .unwrap_or_default();

        let title = current.metadata.title.clone();
        (title, progress, next_info)
    };

    let embed = serenity_prelude::CreateEmbed::new()
        .title("Now playing")
        .description(format!("**{title}**\n\n{progress}\n\n{next_info}"))
        .color(0x1DB954);

    ctx.send(poise::CreateReply::default().embed(embed)).await?;

    Ok(())
}
