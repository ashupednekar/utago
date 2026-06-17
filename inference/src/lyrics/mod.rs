use anyhow::Result;
use kalosm::language::Llama;

pub mod spec;

pub struct Lyric {
    model: Llama
}

impl Lyric{
    pub async fn new() -> Result<Self>{
       let model =  Llama::phi_3().await?;
       Ok(Lyric{
           model
       })
    }
}

pub async fn get_lyrics() -> Result<String> {

    Ok("".to_string())
}
