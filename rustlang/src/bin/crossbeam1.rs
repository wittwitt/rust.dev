use std::{cell::Cell, sync::Arc, thread};

use crossbeam_utils::atomic::AtomicCell;

fn main() {
    std_cell();
    atomic_cell();
}

fn atomic_cell() -> () {
    let a = Arc::new(AtomicCell::new(123));

    let mut index = 0;
    let mut ls = Vec::new();

    loop {
        let to_move = a.clone();
        let t = thread::spawn(move || {
            println!("{:?}", to_move.load());
            to_move.store(100);
        });

        ls.push(t);

        index = index + 1;
        if index > 10 {
            break;
        }
    }

    for item in ls.into_iter() {
        item.join().unwrap();
    }

    println!("{:?}", a.load());
}

fn std_cell() {
    let a = Cell::new(5);
    a.set(6);
    a.set(7);
}
