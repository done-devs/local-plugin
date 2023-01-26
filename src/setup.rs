use crate::database::{migration_status, migrate_database, ensure_migrations_up_to_date};
use anyhow::Result;

pub fn init() -> Result<()> {
    tracing_subscriber::fmt()
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::FULL)
        .with_max_level(tracing::Level::INFO)
        .init();
    if migration_status().is_err() {
        migrate_database()?;
    }
    ensure_migrations_up_to_date()?;
    Ok(())
}
