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
        let song = format!("song: {:?}", &song_name);
        tokio::spawn(async move {
            let mut res = chat.add_message(song);
            while let Some(token) = res.next().await {
                tx.send(token).await.unwrap();
            }
        });
        Ok(rx)
    }
}

mod tests {
    use std::time::Duration;
    use tokio::time::timeout;

    #[tokio::test]
    async fn test_get_lyrics() -> anyhow::Result<()> {
        use crate::lyrics::Lyric;
        let lyric = Lyric::new().await?;
        let mut c = String::new();
        let mut res = lyric.get("hey jude").await?;

        timeout(Duration::from_secs(300), async {
            while let Some(t) = res.recv().await {
                if t == "\n" {
                    println!("{:?}", &c);
                    c.clear();
                } else {
                    c.push_str(&t);
                }
            }

            if !c.is_empty() {
                println!("{:?}", &c);
            }
        })
        .await?;

        Ok(())
    }
}
