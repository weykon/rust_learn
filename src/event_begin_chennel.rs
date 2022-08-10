use std::sync::mpsc;
use std::thread;

// mpsc : multiple producer, single consumer. 

pub fn main() {

    // a transmitter and a receiver
    let (tx, rx) = mpsc::channel();


    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

