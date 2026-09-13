use std::{sync::mpsc, thread};

mod db;
mod socket;
mod util;

fn main() {
    let (db_tx, socket_rx) = mpsc::channel();
    let (socket_tx, db_rx) = mpsc::channel();

    tracing_subscriber::fmt().init();

    let db_thread = thread::spawn(move || db::database(db_tx, db_rx));
    let socket_thread =
        thread::spawn(move || socket::socket(socket_tx, socket_rx));

    db_thread.join().unwrap();
    socket_thread.join().unwrap();
}
