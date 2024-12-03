use crate::common::read_file::{read_by_line, split_line_i32};

const MIN_DIFF: i32 = 1;
const MAX_DIFF: i32 = 3;

pub fn part1() -> Result<i32, std::io::Error> {
    let lines = read_by_line("src/days/day02.txt")?;
    let mut count = 0;
    for line in lines {
        let line = line?;
        let nums = split_line_i32(&line);
        let is_safe = is_increasing(&nums) || is_decreasing(&nums);
        count += if is_safe {1} else {0};
    }
    Ok(count)  
}

fn is_increasing(nums: &Vec<i32>) -> bool {
    for i in 1..nums.len() {
        let diff = nums[i] - nums[i-1];
        if !(diff >= MIN_DIFF && diff <= MAX_DIFF) {
            return false;
        }
        
    }
    true
}

fn is_decreasing(nums: &Vec<i32>) -> bool {
    for i in 1..nums.len() {
        let diff = nums[i-1]-nums[i];
        if !(diff >= MIN_DIFF && diff <= MAX_DIFF) {
            return false;
        }
        
    }
    true
}

fn is_safe_removing(nums: Vec<i32>) -> bool {
    for i in 0..nums.len() {
        let mut mod_nums = nums.clone();
        mod_nums.remove(i);
        let is_safe = is_increasing(&mod_nums) || is_decreasing(&mod_nums);
        if is_safe {
            return true;
        }
    }
    false
}

pub fn part2() -> Result<i32, std::io::Error> {
    let lines = read_by_line("src/days/day02.txt")?;
    let mut count = 0;
    for line in lines {
        let line = line?;
        let nums = split_line_i32(&line);
        // println!("{:?}", nums);
        let is_safe = is_safe_removing(nums); 
        count += if is_safe {1} else {0};
    }
    Ok(count)  
}

