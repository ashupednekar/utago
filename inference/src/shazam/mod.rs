pub struct FingerprintRequest {
    pub received_bytes: usize,
}

impl FingerprintRequest {
    pub fn placeholder(received_bytes: usize) -> Self {
        Self { received_bytes }
    }
}
