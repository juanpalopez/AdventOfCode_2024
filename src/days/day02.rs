use crate::common::read_file::{read_by_line, split_line_i32};

const MIN_DIFF: i32 = 1;
const MAX_DIFF: i32 = 3;

fn is_safe_report(nums: Vec<i32>, toleration: i32) -> bool {

    let mut is_increasing = true;
    let mut increasing_out_of_tolerance = 0;
    let mut is_decreasing: bool = true;
    let mut decreasing_out_of_tolerance = 0;

    for i in 1..nums.len() {
        let diff = (nums[i] - nums[i-1]).abs();
        if nums[i] < nums[i-1] {
            is_increasing = false;
            if diff < MIN_DIFF || diff > MAX_DIFF {
                decreasing_out_of_tolerance += 1; 
            }
        }
        else if nums[i] > nums[i-1] {
            is_decreasing = false;
            if diff < MIN_DIFF || diff > MAX_DIFF {
                increasing_out_of_tolerance += 1; 
            }
        }
        else {
            is_increasing = false;
            is_decreasing = false;
        }
    }

    (is_decreasing && decreasing_out_of_tolerance <= toleration) || 
    (is_increasing && increasing_out_of_tolerance <= toleration)
}

pub fn part1() -> Result<i32, std::io::Error> {
    let lines = read_by_line("src/days/day02.txt")?;
    let mut count = 0;
    for line in lines {
        let line = line?;
        let nums = split_line_i32(&line);
        // println!("{:?}", nums);
        count += if is_safe_report(nums,0) {1} else {0};
    }
    Ok(count)  
}

pub fn part2() -> Result<i32, std::io::Error> {
    let lines = read_by_line("src/days/day02.txt")?;
    let mut count = 0;
    for line in lines {
        let line = line?;
        let nums = split_line_i32(&line);
        // println!("{:?}", nums);
        count += if is_safe_report(nums,1) {1} else {0};
    }
    Ok(count)  
}

