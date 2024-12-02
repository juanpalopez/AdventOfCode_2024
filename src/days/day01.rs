use std::collections::{BinaryHeap, HashMap};

use crate::common::read_file;

pub fn part1() -> Result<i32, std::io::Error> {
    const FILE_NAME: &str = "src/days/day01a.txt";
    let lines = read_file::read_by_line(FILE_NAME).unwrap();
    let mut sum = 0;

    // instantiate a heap to track increasing values
    let mut heap_left: BinaryHeap<i32> = BinaryHeap::new();
    let mut heap_right: BinaryHeap<i32> = BinaryHeap::new();

    for line in lines {
        let line = line?;
        
        //split on whitespaces
        let nums = line.split_whitespace().collect::<Vec<&str>>();

        let left_value: i32 = nums[0].parse().unwrap();
        let right_value: i32 = nums[1].parse().unwrap();

        heap_left.push(left_value);
        heap_right.push(right_value);        
    }
    
    // while binary heaps are not empty pop the top value and add to sum
    while !heap_left.is_empty() && !heap_right.is_empty() {
        let left_value = heap_left.pop().unwrap();
        let right_value = heap_right.pop().unwrap();
        sum += (left_value - right_value).abs(); 
    }

    Ok(sum)
}

pub fn part2() -> Result<i32, std::io::Error> {
    const FILE_NAME: &str = "src/days/day01a.txt";
    let lines = read_file::read_by_line(FILE_NAME).unwrap();
    let mut left_vector: Vec<i32> = Vec::new();
    let mut sum = 0;

    // instantiate a unordered map to count values
    let mut counts: HashMap<i32,i32> = HashMap::new();

    for line in lines {
        let line = line?;
        //split on whitespaces
        let nums = line.split_whitespace().collect::<Vec<&str>>();
        let left_value: i32 = nums[0].parse().unwrap();
        let right_value: i32 = nums[1].parse().unwrap();
        *counts.entry(right_value).or_insert(0) += 1;
        left_vector.push(left_value);

    }

    for left_value in left_vector{
        let count = counts.get(&left_value);
        match count {
            Some(c) => {
                sum += left_value * c;
            }
            None => {}
        }
    }

    Ok(sum)
}