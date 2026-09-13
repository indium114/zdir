use redb::{
  Database,
  Error,
  ReadableDatabase,
  ReadableTable,
  TableDefinition,
};

fn db_path() -> String {
    "/tmp/zdir-".to_string() + &unsafe { libc::getuid().to_string() } + ".sock"
}

pub fn database() {
    println!("Database!");
    println!("Path: {}", db_path());
}
