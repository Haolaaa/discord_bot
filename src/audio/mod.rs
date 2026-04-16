use poise::serenity_prelude;
use uuid::Uuid;

pub mod source;
pub mod ytdlp;

#[derive(Debug, Clone)]
pub struct TrackMetadata {
    pub title: String,
    pub duration: Option<String>,
    pub duration_secs: Option<u64>,
    pub url: String,
    pub thumbnail_url: Option<String>,
    pub source_type: SourceType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceType {
    YouTube,
    DirectUrl,
}

#[derive(Debug, Clone)]
pub struct QueuedTrack {
    pub id: Uuid,
    pub metadata: TrackMetadata,
    pub query: String,
    pub requested_by: serenity_prelude::UserId,
}

impl QueuedTrack {
    pub fn new(
        metadata: TrackMetadata,
        query: String,
        requested_by: serenity_prelude::UserId,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            metadata,
            query,
            requested_by,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_metadata() -> TrackMetadata {
        TrackMetadata {
            title: "Test Track".into(),
            duration: Some("3:33".into()),
            duration_secs: Some(213),
            url: "https://youtube.com/watch?v=test".into(),
            thumbnail_url: None,
            source_type: SourceType::YouTube,
        }
    }

    #[test]
    fn queued_track_has_unique_id() {
        let user_id = serenity_prelude::UserId::new(1);
        let t1 = QueuedTrack::new(sample_metadata(), "query".into(), user_id);
        let t2 = QueuedTrack::new(sample_metadata(), "query".into(), user_id);
        assert_ne!(t1.id, t2.id);
    }

    #[test]
    fn source_type_equality() {
        assert_eq!(SourceType::YouTube, SourceType::YouTube);
        assert_ne!(SourceType::YouTube, SourceType::DirectUrl);
    }
}
