use std::io::{BufReader, BufRead, Result};
use std::fs::File;

fn read_it(filename: &str) -> Result<Vec<usize>> {
    let file = File::open(filename)?;
    let file_reader = BufReader::new(file);

    let mut nums: Vec<usize> = vec![];

    for line in file_reader.lines() {
        match line {
            Err(why)  => return Err(why),
            Ok(l) => match l.trim().parse::<usize>() {
                Err(_) => {
                    println!("Non num found: {:?}", l);
                }
                Ok(number) => nums.push(number),
            }
        }
    }

    Ok(nums)
}

fn vec_to_bool_array(nums: &Vec<usize>) -> [bool; 2021] {
    let mut num_set: [bool; 2021] = [false; 2021];

    for num in nums.iter() {
        num_set[*num] = true;
    }

    num_set
}

fn search_for_two(nums: &Vec<usize>, num_set: &[bool; 2021]) {
    for num in nums.iter() {
        let diff: usize = (2020 - num) as usize;
        if num_set[diff] {
            println!("Found match: {:?} {:?}", num, diff);
        }
    }
}

fn search_for_three(nums: &Vec<usize>, num_set: &[bool; 2021]) {
    for num in nums.iter() {
        for num2 in nums.iter() {
            let diff: isize = 2020 - *num as isize - *num2 as isize;
            if diff < 0 {
                continue;
            }

            let diff: usize = diff as usize;
            if num_set[diff] {
                println!("Found match: {:?} {:?} {:?}", num, num2, diff);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it() {
        let nums = read_it("res/input.txt").unwrap();
        println!("Read {:?} lines", nums.len());
        println!("{:?}", nums);
        
        let num_set = vec_to_bool_array(&nums);
        search_for_two(&nums, &num_set);
        search_for_three(&nums, &num_set);
    }
}