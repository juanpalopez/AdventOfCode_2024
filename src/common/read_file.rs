use std::fs::File;
use std::io::{self, BufRead};

pub fn read_by_line(file_name: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(file_name)?;
    Ok(io::BufReader::new(file).lines())
}

// TODO: refactor to use generics
pub fn split_line_i32(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}