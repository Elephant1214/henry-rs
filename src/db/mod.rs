use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode};
use sqlx::{Pool, Sqlite};
use std::path::Path;

pub mod model {
    pub mod guild;
}

pub struct HenryDb {
    pub pool: Pool<Sqlite>,
}

// TODO: Make this "do" all transactions, will be pub pool for now
impl HenryDb {
    pub async fn new(db_file: impl AsRef<Path>) -> Self {
        HenryDb {
            pool: Self::connect(db_file).await,
        }
    }

    async fn connect(db_file: impl AsRef<Path>) -> Pool<Sqlite> {
        Pool::connect_with(
            SqliteConnectOptions::new()
                .filename(db_file)
                .journal_mode(SqliteJournalMode::Wal),
        )
        .await
        .unwrap()
    }
}
