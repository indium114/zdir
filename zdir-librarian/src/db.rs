use tracing::{info, warn};
use redb::{
  Database,
  Error,
  ReadableDatabase,
  ReadableTable,
  TableDefinition,
};
use std::{fs, path::Path, sync::{Arc, mpsc}, thread, time::Duration};

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
    let db = Arc::new(db);

    // MARK: housekeeping thread
    let housekeeping_db = db.clone();
    let housekeeping_thread = thread::spawn(move || {
        info!("Starting housekeeping thread");

        loop {
            info!("Running housekeeping");

            let write_tx = housekeeping_db.begin_write().unwrap();
            {
                let mut table = write_tx.open_table(TABLE).unwrap();

                let updated: Vec<(String, (f64, u64))> = table
                    .iter()
                    .unwrap()
                    .map(|entry| {
                        let (path, value) = entry.unwrap();
                        let path = path.value();
                        let (frecency, last_accessed) = value.value();

                        let entry = crate::util::rank(crate::util::Entry {
                            path,
                            frecency,
                            last_accessed,
                        });

                        (entry.path, (entry.frecency, entry.last_accessed))
                    })
                    .collect();

                for (k, v) in updated {
                    table.insert(k, v).unwrap();
                }
            }
            write_tx.commit().unwrap();

            info!("Finished housekeeping");
            thread::sleep(Duration::from_secs(crate::util::HOUR as u64));
        }

    });
    housekeeping_thread.join().unwrap();

    // MARK: comms thread
    let comms_thread = thread::spawn(move || {
        info!("Starting comms thread");
    });
    comms_thread.join().unwrap();
}
