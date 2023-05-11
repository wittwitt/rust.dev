use std::io::Read;

fn main() {
    match std::fs::File::open("./READMD.md") {
        Ok(mut f) => {
            let mut str = String::new();
            match f.read_to_string(&mut str) {
                Ok(_size) => println!("{}", str),
                Err(e) => println!("read: {}", e),
            }
        }
        Err(e) => {
            println!("open: {}", e)
        }
    }
}
