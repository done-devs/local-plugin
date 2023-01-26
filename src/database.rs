use crate::diesel_migrations::MigrationHarness;
use anyhow::{Context, Result};
use diesel::{Connection, SqliteConnection};
use diesel_migrations::EmbeddedMigrations;
use libset::{format::FileFormat, new_file, project::Project};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");
const DATABASE_NAME: &str = "done_database.db";

fn migrate_database() -> Result<()> {
    let local_plugin_project = Project::new("dev", "edfloreshz", "local-plugin")
        .add_files(&[new_file!(DATABASE_NAME).set_format(FileFormat::Plain)])?;
    let done_project = Project::open("dev", "edfloreshz", "done")?;

    let file = done_project
        .path()
        .context("The database doesn't exist")?
        .join("dev.edfloreshz.Done.db");

    let new_file = local_plugin_project
        .path()
        .context("The project has not been created")?
        .join(DATABASE_NAME);

    if file.exists() {
        std::fs::copy(file, new_file)?;
    }
    Ok(())
}

fn migration_status() -> Result<()> {
    Project::open("dev", "edfloreshz", "local-plugin")?;
    Ok(())
}

fn database_url() -> Result<String> {
    let database_url = Project::open("dev", "edfloreshz", "local-plugin")?
        .path()
        .context("The project has not been created")?
        .join(DATABASE_NAME);

    if !database_url.exists() {
        std::fs::File::create(&database_url)?;
    }
    let path = database_url
        .to_str()
        .context("Failed to convert path to string")?;
    Ok(path.to_string())
}

pub fn establish_connection() -> Result<SqliteConnection> {
    if migration_status().is_err() {
        migrate_database()?
    }

    let url = database_url()?;

    let mut connection =
        SqliteConnection::establish(url.as_str()).context("Error connecting to database")?;
    connection.run_pending_migrations(MIGRATIONS).unwrap();
    Ok(connection)
}
