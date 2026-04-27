use crate::{
    audio::{QueuedTrack, source::AudioSource, ytdlp::YtDlpSource},
    client::BotData,
    error::BotError,
    guild_state::CurrentTrack,
    queue::LoopMode,
};
use poise::serenity_prelude;
use songbird::{Event, EventContext, EventHandler as VoiceEventHandler, TrackEvent};
use std::{sync::Arc, time::Instant};
use tracing::instrument;

pub mod info;
pub mod loop_cmd;
pub mod pause;
pub mod play;
pub mod resume;
pub mod stop;
pub mod volume;

#[instrument(skip(data, manager))]
pub async fn play_next(
    data: &BotData,
    manager: Arc<songbird::Songbird>,
    guild_id: serenity_prelude::GuildId,
    http: &Arc<serenity_prelude::Http>,
) -> Result<(), BotError> {
    let track = {
        let mut state = data.guild_states.entry(guild_id).or_default();
        state.current_track = None;
        state.queue.dequeue()
    };

    let track = match track {
        Some(t) => t,
        None => {
            if let Some(mut state) = data.guild_states.get_mut(&guild_id) {
                state.idle_since = Some(Instant::now());
            }
            tracing::info!(%guild_id, "Queue empty, entering idle");
            return Ok(());
        }
    };

    let handler_lock = manager
        .get(guild_id)
        .ok_or(BotError::VoiceError("Not in a voice channel".into()))?;

    let source = YtDlpSource;
    let resolve_start = Instant::now();
    let input = source
        .create_input(&track.metadata.url, &data.http_client)
        .await?;
    let resolve_duration = resolve_start.elapsed();
    tracing::info!(duration = ?resolve_duration, title = %track.metadata.title, "Audio source created");

    let track_handle = {
        let mut handler = handler_lock.lock().await;
        let handle = handler.play_input(input);
        handle.set_volume(
            data.guild_states
                .get(&guild_id)
                .map(|s| s.volume)
                .unwrap_or(0.1),
        )?;
        handle
    };

    let text_channel_id = data
        .guild_states
        .get(&guild_id)
        .map(|s| s.text_channel_id)
        .unwrap_or(serenity_prelude::ChannelId::new(1));

    track_handle.add_event(
        Event::Track(TrackEvent::End),
        TrackEndHandler {
            guild_id,
            data: data as *const BotData as usize,
            manager: manager.clone(),
            http: http.clone(),
        },
    )?;

    if let Some(mut state) = data.guild_states.get_mut(&guild_id) {
        state.current_track = Some(CurrentTrack {
            metadata: track.metadata.clone(),
            handle: track_handle,
            started_at: Instant::now(),
        });
        state.idle_since = None;
    }

    let _ = text_channel_id
        .say(
            http,
            format!(
                "Now playing: **{}** [{}]",
                track.metadata.title,
                track.metadata.duration.as_deref().unwrap_or("?:??")
            ),
        )
        .await;

    tracing::info!(title = %track.metadata.title, %guild_id, "Playing track");
    Ok(())
}

struct TrackEndHandler {
    guild_id: serenity_prelude::GuildId,
    data: usize,
    manager: Arc<songbird::Songbird>,
    http: Arc<serenity_prelude::Http>,
}

unsafe impl Send for TrackEndHandler {}
unsafe impl Sync for TrackEndHandler {}

#[serenity::async_trait]
impl VoiceEventHandler for TrackEndHandler {
    async fn act(&self, _ctx: &EventContext<'_>) -> Option<Event> {
        let data = unsafe { &*(self.data as *const BotData) };

        let finished_track_meta = data
            .guild_states
            .get(&self.guild_id)
            .and_then(|s| s.current_track.as_ref().map(|ct| ct.metadata.clone()));

        if let Some(meta) = &finished_track_meta {
            if let Some(mut state) = data.guild_states.get_mut(&self.guild_id) {
                match state.queue.loop_mode {
                    LoopMode::Single => {
                        if state.current_track.is_some() {
                            let re_track =
                                QueuedTrack::new(meta.clone(), serenity_prelude::UserId::new(1));
                            state.queue.enqueue_front(re_track);
                        }
                    }
                    LoopMode::Queue => {
                        if state.current_track.is_some() {
                            let re_track =
                                QueuedTrack::new(meta.clone(), serenity_prelude::UserId::new(1));
                            state.queue.enqueue(re_track);
                        }
                    }
                    LoopMode::Off => {}
                }
            }
        }

        if let Err(e) = play_next(data, self.manager.clone(), self.guild_id, &self.http).await {
            tracing::error!(?e, guild_id = %self.guild_id, "Failed to play next track");
        }

        None
    }
}
