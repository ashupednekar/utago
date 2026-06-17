#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AudioBytes {
    pub bytes: Vec<u8>,
    pub mime_type: String,
    pub sample_rate_hz: u32,
    pub channels: u16,
}

impl AudioBytes {
    pub fn new(
        bytes: Vec<u8>,
        mime_type: impl Into<String>,
        sample_rate_hz: u32,
        channels: u16,
    ) -> Self {
        Self {
            bytes,
            mime_type: mime_type.into(),
            sample_rate_hz,
            channels,
        }
    }

    pub fn placeholder_hum_phrase() -> Self {
        Self::new(vec![0; 4096], "audio/placeholder-hum", 44_100, 1)
    }

    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }
}

pub trait AudioByteHandler {
    type Output;

    fn handle_audio_bytes(&self, sample: AudioBytes) -> Self::Output;
}
