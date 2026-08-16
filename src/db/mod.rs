use anyhow::{Result, anyhow};
use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use postgresql_embedded::blocking::PostgreSQL;
use postgresql_embedded::{Settings, SettingsBuilder, VersionReq};
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

pub type DbConnectionManager = ConnectionManager<PgConnection>;
pub type DbPool = Pool<DbConnectionManager>;
pub type DbConnection = PooledConnection<DbConnectionManager>;

pub struct DBHandler {
    state: DBState,
    db: PostgreSQL,
    pool: Option<DbPool>,
}

impl DBHandler {
    pub fn new() -> Result<Self> {
        let pg_settings = SettingsBuilder::new()
            .version(VersionReq::parse("=18")?)
            .temporary(false)
            .build();
        Ok(Self {
            db: start_db(pg_settings)?,
            state: DBState::Started,
            pool: None,
        })
    }

    fn init_db(&mut self) -> Result<()> {
        match &mut self.state {
            DBState::Started => {
                if !self.db.database_exists(DB_NAME)? {
                    info!("DB name does not exist, will create it");
                    self.db.create_database(DB_NAME)?;
                }

                self.build_pool()?;
                self.state = DBState::Initialized;
                Ok(())
            }
            DBState::Stopped => todo!("Implement errors"),
            DBState::Initialized | DBState::Ready => Ok(()),
        }
    }

    fn build_pool(&mut self) -> Result<()> {
        debug!("Building Diesel connection pool");

        let database_url = self.db.settings().url(DB_NAME);
        let manager = ConnectionManager::<PgConnection>::new(database_url);

        self.pool = Some(Pool::builder().build(manager)?);

        Ok(())
    }

    fn perform_migrations(&mut self) -> Result<()> {
        match &self.state {
            DBState::Started | DBState::Stopped => todo!("Implement errors"),
            DBState::Initialized => {}
            DBState::Ready => return Ok(()),
        }

        let mut conn = self.get_diesel_connection(false)?;

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

    fn get_diesel_connection(&self, require_ready: bool) -> Result<DbConnection> {
        debug!("Getting diesel connection");
        match self.state {
            DBState::Started | DBState::Stopped => todo!("Implement errors"),
            DBState::Initialized if require_ready => todo!("Implement errors"),
            DBState::Initialized | DBState::Ready => {
                let pool = self
                    .pool
                    .as_ref()
                    .ok_or_else(|| anyhow!("Database connection pool is not initialized"))?;

                Ok(pool.get()?)
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
            self.pool = None;
            self.db.stop()?;
            self.state = DBState::Stopped;
        }
        Ok(())
    }

    pub fn pool(&self) -> Result<DbPool> {
        match self.state {
            DBState::Ready => self
                .pool
                .clone()
                .ok_or_else(|| anyhow!("Database connection pool is not initialized")),
            _ => todo!("Implement errors"),
        }
    }
}

fn start_db(settings: Settings) -> Result<PostgreSQL> {
    info!("Installing PostgreSQL {}", settings.version.to_string());
    let mut pg = PostgreSQL::new(settings);
    pg.setup()?;
    info!("Starting up PostgreSQL");
    pg.start()?;
    Ok(pg)
}
