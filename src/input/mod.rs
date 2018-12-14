use std::io::{BufRead, BufReader, Read};
use std::fs::File;

pub fn reader(day: u8) -> BufReader<File> {
    let filename = format!("src/input/{}", day);
    let file = File::open(filename).unwrap();
    BufReader::new(file)
}


pub fn read_to_lines(reader: BufReader<File>) -> Vec<String> {
    reader.lines().map(|line|
        line.unwrap().to_string()
    ).collect()
}

pub fn read_to_string(day: u8) -> String {
    let mut buf = String::new();
    let mut reader = reader(day);
    reader.read_to_string(&mut buf).unwrap();
    buf
}