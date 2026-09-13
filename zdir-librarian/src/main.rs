use std::thread;

mod db;
mod socket;

fn main() {
    tracing_subscriber::fmt().init();

    let db_thread = thread::spawn(|| db::database());
    let socket_thread = thread::spawn(|| socket::socket());

    db_thread.join().unwrap();
    socket_thread.join().unwrap();
}
