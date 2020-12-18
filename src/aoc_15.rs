use std::collections::HashMap;

#[derive(Debug)]
pub struct Doit {
    initial_nums: Vec<usize>,
    nums: HashMap<usize, usize>,
    current: usize,
    current_index: usize,
}

impl Doit {
    pub fn doit(nums: &Vec<usize>) -> Self {
        Self {
            initial_nums: nums.clone(),
            nums: HashMap::new(),
            current: nums[0],
            current_index: 0,
        }
    }
}

impl Iterator for Doit {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        // println!("{:?}", self);
        if self.current_index < self.initial_nums.len() {
            self.current = self.initial_nums[self.current_index];
            if self.current_index < self.initial_nums.len() - 1 {
                self.nums.insert(self.current, self.current_index + 1);
            }
        } else {
            let last = self.current;
            self.current = 
                if let Some(val) = self.nums.get(&self.current) {
                    self.current_index - val
                } else {
                    0
                };
            self.nums.insert(last, self.current_index);
        }

        self.current_index += 1;
        Some(self.current)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn drive(filename: &str) {
        let input = std::fs::read_to_string(filename).unwrap();
        let nums = input
            .split(",")
            .filter_map(|val| val.parse::<usize>().ok())
            .collect::<Vec<usize>>();

        let doit = Doit::doit(&nums);
        for (i, val) in doit.enumerate().take(2020) {
            println!("{:4} {:>4}", i+1, val);
        }

        println!("Big one {}", Doit::doit(&nums).nth(29999999).unwrap());
    }

    #[test]
    fn it_works() {
        drive("res/15/input_simple.txt");
    }

    #[test]
    fn test_it() {
        drive("res/15/input.txt");
    }
}
