mod app;
mod types;

use anyhow::Result;
use tracing::info;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::filter::Targets;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};
use human_panic::setup_panic;

fn setup_logging() {
    let filter = Targets::new()
        .with_default(LevelFilter::INFO)
        .with_targets(vec![
            ("wgpu_core", LevelFilter::WARN),
            ("iced_winit", LevelFilter::WARN),
            ("iced_wgpu", LevelFilter::WARN),
            ("calloop", LevelFilter::WARN),
        ]);
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(filter)
        .init();
}

#[allow(clippy::unnecessary_wraps)]
fn main() -> Result<()> {
    setup_panic!();
    setup_logging();

    info!("Hello, Mom!");
    app::run()?;

    Ok(())
}
