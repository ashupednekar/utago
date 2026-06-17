pub mod lyrics;
pub mod shazam;

pub mod prelude;
use audio::AudioBytes;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecognitionMatch {
    pub song_id: String,
    pub title: String,
    pub artist: String,
    pub timestamp_ms: u32,
    pub confidence_percent: u8,
    pub received_bytes: usize,
}

pub fn handle_audio_bytes(sample: AudioBytes) -> RecognitionMatch {
    let active_song = library::active_song();

    RecognitionMatch {
        song_id: active_song.id,
        title: active_song.title,
        artist: active_song.artist,
        timestamp_ms: 56_000,
        confidence_percent: 91,
        received_bytes: sample.len(),
    }
}

pub fn handle_audio_byte_slice(bytes: &[u8]) -> RecognitionMatch {
    handle_audio_bytes(AudioBytes::new(
        bytes.to_vec(),
        "audio/byte-slice",
        44_100,
        1,
    ))
}
