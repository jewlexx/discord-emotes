use std::{fs::File, io::Write};

use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use tracing_subscriber::fmt::{self, format::FmtSpan};

use providers::Provider;

mod providers;

#[macro_use]
extern crate tracing;

#[macro_use]
extern crate async_trait;

const DEMO_ID: &str = "61f638a2084cfa2e05d2569b";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    fmt::fmt()
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .finish();

    let seventv_emotes = providers::seventv::SevenTvSet::get(DEMO_ID).await?;

    seventv_emotes
        .emotes
        .par_iter()
        .map(|emote| -> anyhow::Result<()> {
            let url = emote.data.host.url.clone();
            let name = emote.data.name.clone();
            let id = emote.data.id.clone();

            trace!("Downloading emote {} ({}) from {}", name, id, url);

            let mut file = File::create(name)?;

            let bytes = reqwest::blocking::get(url)?.bytes()?;

            file.write_all(&bytes)?;

            Ok(())
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    info!("Hello, world!");

    Ok(())
}
