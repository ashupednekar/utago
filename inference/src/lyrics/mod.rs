use anyhow::Result;
use kalosm::language::{ChatModelExt, Llama, StreamExt};
use tokio::sync::mpsc::{self, Receiver};

pub mod spec;
pub struct Lyric {
    model: Llama,
}

impl Lyric {
    pub async fn new() -> Result<Self> {
        let model = Llama::phi_3().await?;
        Ok(Lyric { model })
    }

    pub async fn get(&self, song_name: &str) -> anyhow::Result<Receiver<String>> {
        let (tx, rx) = mpsc::channel::<String>(1);
        let mut chat = self
            .model
            .chat()
            .with_system_prompt(common::settings.lyrics.prompt.clone());
        let mut res = chat.add_message(format!("song: {song_name}"));
        while let Some(token) = res.next().await {
            tx.send(token).await?;
        }
        Ok(rx)
    }
}
