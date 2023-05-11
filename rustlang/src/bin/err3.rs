fn main() {
    match say(15) {
        Ok(s) => {
            println!("OK: {}", s)
        }
        Err(e) => match e {
            CError::YOULITE => {
                println!("ERROR: to lite")
            }
            CError::GO(i) => {
                println!("ERROR: i: {}", i)
            }
        },
    }
}

fn say(i: i32) -> Result<String, CError> {
    if i < 10 {
        return Err(CError::YOULITE);
    } else if i < 20 {
        return Err(CError::GO(i));
    }
    Ok(String::from("okok"))
}

enum CError {
    YOULITE,
    GO(i32),
}
