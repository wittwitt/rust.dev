fn main() {
    let num_sectors_per_chunk: usize = 2;

    for sector_index in 0..100 {
        let j = sector_index / num_sectors_per_chunk;

        let i = sector_index - j * num_sectors_per_chunk;

        println!("{},{},{}", sector_index, j, i);
    }
}
