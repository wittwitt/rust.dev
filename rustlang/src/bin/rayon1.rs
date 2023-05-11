use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn main() {
    let mut list = Vec::new();

    for i in 0..2000 {
        list.push(i);
    }

    std::thread::sleep(std::time::Duration::from_secs(5));

    list.par_iter()
        // .with_max_len(100)
        .map(|map_op| {
            println!(
                "{},{},{}",
                rayon::current_num_threads(),
                rayon::current_thread_index().unwrap(),
                map_op
            );
            std::thread::sleep(std::time::Duration::from_secs(1));
        })
        .count();
}
