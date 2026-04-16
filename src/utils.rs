pub fn handle_songbird_error(error: songbird::error::JoinError) -> &'static str {
    match error {
        songbird::error::JoinError::Dropped => "request was cancelled/dropped",
        songbird::error::JoinError::NoSender => "no gateway destination",
        songbird::error::JoinError::NoCall => "tried to leave a non-existent call",
        songbird::error::JoinError::TimedOut => "gateway response from Discord timed out",
        e => {
            tracing::error!("join error: {}", e.to_string());
            "unhandled error"
        }
    }
}
