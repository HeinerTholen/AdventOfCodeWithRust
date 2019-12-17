use std::sync::mpsc;
use std::thread;

mod intcode_cmp;
mod input;

fn main() {
    println!("Task 1:");
    let (tx_in, rx_in) = mpsc::channel();
    let (tx_out, rx_out) = mpsc::channel();
    let mut icc = intcode_cmp::IntCodeComputer {
        program: input::PROGRAM_INPUT.to_vec(),
        tx: tx_out,
        rx: rx_in,
    };
    thread::spawn(move || icc.compute());
    tx_in.send(0).unwrap();
    tx_in.send(4).unwrap();
    println!("{:?}", rx_out.recv().unwrap());
}
