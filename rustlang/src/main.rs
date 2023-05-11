use std::{error::Error, fs::File, sync::atomic};

// use serde::{Deserialize, Serialize};
// use serde_json::Value;

const AP_TEMP: &str = "LOTUS_AP_TEMP";
const AP_COMMITMENT_FILE: &str = "ap_commitment.json";

fn load_commitment() -> Result<[u8; 32], Box<dyn Error>> {
    let ap_temp_path = std::env::var(AP_TEMP)?;
    let commitment_path = std::path::Path::new(ap_temp_path.as_str()).join(AP_COMMITMENT_FILE);
    // println!("commitment_path: {:?}", commitment_path);

    let file = File::open(commitment_path)?;
    let commitment: [u8; 32] = serde_json::from_reader(file)?;
    Ok(commitment)
}

fn save_commitment(commitment: [u8; 32]) -> Result<(), Box<dyn Error>> {
    let ap_temp_path = std::env::var(AP_TEMP)?;
    let commitment_path = std::path::Path::new(ap_temp_path.as_str()).join(AP_COMMITMENT_FILE);

    let commitment_file = File::create(commitment_path)?;
    serde_json::to_writer(commitment_file, &commitment)?;
    Ok(())
}

fn af() -> Result<(), std::io::Error> {
    let mut commitment: [u8; 32];

    let commitment_e = load_commitment();
    if let Ok(_e) = commitment_e {
        return Ok(());
    }

    match commitment_e {
        Ok(c) => {
            commitment = c;
            println!("load: ok, {:?}", commitment);
            return Ok(());
        }
        Err(e) => {
            println!("load: {}", e);
            commitment = [0; 32]
        }
    }

    commitment[3] = commitment[3] + 1;
    if let Err(e) = save_commitment(commitment) {
        println!("save: {}", e);
    }

    Ok(())
}

fn main() {
    println!("Hello, world!");
    af().unwrap();

    log::debug!("some error")
}

#[test]
fn f2() {
    let i: i32 = 10;
    println!("{},{}", "abc", i);
}
