use std::collections::HashMap;

pub fn do_it(nums: &Vec<usize>, window: usize) -> Option<usize> {
    assert!(nums.len() >= window);

    let mut working_nums: HashMap<usize, usize> = HashMap::new();
    for num in nums.iter().take(window) {
        *working_nums.entry(*num).or_insert(0) += 1;
    }

    for i in window..nums.len() {
        let num = nums[i];
        let mut found = false;
        for (diff, _count) in working_nums.iter() {
            if num < *diff {
                continue;
            }
            if working_nums.contains_key(&(num - diff)) {
                found = true;
                break;
            }
        }

        if found {
            let back = nums[i - window];
            let rem = working_nums.remove(&back).unwrap();
            if rem > 1 {
                working_nums.insert(back, rem - 1);
            }
        } else {
            return Some(num);
        }

        *working_nums.entry(num).or_insert(0) += 1;
    }

    None
}

pub fn do_it_2(nums: &Vec<usize>, target: usize) -> Option<usize> {
    let mut low = 0;
    let mut high = 1;

    let target = target as isize;
    let mut accum = nums[low] as isize + nums[high] as isize;
    while accum != target {
        if accum > target {
            accum -= nums[low] as isize;
            low += 1;            
        } else if accum < target {
            high += 1;
            accum += nums[high] as isize;
        } 

        if low == high || high >= nums.len() {
            return None;
        }
    }
    
    Some(
        // Lol
        nums[low..=high].iter().min().unwrap() + 
        nums[low..=high].iter().max().unwrap() 
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn drive(filename: &str, window: usize) {
        let input = std::fs::read_to_string(filename).unwrap();

        let nums = input
            .lines()
            .map(|line| line.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let part_1 = do_it(&nums, window);
        println!("Do it: {:?}", part_1);
        println!("Do it 2: {:?}", do_it_2(&nums, part_1.unwrap()));
    }

    #[test]
    fn it_works() {
        drive("res/09/input_simple.txt", 5);
    }

    #[test]
    fn test_it() {
        drive("res/09/input.txt", 25);
    }
}
