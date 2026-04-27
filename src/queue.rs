use crate::audio::QueuedTrack;
use rand::seq::SliceRandom;
use std::collections::VecDeque;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum LoopMode {
    #[default]
    Off,
    Single,
    Queue,
}

impl LoopMode {
    pub fn cycle(self) -> Self {
        match self {
            LoopMode::Off => LoopMode::Single,
            LoopMode::Single => LoopMode::Queue,
            LoopMode::Queue => LoopMode::Off,
        }
    }

    pub fn display_name(self) -> &'static str {
        match self {
            LoopMode::Off => "Off",
            LoopMode::Single => "Single",
            LoopMode::Queue => "Queue",
        }
    }
}

#[derive(Debug, Default)]
pub struct GuildQueue {
    tracks: VecDeque<QueuedTrack>,
    pub loop_mode: LoopMode,
}
impl GuildQueue {
    pub fn enqueue(&mut self, track: QueuedTrack) {
        self.tracks.push_back(track);
    }

    pub fn enqueue_front(&mut self, track: QueuedTrack) {
        self.tracks.push_front(track);
    }

    pub fn dequeue(&mut self) -> Option<QueuedTrack> {
        self.tracks.pop_front()
    }

    pub fn skip(&mut self) -> Option<QueuedTrack> {
        self.tracks.pop_front()
    }

    pub fn shuffle(&mut self) {
        let mut rng = rand::rng();
        let mut vec = self.tracks.drain(..).collect::<Vec<_>>();
        vec.shuffle(&mut rng);
        self.tracks = vec.into();
    }

    pub fn remove(&mut self, index: usize) -> Option<QueuedTrack> {
        self.tracks.remove(index)
    }

    pub fn clear(&mut self) {
        self.tracks.clear();
    }

    pub fn peek_next(&self) -> Option<&QueuedTrack> {
        self.tracks.front()
    }

    pub fn list(&self) -> &VecDeque<QueuedTrack> {
        &self.tracks
    }

    pub fn len(&self) -> usize {
        self.tracks.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tracks.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::audio::{QueuedTrack, SourceType, TrackMetadata};
    use poise::serenity_prelude as serenity;

    fn make_track(name: &str) -> QueuedTrack {
        QueuedTrack::new(
            TrackMetadata {
                title: name.into(),
                duration: None,
                duration_secs: None,
                url: format!("https://youtube.com/watch?v={name}"),
                thumbnail_url: None,
                source_type: SourceType::YouTube,
            },
            serenity::UserId::new(1),
        )
    }

    #[test]
    fn enqueue_dequeue_order() {
        let mut q = GuildQueue::default();
        q.enqueue(make_track("a"));
        q.enqueue(make_track("b"));
        q.enqueue(make_track("c"));

        assert_eq!(q.dequeue().unwrap().metadata.title, "a");
        assert_eq!(q.dequeue().unwrap().metadata.title, "b");
        assert_eq!(q.dequeue().unwrap().metadata.title, "c");
        assert!(q.dequeue().is_none());
    }

    #[test]
    fn skip_returns_front() {
        let mut q = GuildQueue::default();
        q.enqueue(make_track("first"));
        q.enqueue(make_track("second"));

        let skipped = q.skip().unwrap();
        assert_eq!(skipped.metadata.title, "first");
        assert_eq!(q.len(), 1);
    }

    #[test]
    fn skip_empty_returns_none() {
        let mut q = GuildQueue::default();
        assert!(q.skip().is_none());
    }

    #[test]
    fn shuffle_preserves_length() {
        let mut q = GuildQueue::default();
        for i in 0..10 {
            q.enqueue(make_track(&format!("track{i}")));
        }
        q.shuffle();
        assert_eq!(q.len(), 10);
    }

    #[test]
    fn shuffle_empty_does_not_panic() {
        let mut q = GuildQueue::default();
        q.shuffle();
        assert!(q.is_empty());
    }

    #[test]
    fn shuffle_single_item() {
        let mut q = GuildQueue::default();
        q.enqueue(make_track("only"));
        q.shuffle();
        assert_eq!(q.len(), 1);
        assert_eq!(q.peek_next().unwrap().metadata.title, "only");
    }

    #[test]
    fn remove_valid_index() {
        let mut q = GuildQueue::default();
        q.enqueue(make_track("a"));
        q.enqueue(make_track("b"));
        q.enqueue(make_track("c"));

        let removed = q.remove(1).unwrap();
        assert_eq!(removed.metadata.title, "b");
        assert_eq!(q.len(), 2);
    }

    #[test]
    fn remove_invalid_index() {
        let mut q = GuildQueue::default();
        q.enqueue(make_track("a"));
        assert!(q.remove(5).is_none());
    }

    #[test]
    fn clear_empties_queue() {
        let mut q = GuildQueue::default();
        q.enqueue(make_track("a"));
        q.enqueue(make_track("b"));
        q.clear();
        assert!(q.is_empty());
        assert_eq!(q.len(), 0);
    }

    #[test]
    fn peek_next_does_not_remove() {
        let mut q = GuildQueue::default();
        q.enqueue(make_track("a"));
        assert!(q.peek_next().is_some());
        assert_eq!(q.len(), 1);
    }

    #[test]
    fn enqueue_front_adds_to_beginning() {
        let mut q = GuildQueue::default();
        q.enqueue(make_track("a"));
        q.enqueue(make_track("b"));
        q.enqueue_front(make_track("front"));
        assert_eq!(q.peek_next().unwrap().metadata.title, "front");
    }

    #[test]
    fn loop_mode_cycle() {
        assert_eq!(LoopMode::Off.cycle(), LoopMode::Single);
        assert_eq!(LoopMode::Single.cycle(), LoopMode::Queue);
        assert_eq!(LoopMode::Queue.cycle(), LoopMode::Off);
    }

    #[test]
    fn loop_mode_display_names() {
        assert_eq!(LoopMode::Off.display_name(), "Off");
        assert_eq!(LoopMode::Single.display_name(), "Single");
        assert_eq!(LoopMode::Queue.display_name(), "Queue");
    }
}
