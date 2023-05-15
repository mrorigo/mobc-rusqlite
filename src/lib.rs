pub use mobc;
use mobc::async_trait;
use mobc::Manager;

#[derive(Debug)]
pub struct RusqliteConnectionManager {
    file_name: Option<String>,
}

impl RusqliteConnectionManager {
    pub fn new(file_name: Option<String>) -> Self {
        Self { file_name }
    }
}

#[async_trait]
impl Manager for RusqliteConnectionManager {
    type Connection = rusqlite::Connection;

    type Error = rusqlite::Error;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        let conn = if let Some(ref name) = self.file_name {
            rusqlite::Connection::open(name)?
        } else {
            rusqlite::Connection::open_in_memory()?
        };

        Ok(conn)
    }

    async fn check(&self, conn: Self::Connection) -> Result<Self::Connection, Self::Error> {
        conn.query_row("SELECT datetime()", [], |row: &rusqlite::Row| {
            row.get::<usize, String>(0)
        })?;
        Ok(conn)
    }
}
