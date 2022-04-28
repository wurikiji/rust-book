use std::{sync::mpsc, thread};

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // this line will not compile, because it violates the borrowing rule.
        // println!("val is {}", val);
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received)
}
