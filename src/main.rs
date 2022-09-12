use tracing_subscriber::fmt::{self, format::FmtSpan};

mod providers;

#[macro_use]
extern crate log;

#[macro_use]
extern crate async_trait;

const DEMO_ID: &str = "61f638a2084cfa2e05d2569b";

#[tokio::main]
async fn main() {
    fmt::fmt()
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .finish();

    info!("Hello, world!");
}
