use tracing::{info, warn};
use redb::{
  Database,
  Error,
  ReadableDatabase,
  ReadableTable,
  TableDefinition,
};
use std::{fs, path::Path, sync::mpsc};

// NOTE: path, (frecency, last_accessed)
const TABLE: TableDefinition<String, (f64, u64)> = TableDefinition::new("directories");

fn db_path() -> String {
    let home = dirs::home_dir()
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_default();
    home + "/.local/share/zdir/zdir.db"
}

pub fn database(tx: mpsc::Sender<String>, rx: mpsc::Receiver<String>) {
    info!("Starting database");

    let db: Database = match Path::new(&db_path()).exists() {
        true => {
            info!(path = db_path(), "Loading database");
            Database::create(db_path()).expect("Failed to load database. Is it corrupted?")
        }
        false => {
            warn!(path = db_path(), "Creating new database");
            let path = db_path();
            let path = Path::new(&path);
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).unwrap();
            }
            fs::File::create(path).unwrap();
            Database::create(db_path()).expect("Failed to create database. Is the path writable?")
        }
    };
}
