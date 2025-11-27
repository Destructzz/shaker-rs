use std::{thread::{self}, time::Duration};

mod shaker;
use shaker::Shaker;
mod errors;

const BAUDE_RATE: u32 = 2_000_000;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut sh = Shaker::new(BAUDE_RATE);
    sh.init()?;

    

    loop {
        if let Err(e) = sh.process(50, 10) {
            println!("Something happened: {:?}", e);
        }
        thread::sleep(Duration::from_millis(500));
    }
}
