use std::{
    fs::{create_dir, File},
    io::Write,
};

use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

use providers::{Provider, ProviderEmotes};

mod providers;

#[macro_use]
extern crate tracing;

const DEMO_ID: &str = "61f638a2084cfa2e05d2569b";

fn verify_dotenv() -> Vec<&'static str> {
    if dotenv::dotenv().is_ok() {
        info!("Loaded .env file");
    } else {
        panic!("No .env file found");
    }

    let mut missing = Vec::new();

    if dotenv::var("CLIENT_ID").is_err() {
        missing.push("CLIENT_ID");
    }

    if dotenv::var("CLIENT_SECRET").is_err() {
        missing.push("CLIENT_SECRET");
    }

    missing
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    {
        let missing_env = verify_dotenv();

        if !missing_env.is_empty() {
            panic!("Missing environment variables: {:?}", missing_env);
        }
    }

    let seventv_emotes: ProviderEmotes = providers::seventv::SevenTvSet::get(DEMO_ID)?.into();
    let pwd = std::env::current_dir()?;

    let emotes_dir = pwd.join("emotes");

    if !emotes_dir.exists() {
        create_dir(&emotes_dir)?;
    }

    seventv_emotes
        .emotes
        .par_iter()
        .map(|emote| -> anyhow::Result<()> {
            let file_name = format!("{}.{}", emote.name, emote.extension);

            trace!("Downloading emote {} from {}", emote.name, emote.url);

            let mut file = File::create(emotes_dir.join(file_name))?;

            let bytes = reqwest::blocking::get(&emote.url)?.bytes()?;

            file.write_all(&bytes)?;

            Ok(())
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    Ok(())
}
