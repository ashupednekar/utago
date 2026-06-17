#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SongSource {
    SpotifyLink(String),
    LocalDiscovery,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SongDraft {
    pub title: String,
    pub artist: String,
    pub description: String,
    pub lyrics: String,
    pub source: SongSource,
}

impl SongDraft {
    pub fn from_spotify_link(link: impl Into<String>) -> Self {
        Self {
            title: "Spotify import".to_string(),
            artist: "Pending lookup".to_string(),
            description: "Track metadata will resolve from Spotify later.".to_string(),
            lyrics: String::new(),
            source: SongSource::SpotifyLink(link.into()),
        }
    }

    pub fn local_discovery(description: impl Into<String>, lyrics: impl Into<String>) -> Self {
        Self {
            title: "Local discovery".to_string(),
            artist: "Unknown local artist".to_string(),
            description: description.into(),
            lyrics: lyrics.into(),
            source: SongSource::LocalDiscovery,
        }
    }

    pub fn staging_label(&self) -> String {
        match &self.source {
            SongSource::SpotifyLink(link) if link.trim().is_empty() => {
                "Spotify draft staged without a link".to_string()
            }
            SongSource::SpotifyLink(_) => "Spotify draft staged".to_string(),
            SongSource::LocalDiscovery => "Local discovery draft staged for AI fill".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LyricLine {
    pub timestamp_ms: u32,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LibrarySong {
    pub id: String,
    pub title: String,
    pub artist: String,
    pub duration_ms: u32,
    pub lyrics: Vec<LyricLine>,
}

pub fn active_song() -> LibrarySong {
    featured_songs()
        .into_iter()
        .next()
        .expect("featured songs should include the active demo song")
}

pub fn featured_songs() -> Vec<LibrarySong> {
    vec![
        LibrarySong {
            id: "local-seaside-static".to_string(),
            title: "Seaside Static".to_string(),
            artist: "Mira Vale".to_string(),
            duration_ms: 188_000,
            lyrics: vec![
                lyric(16_000, "I kept a signal under my tongue"),
                lyric(32_000, "White noise folded into sun"),
                lyric(56_000, "Meet me where the chorus breaks"),
                lyric(78_000, "I will sing the timestamp awake"),
                lyric(102_000, "Every echo knows your name"),
            ],
        },
        LibrarySong {
            id: "spotify-orange-room".to_string(),
            title: "Orange Room Demo".to_string(),
            artist: "Imported Track".to_string(),
            duration_ms: 214_000,
            lyrics: vec![lyric(22_000, "A draft waits for synced lyrics")],
        },
        LibrarySong {
            id: "local-night-bus".to_string(),
            title: "Night Bus Choir".to_string(),
            artist: "Local submission".to_string(),
            duration_ms: 171_000,
            lyrics: vec![lyric(18_000, "Local song metadata pending")],
        },
    ]
}

fn lyric(timestamp_ms: u32, text: &str) -> LyricLine {
    LyricLine {
        timestamp_ms,
        text: text.to_string(),
    }
}
