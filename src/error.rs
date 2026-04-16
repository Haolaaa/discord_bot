#[derive(Debug, thiserror::Error)]
pub enum BotError {
    #[error("You must be in a voice channel to use this command.")]
    NotInVoiceChannel,

    #[error("Nothing is currently playing.")]
    NothingPlaying,

    #[error("The queue is empty.")]
    QueueEmpty,

    #[error("Invalid index: {0}")]
    InvalidIndex(usize),

    #[error("Could not find any results for: {0}")]
    NoSearchResults(String),

    #[error("YouTube extraction failed: {0}")]
    YtDlpError(String),

    #[error("Voice connection failed: {0}")]
    VoiceError(String),

    #[error("Songbird join error: {0}")]
    SongbirdJoin(#[from] songbird::error::JoinError),

    #[error("Songbird control error: {0}")]
    SongbirdControl(#[from] songbird::tracks::ControlError),

    #[error("Serenity error: {0}")]
    Serenity(#[from] serenity::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Internal error: {0}")]
    Internal(String),
}

impl BotError {
    pub fn is_user_facing(&self) -> bool {
        matches!(
            self,
            BotError::NotInVoiceChannel
                | BotError::NothingPlaying
                | BotError::InvalidIndex(_)
                | BotError::NoSearchResults(_)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_facing_errors_display_correctly() {
        assert_eq!(
            BotError::NotInVoiceChannel.to_string(),
            "You must be in a voice channel to use this command."
        );
        assert_eq!(
            BotError::NothingPlaying.to_string(),
            "Nothing is currently playing."
        );
        assert_eq!(BotError::QueueEmpty.to_string(), "The queue is empty.");
        assert_eq!(BotError::InvalidIndex(5).to_string(), "Invalid index: 5");
        assert_eq!(
            BotError::NoSearchResults("test query".into()).to_string(),
            "Could not find any results for: test query"
        );
    }

    #[test]
    fn is_user_facing_classification() {
        assert!(BotError::NotInVoiceChannel.is_user_facing());
        assert!(BotError::NothingPlaying.is_user_facing());
        assert!(BotError::QueueEmpty.is_user_facing());
        assert!(!BotError::YtDlpError("fail".into()).is_user_facing());
        assert!(!BotError::Internal("fail".into()).is_user_facing());
    }
}
