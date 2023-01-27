use crate::diesel_migrations::MigrationHarness;
use anyhow::{anyhow, Context, Result};
use diesel::{Connection, SqliteConnection};
use diesel_migrations::EmbeddedMigrations;
use libset::project::Project;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");
pub const PREVIOUS_DATABASE_NAME: &str = "dev.edfloreshz.Done.db";
pub const DATABASE_NAME: &str = "done_database.db";

pub struct Database;

impl Database {
    pub fn migrate_database() -> Result<()> {
        let local_plugin_project = Project::open("dev", "edfloreshz", "local-plugin")?;

        let done_path =
            Project::exists("dev", "edfloreshz", "done").context("Done is not installed")?;

        let previous_database = done_path.join(PREVIOUS_DATABASE_NAME);

        let new_database = local_plugin_project
            .path()
            .context("The project has not been created")?
            .join(DATABASE_NAME);

        if previous_database.exists() {
            std::fs::copy(previous_database, new_database)?;
        }
        Ok(())
    }

    pub fn migration_status() -> Result<bool> {
        match Project::exists("dev", "edfloreshz", "done") {
            Some(path) => {
                let previous_database = path.join(PREVIOUS_DATABASE_NAME);
                let new_database = Project::open("dev", "edfloreshz", "local-plugin")?
                    .path()
                    .context("Local project not initialized")?
                    .join(DATABASE_NAME);

                let new_database_empty = std::fs::metadata(new_database)?.len() == 0;
                let apply_migration = previous_database.exists() && new_database_empty;
                Ok(apply_migration)
            }
            None => Ok(false),
        }
    }

    fn database_url() -> Result<String> {
        let url = Project::open("dev", "edfloreshz", "local-plugin")?
            .path()
            .context("The project has not been created")?
            .join(DATABASE_NAME)
            .to_str()
            .context("Failed to convert path to string")?
            .to_string();
        Ok(url)
    }

    pub fn establish_connection() -> Result<SqliteConnection> {
        SqliteConnection::establish(Database::database_url()?.as_str())
            .context("Error connecting to database")
    }

    pub fn ensure_migrations_up_to_date() -> Result<()> {
        let mut connection = SqliteConnection::establish(Database::database_url()?.as_str())
            .context("Error connecting to database")?;
        match connection.run_pending_migrations(MIGRATIONS) {
            Ok(_) => Ok(()),
            Err(err) => {
                tracing::error!("{err}");
                Err(anyhow!(err))
            }
        }
    }
}
