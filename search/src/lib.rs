use library::{LibrarySong, SongDraft};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiscoverySurface {
    Spotify,
    Local,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiscoveryOption {
    pub label: String,
    pub detail: String,
    pub surface: DiscoverySurface,
}

pub fn discovery_options() -> Vec<DiscoveryOption> {
    vec![
        DiscoveryOption {
            label: "Spotify link".to_string(),
            detail: "metadata import".to_string(),
            surface: DiscoverySurface::Spotify,
        },
        DiscoveryOption {
            label: "Local music".to_string(),
            detail: "lyrics + description".to_string(),
            surface: DiscoverySurface::Local,
        },
    ]
}

pub fn draft_local_song(description: impl Into<String>, lyrics: impl Into<String>) -> SongDraft {
    SongDraft::local_discovery(description, lyrics)
}

pub fn draft_local_directory(path: impl Into<String>, description: impl Into<String>) -> SongDraft {
    SongDraft::local_directory(path, description)
}

pub fn library_preview() -> Vec<LibrarySong> {
    library::featured_songs()
}
