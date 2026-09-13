use std::{
    fs,
    io::{BufRead, BufReader, Write},
    os::unix::net::UnixListener,
    path::Path,
    sync::mpsc,
};
use tracing::{error, info};

fn socket_path() -> String {
    "/tmp/zdir-".to_string() + &unsafe { libc::getuid().to_string() } + ".sock"
}

pub fn socket(tx: mpsc::Sender<String>, rx: mpsc::Receiver<String>) {
    info!(path = socket_path(), "Starting socket");

    if Path::new(&socket_path()).exists() {
        fs::remove_file(socket_path()).unwrap();
    }

    let listener = UnixListener::bind(socket_path()).unwrap();
    for connection in listener.incoming() {
        match connection {
            Ok(mut stream) => {
                let mut reader = BufReader::new(stream.try_clone().unwrap());
                let mut message = String::new();
                reader.read_line(&mut message).unwrap();
                info!(msg = message, "Received message"); // DEBUG

                let _ = tx.send(message);

                while let Ok(query) = rx.recv() {
                    stream.write_all(query.as_bytes()).unwrap();
                    stream.flush().unwrap();
                    break;
                }
            }
            Err(e) => error!(err = e.to_string(), "Error while listening"),
        }
    }
}
