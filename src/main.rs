use reqwest;

mod days;
mod common;

use crate::days::day01;
use crate::days::day02;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let day01_part1_res = day01::part1().unwrap();
    println!("day01 part1 = {}", day01_part1_res);

    let day01_part2_res = day01::part2().unwrap();
    println!("day01 part2 = {}", day01_part2_res);

    let day02_part1_res = day02::part1().unwrap();
    println!("day02 part1 = {}", day02_part1_res);
    
    let day02_part2_res = day02::part2().unwrap();
    println!("day02 part2 = {}", day02_part2_res);
    Ok(())
}
