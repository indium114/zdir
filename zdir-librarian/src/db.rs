use crate::util::{Entry, matches};
use redb::{Database, ReadableDatabase, ReadableTable, TableDefinition};
use std::{
    fs,
    path::Path,
    sync::{Arc, mpsc},
    thread,
    time::{SystemTime, UNIX_EPOCH},
};
use tracing::{info, warn};

const MAXAGE: f64 = 10_000.0;
const AGING_TARGET: f64 = 0.9 * MAXAGE;

// NOTE: path, (frecency, last_accessed)
const TABLE: TableDefinition<String, (f64, u64)> =
    TableDefinition::new("directories");

fn db_path() -> String {
    let home = dirs::home_dir()
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_default();
    home + "/.local/share/zdir/zdir.db"
}

fn score(db: Arc<Database>, entry: Entry) {
    info!("Scoring");

    let write_txn = db.begin_write().unwrap();
    {
        let mut table = write_txn.open_table(TABLE).unwrap();
        let ranked_entry = crate::util::rank(entry);
        let _ = table.insert(
            ranked_entry.path,
            (ranked_entry.frecency, ranked_entry.last_accessed),
        );
    }
    write_txn.commit().unwrap();

    age(db);

    info!("Finished scoring");
}

fn age(db: Arc<Database>) {
    info!("Aging");

    let write_txn = db.begin_write().unwrap();
    {
        let mut table = write_txn.open_table(TABLE).unwrap();

        let entries: Vec<(String, (f64, u64))> = table
            .iter()
            .unwrap()
            .map(|e| {
               let (path, value) = e.unwrap();
               (path.value().to_string(), value.value())
            })
            .collect();

        let total: f64 = entries.iter().map(|(_, (f, _))| f).sum();
        if total > MAXAGE {
            let k = AGING_TARGET / total;
            for (path, (frecency, last_accessed)) in entries {
                let new_frecency = frecency * k;
                if new_frecency < 1.0 {
                    let _ = table.remove(&path);
                } else {
                    let _ = table.insert(&path, (new_frecency, last_accessed));
                }
            }
        }
    }

    write_txn.commit().unwrap();
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

    // MARK: comms thread
    let comms_db = db.clone();
    #[allow(unused_variables)]
    let comms_thread = thread::spawn(move || {
        info!("Starting comms thread");

        while let Ok(query) = rx.recv() {
            let query = query.trim_end_matches('\n').to_string();
            if query.starts_with("PICK:") {
                let path = query.strip_prefix("PICK:").unwrap();
                let read_txn = comms_db.begin_read().unwrap();
                let table = read_txn.open_table(TABLE).unwrap();

                let current = table
                    .get(path.to_string())
                    .unwrap()
                    .map(|v| v.value())
                    .unwrap_or((0.0, 0));

                let entry = Entry {
                    path: path.to_string(),
                    frecency: current.0,
                    last_accessed: current.1,
                };

                let _ = tx.send("ACK:".to_string());

                score(db.clone(), entry);
            } else {
                match query.starts_with('/') {
                    true => {
                        let write_txn = comms_db.begin_write().unwrap();
                        {
                            let mut table =
                                write_txn.open_table(TABLE).unwrap();
                            if table.get(&query).unwrap().is_none() {
                                info!(path = query, "Writing path to database");
                                let _ = table.insert(&query, (0.0, SystemTime::now().duration_since(UNIX_EPOCH).expect("You've travelled back to... before 1970? How do you even have a computer?").as_secs()));
                            }
                        }
                        write_txn.commit().unwrap();
                        let _ = tx.send("0.0:".to_owned() + &query);
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
                                let (frecency, _last_accessed) = value.value();

                                if matches(&path, query.split(' ').collect()) {
                                    frecency.to_string() + ":" + &path
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
        }
    });

    comms_thread.join().unwrap();
}
