use tracing::info;

fn socket_path() -> String {
    "/tmp/zdir-".to_string() + &unsafe { libc::getuid().to_string() } + ".sock"
}

pub fn socket() {
    info!(path = socket_path(), "Starting socket")
}
