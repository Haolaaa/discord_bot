use crate::{client::Context, error::BotError};

/// Show the current queue
#[poise::command(slash_command, prefix_command)]
pub async fn queue(
    ctx: Context<'_>,
    #[description = "Page number (10 tracks per page)"] page: Option<u32>,
) -> Result<(), BotError> {
    let guild_id = ctx.guild_id().unwrap_or_default();

    let state = ctx
        .data()
        .guild_states
        .get(&guild_id)
        .ok_or(BotError::NothingPlaying)?;

    let current_info = state.current_track.as_ref().map(|ct| {
        format!(
            "**Now playing:** {} [{}]",
            ct.metadata.title,
            ct.metadata.duration.as_deref().unwrap_or("?:??")
        )
    });

    let queue = state.queue.list();
    if queue.is_empty() && current_info.is_none() {
        drop(state);
        return Err(BotError::QueueEmpty);
    }

    let page = page.unwrap_or(1).max(1) as usize;
    let per_page = 10;
    let total_pages = queue.len().max(1).div_ceil(per_page);
    let start = (page - 1) * per_page;

    let mut lines = Vec::new();
    if let Some(info) = &current_info {
        lines.push(info.clone());
        lines.push(String::new());
    }

    if queue.is_empty() {
        lines.push("Queue is empty.".into());
    } else {
        lines.push(format!("**Queue** ({} tracks):", queue.len()));
        for (i, track) in queue.iter().enumerate().skip(start).take(per_page) {
            lines.push(format!(
                "`{}.` {} [{}]",
                i + 1,
                track.metadata.title,
                track.metadata.duration.as_deref().unwrap_or("?:??")
            ));
        }
        if total_pages > 1 {
            lines.push(format!("\nPage {page}/{total_pages}"));
        }
    }

    drop(state);

    ctx.say(lines.join("\n")).await?;

    Ok(())
}
