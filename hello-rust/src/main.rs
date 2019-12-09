use ferris_says::say;
use std::io::{stdout, BufWriter};

use std::sync::mpsc as mpsc;
use std::thread as thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();

    thread::spawn(move || tx.send(" Hello "));
    thread::spawn(move || tx2.send(" World "));

    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    say(rx.recv().unwrap().as_bytes(), 10, &mut writer).unwrap();
    say(rx.recv().unwrap().as_bytes(), 10, &mut writer).unwrap();
}
