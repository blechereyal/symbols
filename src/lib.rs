use once_cell::sync::Lazy;
use std::{fs::File, io::Read};
pub static US_STOCKS: Lazy<Vec<String>> = Lazy::new(|| {
    let mut file = File::open("src/us_stocks.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.split("\n").map(|e| e.to_owned()).collect()
});

pub mod advanced_method;
pub mod intial_method;
pub mod symbol;
