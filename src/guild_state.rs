use crate::{audio::TrackMetadata, queue::GuildQueue};
use poise::serenity_prelude;
use songbird::tracks::TrackHandle;
use std::time::Instant;

pub struct GuildPlayer {
    pub queue: GuildQueue,
    pub current_track: Option<CurrentTrack>,
    pub text_channel_id: serenity_prelude::ChannelId,
    pub volume: f32,
    pub idle_since: Option<Instant>,
}

pub struct CurrentTrack {
    pub metadata: TrackMetadata,
    pub handle: TrackHandle,
    pub started_at: Instant,
}

impl Default for GuildPlayer {
    fn default() -> Self {
        Self {
            queue: GuildQueue::default(),
            current_track: None,
            text_channel_id: serenity_prelude::ChannelId::new(1),
            volume: 0.5,
            idle_since: Some(Instant::now()),
        }
    }
}
