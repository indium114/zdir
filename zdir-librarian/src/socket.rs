use tracing::info;
use std::sync::mpsc;

fn socket_path() -> String {
    "/tmp/zdir-".to_string() + &unsafe { libc::getuid().to_string() } + ".sock"
}

pub fn socket(tx: mpsc::Sender<String>, rx: mpsc::Receiver<String>) {
    info!(path = socket_path(), "Starting socket")
}
