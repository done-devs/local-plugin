use crate::database::{Database, DATABASE_NAME};
use anyhow::Result;
use libset::{format::FileFormat, new_file, project::Project};

pub fn init() -> Result<()> {
    tracing_subscriber::fmt()
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::FULL)
        .with_max_level(tracing::Level::INFO)
        .init();
    Project::new("dev", "edfloreshz", "local-plugin")
        .author("Eduardo Flores <edfloreshz@gmail.com>")
        .about("Local task service")
        .version("0.1.2")
        .add_files(&[new_file!(DATABASE_NAME).set_format(FileFormat::Plain)])?;

    if Database::migration_status()? {
        Database::migrate_database()?;
    }
    Database::ensure_migrations_up_to_date()?;

    Ok(())
}
