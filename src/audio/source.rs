use songbird::input::Input;

use crate::{audio::TrackMetadata, error::BotError};

pub trait AudioSource: Send + Sync {
    fn name(&self) -> &str;
    fn can_handle(&self, query: &str) -> bool;
    async fn resolve_metadata(&self, query: &str) -> Result<Vec<TrackMetadata>, BotError>;
    async fn create_input(
        &self,
        url: &str,
        http_client: &reqwest::Client,
    ) -> Result<Input, BotError>;
    async fn search(&self, query: &str, limit: usize) -> Result<Vec<TrackMetadata>, BotError>;
}
