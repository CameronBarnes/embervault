use anyhow::{Result, anyhow};
use diesel::{Connection, PgConnection};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use postgresql_embedded::blocking::PostgreSQL;
use tracing::{debug, info};

pub mod schema;

const DB_NAME: &str = "embervault";
const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DBState {
    Started,
    Initialized,
    Ready,
    Stopped,
}

pub struct DBHandler {
    state: DBState,
    db: PostgreSQL,
    connection: Option<PgConnection>,
}

impl DBHandler {
    pub fn new() -> Result<Self> {
        Ok(Self {
            db: start_db()?,
            state: DBState::Started,
            connection: None,
        })
    }

    fn init_db(&mut self) -> Result<()> {
        match &mut self.state {
            DBState::Started => {
                if !self.db.database_exists(DB_NAME)? {
                    info!("DB name does not exist, will create it");
                    self.db.create_database(DB_NAME)?;
                }
                self.state = DBState::Initialized;
                Ok(())
            }
            DBState::Stopped => todo!("Implement errors"),
            DBState::Initialized | DBState::Ready => Ok(()),
        }
    }

    fn perform_migrations(&mut self) -> Result<()> {
        match &self.state {
            DBState::Started | DBState::Stopped => todo!("Implement errors"),
            DBState::Initialized => {}
            DBState::Ready => return Ok(()),
        }

        let conn = self.get_diesel_connection_internal(false)?;

        debug!("Querrying for migrations");
        if conn
            .has_pending_migration(MIGRATIONS)
            .map_err(|e| anyhow!("Diesel failure when checking for migrations: {e}"))?
        {
            info!("Running pending Migrations");
            conn.run_pending_migrations(MIGRATIONS)
                .map_err(|e| anyhow!("DB Migration failure: {e}"))?;
        } else {
            info!("No pending Migrations");
        }

        self.state = DBState::Ready;
        Ok(())
    }

    fn get_diesel_connection_internal(&mut self, require_ready: bool) -> Result<&mut PgConnection> {
        debug!("Getting diesel connection");
        match &self.state {
            DBState::Started | DBState::Stopped => todo!("Implement errors"),
            DBState::Initialized => {
                if require_ready {
                    todo!("Implement errors")
                }
                if self.connection.is_none() {
                    self.connection =
                        Some(PgConnection::establish(&self.db.settings().url(DB_NAME))?);
                }
                Ok(self.connection.as_mut().unwrap())
            }
            DBState::Ready => {
                if self.connection.is_none() {
                    self.connection =
                        Some(PgConnection::establish(&self.db.settings().url(DB_NAME))?);
                }
                Ok(self.connection.as_mut().unwrap())
            }
        }
    }

    pub fn initialize(&mut self) -> Result<()> {
        match &self.state {
            DBState::Started => {
                info!("Initializing DB");
                self.init_db()?;
                info!("Running any pending migrations");
                self.perform_migrations()
            }
            DBState::Initialized => {
                info!("Already Initialzied, running any pending migrations");
                self.perform_migrations()
            }
            DBState::Ready => Ok(()),
            DBState::Stopped => todo!("Implement errors"),
        }
    }

    pub fn stop(&mut self) -> Result<()> {
        if !matches!(self.state, DBState::Stopped) {
            self.db.stop()?;
            self.state = DBState::Stopped;
        }
        Ok(())
    }

    pub fn get_diesel_connection(&mut self) -> Result<&mut PgConnection> {
        self.get_diesel_connection_internal(true)
    }
}

fn start_db() -> Result<PostgreSQL> {
    let mut pg = PostgreSQL::default();
    pg.setup()?;
    pg.start()?;
    Ok(pg)
}
