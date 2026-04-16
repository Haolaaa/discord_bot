use crate::{
    audio::{QueuedTrack, source::AudioSource, ytdlp::YtDlpSource},
    client::Context,
    commands::{music::play_next, voice::get_user_voice_channel},
    error::BotError,
};

#[poise::command(slash_command, prefix_command)]
pub async fn play(
    ctx: Context<'_>,
    #[description = "YouTube URL or search query"]
    #[rest]
    query: String,
) -> Result<(), BotError> {
    let guild_id = ctx
        .guild_id()
        .ok_or(BotError::Internal("Not in guild".into()))?;

    let user_channel = get_user_voice_channel(ctx)?;

    let manager = songbird::get(ctx.serenity_context())
        .await
        .ok_or(BotError::Internal("Songbird not initialized".into()))?;

    if manager.get(guild_id).is_none() {
        manager.join(guild_id, user_channel).await?;
    }

    ctx.defer().await?;

    let source = YtDlpSource;
    let metadata = source.resolve_metadata(&query).await?;

    let queued_track = QueuedTrack::new(metadata.clone(), query, ctx.author().id);

    let should_play = {
        let mut state = ctx.data().guild_states.entry(guild_id).or_default();
        let is_idle = state.current_track.is_none();
        state.queue.enqueue(queued_track);
        state.text_channel_id = ctx.channel_id();
        is_idle
    };

    if should_play {
        play_next(ctx.data(), manager, guild_id, &ctx.serenity_context().http).await?;
    } else {
        let queue_len = ctx
            .data()
            .guild_states
            .get(&guild_id)
            .map(|s| s.queue.len())
            .unwrap_or(0);

        ctx.say(format!(
            "Queued at #{}: **{}** [{}]",
            queue_len,
            metadata.title,
            metadata.duration.as_deref().unwrap_or("?:??")
        ))
        .await?;
    }

    Ok(())
}
