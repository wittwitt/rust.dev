use std::{
    fs::{File, OpenOptions},
    path::PathBuf,
};

fn main() {
    {
        let data_path = "/home/sh/tmp/miner-wdpost-nocommit1111.log.hhh";
        let file = File::open(&data_path).unwrap();
        let metadata = file.metadata().unwrap();
        let store_size = metadata.len() as usize;
        println!("{}", store_size);
    }

    {
        let data_path = "/home/sh/tmp/miner-wdpost-nocommit1111.log.link";
        let file = File::open(&data_path).unwrap();
        let metadata = file.metadata().unwrap();
        let store_size = metadata.len() as usize;
        println!("{}", store_size);
    }

    {
        let data_path = "/home/sh/tmp/miner-wdpost-nocommit1111.log.link";
        let f = OpenOptions::new().read(true).open(&data_path).unwrap();
        let metadata = f.metadata().unwrap();
        let store_size = metadata.len() as usize;
        println!("{}", store_size);
    }
}

#[test]
fn f1() {
    let p = PathBuf::from("t.txt");

    let np = p.with_extension(".tmp");
    assert_eq!(np.to_str().unwrap(), "t..tmp", "");

    let np = p.with_extension("tmp");
    assert_eq!(np.to_str().unwrap(), "t.tmp", "");

    let p = PathBuf::from("t");

    let np = p.with_extension("tmp");
    assert_eq!(np.to_str().unwrap(), "t.tmp", "");

    let p = PathBuf::from("t.io.txt");

    let np = p.with_extension("tmp");
    assert_eq!(np.to_str().unwrap(), "t.io.tmp", "");
}
