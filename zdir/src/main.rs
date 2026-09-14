use std::{
    fs,
    io::{Read, Write},
    os::unix::net::UnixStream,
    path::Path,
    process,
};

mod parser;
mod tui;

fn socket_path() -> String {
    "/tmp/zdir-".to_string() + &unsafe { libc::getuid().to_string() } + ".sock"
}

fn main() {
    let query: String =
        std::env::args().skip(2).collect::<Vec<String>>().join(" ");
    let path: String =
        std::env::args().collect::<Vec<String>>()[1].to_string();

    let query: String = match fs::canonicalize(&query) {
        Ok(p) => p
            .into_os_string()
            .into_string()
            .expect("Path is not valid UTF-8"),
        _ => query,
    };

    if !Path::new(&socket_path()).exists() {
        usefulog::err(format!(
            "{} does not exist. zdir-librarian is likely not running",
            socket_path()
        ));
        process::exit(1);
    }

    let mut socket = UnixStream::connect(socket_path()).unwrap();
    socket.write(format!("{query}\n").as_bytes()).unwrap();

    let mut buffer = [0u8; 4096];
    let count = socket.read(&mut buffer).unwrap();

    let mut results = crate::parser::parse_response(&String::from_utf8_lossy(
        &buffer[..count],
    ));
    results.sort_unstable_by(|a, b| b.0.total_cmp(&a.0));

    if results.is_empty() {
        usefulog::err("No matches.");
        process::exit(1);
    }

    let close = match results.first().zip(results.get(1)) {
        Some((a, b)) => a.0 - b.0 < 200.0,
        None => false,
    };

    match close {
        true => {
            let selection = crate::tui::tui(results);
            let mut pick = UnixStream::connect(socket_path()).unwrap();
            let _ = pick.write(format!("PICK:{selection}\n").as_bytes());
            let mut ack = [0u8; 4096];
            let _ = pick.read(&mut ack).unwrap();
            fs::write(path, selection).unwrap();
        }
        false => {
            let (_, selection) = results.first().unwrap();
            let mut pick = UnixStream::connect(socket_path()).unwrap();
            let _ = pick.write(format!("PICK:{selection}\n").as_bytes());
            let mut ack = [0u8; 4096];
            let _ = pick.read(&mut ack).unwrap();
            fs::write(path, selection).unwrap();
        }
    }
}
