use songbird::input::YoutubeDl;
use tokio::process::Command;
use tracing::instrument;

use crate::{
    audio::{SourceType, TrackMetadata, source::AudioSource},
    error::BotError,
};

pub struct YtDlpSource;

impl YtDlpSource {
    async fn run_ytdlp(args: &[&str]) -> Result<String, BotError> {
        let output = Command::new("yt-dlp")
            .args(args)
            .output()
            .await
            .map_err(|e| BotError::YtDlpError(format!("Failed to spawn yt-dlp: {e}")))?;

        if !output.status.success() && output.stdout.is_empty() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            tracing::error!("yt-dlp failed: {}", stderr);
            return Err(BotError::YtDlpError(format!(
                "yt-dlp exited with {}: {}",
                output.status,
                stderr.lines().next().unwrap_or("unknown error")
            )));
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    fn format_query(query: &str) -> String {
        if query.contains("youtube.com") || query.contains("youtu.be") {
            query.into()
        } else {
            format!("ytsearch:{query}")
        }
    }
}

impl AudioSource for YtDlpSource {
    fn name(&self) -> &str {
        "YouTube (yt-dlp)"
    }

    fn can_handle(&self, query: &str) -> bool {
        query.contains("youtube.com") || query.contains("youtu.be") || !query.starts_with("http")
    }

    #[instrument(skip(self), fields(source = "yt-dlp"))]
    async fn resolve_metadata(&self, query: &str) -> Result<Vec<super::TrackMetadata>, BotError> {
        if !self.can_handle(query) {
            return Err(BotError::Internal("Unsupported query".into()));
        }

        if query.contains("&list=") || query.contains("/playlist") {
            let stdout = Self::run_ytdlp(&[
                "--flat-playlist",
                "--no-download",
                "--extractor-args",
                "youtube:skip=none",
                "-J",
                &Self::format_query(query),
            ])
            .await?;

            let tracks = parse_ytdlp_playlist_output(&stdout)?;

            return Ok(tracks);
        } else {
            let stdout = Self::run_ytdlp(&[
                "--print",
                "title",
                "--print",
                "duration_string",
                "--print",
                "webpage_url",
                "--no-download",
                "--no-playlist",
                &Self::format_query(query),
            ])
            .await?;

            let result = parse_ytdlp_output(&stdout).ok_or_else(|| {
                BotError::YtDlpError(format!("Failed to parse yt-dlp output for: {query}"))
            })?;

            return Ok(vec![result]);
        }
    }

    async fn create_input(
        &self,
        url: &str,
        http_client: &reqwest::Client,
    ) -> Result<songbird::input::Input, BotError> {
        let source = if url.starts_with("http") {
            YoutubeDl::new(http_client.clone(), url.to_string())
        } else {
            YoutubeDl::new_search(http_client.clone(), url.to_string())
        };
        tracing::debug!("Created YoutubeDl input: {}", url);

        Ok(source.into())
    }

    #[instrument(skip(self), fields(source = "yt-dlp"))]
    async fn search(&self, query: &str, limit: usize) -> Result<Vec<TrackMetadata>, BotError> {
        unimplemented!();
    }
}

pub fn parse_ytdlp_output(stdout: &str) -> Option<TrackMetadata> {
    let lines = stdout.lines().collect::<Vec<&str>>();
    if lines.len() < 3 {
        return None;
    }

    let title = lines[0].trim().to_string();
    let duration = lines[1].trim().to_string();
    let url = lines[2].trim().to_string();

    if title.is_empty() || url.is_empty() {
        return None;
    }

    let duration_secs = parse_duration_string(&duration);

    Some(TrackMetadata {
        title,
        duration: if duration.is_empty() {
            None
        } else {
            Some(duration)
        },
        duration_secs,
        url,
        thumbnail_url: None,
        source_type: SourceType::YouTube,
    })
}

pub fn parse_ytdlp_playlist_output(stdout: &str) -> Result<Vec<TrackMetadata>, BotError> {
    let data = serde_json::from_str::<super::YtPlaylist>(stdout)
        .map_err(|e| BotError::Internal(e.to_string()))?;

    let mut tracks = Vec::with_capacity(data.entries.len());
    for entry in data.entries {
        let duration_secs = entry.duration as u64;
        let duration = format!("{}:{:02}", duration_secs / 60, duration_secs % 60);

        tracks.push(TrackMetadata {
            title: entry.title,
            url: entry.url,
            duration_secs: Some(duration_secs),
            duration: Some(duration),
            source_type: SourceType::YouTube,
            thumbnail_url: None,
        });
    }

    Ok(tracks)
}

fn parse_duration_string(s: &str) -> Option<u64> {
    let parts = s.split(":").collect::<Vec<&str>>();

    match parts.len() {
        2 => {
            let mins: u64 = parts[0].parse().ok()?;
            let secs: u64 = parts[1].parse().ok()?;
            Some(mins * 60 + secs)
        }
        3 => {
            let hours: u64 = parts[0].parse().ok()?;
            let mins: u64 = parts[1].parse().ok()?;
            let secs: u64 = parts[2].parse().ok()?;
            Some(hours * 3600 + mins * 60 + secs)
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_handle_youtube_urls() {
        let source = YtDlpSource;
        assert!(source.can_handle("https://www.youtube.com/watch?v=test"));
        assert!(source.can_handle("https://youtu.be/test"));
        assert!(source.can_handle("never gonna give you up"));
        assert!(!source.can_handle("https://soundcloud.com/test"));
    }

    #[test]
    fn parse_valid_output() {
        let output = "Never Gonna Give You Up\n3:33\nhttps://youtube.com/watch?v=dQw4w9WgXcQ\n";
        let meta = parse_ytdlp_output(output).unwrap();
        assert_eq!(meta.title, "Never Gonna Give You Up");
        assert_eq!(meta.duration.as_deref(), Some("3:33"));
        assert_eq!(meta.duration_secs, Some(213));
        assert_eq!(meta.url, "https://youtube.com/watch?v=dQw4w9WgXcQ");
    }

    #[test]
    fn parse_output_with_hours() {
        let output = "Long Video\n1:30:00\nhttps://youtube.com/watch?v=test\n";
        let meta = parse_ytdlp_output(output).unwrap();
        assert_eq!(meta.duration_secs, Some(5400));
    }

    #[test]
    fn parse_incomplete_output() {
        assert!(parse_ytdlp_output("only title\n").is_none());
        assert!(parse_ytdlp_output("").is_none());
    }

    #[test]
    fn parse_empty_title() {
        let output = "\n3:33\nhttps://youtube.com/watch?v=test\n";
        assert!(parse_ytdlp_output(output).is_none());
    }

    #[test]
    fn parse_duration_formats() {
        assert_eq!(parse_duration_string("3:33"), Some(213));
        assert_eq!(parse_duration_string("0:05"), Some(5));
        assert_eq!(parse_duration_string("1:00:00"), Some(3600));
        assert_eq!(parse_duration_string("invalid"), None);
        assert_eq!(parse_duration_string(""), None);
    }
}
