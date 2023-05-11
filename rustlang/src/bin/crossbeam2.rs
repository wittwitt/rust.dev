use std::time::Duration;

use crossbeam_utils::thread::scope;

fn main() {
    scope(|s| {
        let mut runners = Vec::with_capacity(3);

        runners.push(s.spawn(|_| {
            println!("={}", 1);
            std::thread::sleep(Duration::from_millis(1000));
            println!("{}", 2);
        }));

        runners.push(s.spawn(|_| {
            println!("=={}", 3);
            std::thread::sleep(Duration::from_millis(1000));
            println!("{}", 4);
        }));

        std::thread::spawn(|| {
            println!("==={}", 5);
            std::thread::sleep(Duration::from_millis(2000));
            println!("{}", 6);
        });

        for runer in runners {
            runer.join().unwrap();
        }

        println!("xx{}", 7);
    })
    .unwrap();

    println!("{}", 8);
}
