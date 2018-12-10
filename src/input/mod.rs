use std::io::{BufRead, BufReader};
use std::fs::File;

pub fn read(day: u8) -> Vec<String> {
    let filename = format!("src/input/{}", day);
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    reader.lines().map(|line|
        line.unwrap().to_string()
    ).collect()
}