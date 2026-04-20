mod app;
mod db;
mod types;

use anyhow::Result;
use human_panic::setup_panic;
use tracing::info;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::filter::Targets;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

use self::db::DBHandler;

fn setup_logging() {
    let filter = Targets::new()
        .with_default(LevelFilter::INFO)
        .with_targets(vec![
            ("wgpu_core", LevelFilter::WARN),
            ("iced_winit", LevelFilter::WARN),
            ("iced_wgpu", LevelFilter::WARN),
            ("calloop", LevelFilter::WARN),
            #[cfg(debug_assertions)]
            ("embervault", LevelFilter::TRACE),
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

    info!("Starting up DB");
    let mut db_handler = DBHandler::new()?;
    db_handler.initialize()?;

    let _db_conn = db_handler.get_diesel_connection()?;

    info!("Launching GUI");
    app::run()?;

    info!("Shutting down DB");
    db_handler.stop()?;

    Ok(())
}
