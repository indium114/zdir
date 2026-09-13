use crate::util::matches;
use redb::{Database, ReadableDatabase, ReadableTable, TableDefinition};
use std::{
    fs,
    path::Path,
    sync::{Arc, mpsc},
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tracing::{info, warn};

// NOTE: path, (frecency, last_accessed)
const TABLE: TableDefinition<String, (f64, u64)> =
    TableDefinition::new("directories");

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
            Database::create(db_path())
                .expect("Failed to load database. Is it corrupted?")
        }
        false => {
            warn!(path = db_path(), "Creating new database");
            let path = db_path();
            let path = Path::new(&path);
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).unwrap();
            }
            fs::File::create(path).unwrap();
            Database::create(db_path())
                .expect("Failed to create database. Is the path writable?")
        }
    };
    let db = Arc::new(db);

    // MARK: housekeeping thread
    let housekeeping_db = db.clone();
    let housekeeping_thread = thread::spawn(move || {
        info!("Starting housekeeping thread");

        loop {
            info!("Running housekeeping");

            let write_txn = housekeeping_db.begin_write().unwrap();
            {
                let mut table = write_txn.open_table(TABLE).unwrap();

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
            write_txn.commit().unwrap();

            info!("Finished housekeeping");
            thread::sleep(Duration::from_secs(crate::util::HOUR as u64));
        }
    });

    // MARK: comms thread
    let comms_db = db.clone();
    let comms_thread = thread::spawn(move || {
        info!("Starting comms thread");

        while let Ok(query) = rx.recv() {
            let query = query.trim_end_matches('\n').to_string();
            match query.starts_with('/') {
                true => {
                    let write_txn = comms_db.begin_write().unwrap();
                    {
                        let mut table = write_txn.open_table(TABLE).unwrap();
                        if table.get(&query).unwrap().is_none() {
                            info!(path = query, "Writing path to database");
                            let _ = table.insert(&query, (1.0, SystemTime::now().duration_since(UNIX_EPOCH).expect("You've travelled back to... before 1970? How do you even have a computer?").as_secs()));
                        }
                    }
                    write_txn.commit().unwrap();
                    let _ = tx.send(query);
                }
                false => {
                    let read_txn = comms_db.begin_read().unwrap();
                    let table = read_txn.open_table(TABLE).unwrap();

                    let matches: Vec<String> = table
                        .iter()
                        .unwrap()
                        .map(|entry| {
                            let (path, value) = entry.unwrap();
                            let path = path.value();
                            let (frecency, last_accessed) = value.value();

                            if matches(&path, query.split(' ').collect()) {
                                path
                            } else {
                                "".to_string()
                            }
                        })
                        .collect();
                    let match_string = matches.join("**");
                    let _ = tx.send(match_string);
                }
            }
        }
    });

    housekeeping_thread.join().unwrap();
    comms_thread.join().unwrap();
}
