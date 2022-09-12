use std::{
    fs::{create_dir, File},
    io::Write,
};

use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

use providers::Provider;

mod providers;

#[macro_use]
extern crate tracing;

#[macro_use]
extern crate async_trait;

const DEMO_ID: &str = "61f638a2084cfa2e05d2569b";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    let seventv_emotes = providers::seventv::SevenTvSet::get(DEMO_ID).await?;
    let pwd = std::env::current_dir()?;

    let emotes_dir = pwd.join("emotes");

    if !emotes_dir.exists() {
        create_dir(&emotes_dir)?;
    }

    seventv_emotes
        .emotes
        .par_iter()
        .map(|emote| -> anyhow::Result<()> {
            let file = &emote
                .data
                .host
                .files
                .iter()
                .find(|file| file.name == providers::seventv::Name::The4XAvif)
                .unwrap();

            let url = format!("https:{}/{}", emote.data.host.url, file.name);
            let name = &emote.data.name;
            let id = &emote.data.id;
            let file_name = format!("{}.{}", name, file.name);

            trace!("Downloading emote {} ({}) from {}", name, id, url);

            let mut file = File::create(emotes_dir.join(file_name))?;

            let bytes = reqwest::blocking::get(url)?.bytes()?;

            file.write_all(&bytes)?;

            Ok(())
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    Ok(())
}
