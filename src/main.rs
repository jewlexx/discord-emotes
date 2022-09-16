use std::fs::create_dir;

use providers::{Provider, ProviderEmotes};
use tracing_subscriber::fmt::format::FmtSpan;

mod providers;

#[macro_use]
extern crate tracing;

const DEMO_ID: &str = "61f638a2084cfa2e05d2569b";

rotenv_codegen::dotenv_module!(visibility = "pub(crate)");

// Will panic if env vars do not exist
const _: &str = dotenv_vars::CLIENT_SECRET;
const _: &str = dotenv_vars::CLIENT_ID;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_span_events(FmtSpan::ENTER | FmtSpan::EXIT)
        .with_max_level(tracing::Level::TRACE)
        .init();

    let seventv_emotes: ProviderEmotes = providers::seventv::SevenTvSet::get(DEMO_ID)?.into();
    let pwd = std::env::current_dir()?;

    let emotes_dir = pwd.join("emotes");

    if !emotes_dir.exists() {
        create_dir(&emotes_dir)?;
    }

    seventv_emotes.download_to_dir(emotes_dir)?;

    Ok(())
}
