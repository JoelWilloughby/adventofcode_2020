use std::fmt::{Formatter, Error};

#[derive(Debug)]
pub struct Cups {
    next_list: Vec<usize>,
    head: usize,
}

fn mod_sub(num: usize, len: usize) -> usize {
    if num == 1 {
        len
    } else {
        num - 1
    }
}

impl std::fmt::Display for Cups {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let mut curr = self.head;
        loop {
            write!(f, "{} ", curr)?;
            curr = self.next_list[curr];
            if curr == self.head {
                break;
            }
        }
        Ok(())
    }
}

impl Cups {
    pub fn new(nums: &[usize]) -> Self {
        let mut next_list = vec![0; 10];
        for i in 0..8 {
            next_list[nums[i]] = nums[i+1];
        }
        next_list[nums[8]] = nums[0];
        Self {
            next_list,
            head: nums[0]
        }
    }

    pub fn extend_pt_2(&mut self) {
        // Get the last value:
        let mut curr = self.head;
        for _ in 0..8 {
            curr = self.next_list[curr];
        }
        for i in 10..=999999 {
            self.next_list.push(i+1);
        }
        self.next_list[curr] = 10;
        self.next_list.push(self.head);
    }

    pub fn turn(&mut self) {
        let c1 = self.next_list[self.head];
        let c2 = self.next_list[c1];
        let c3 = self.next_list[c2];

        let mut dest_cup = mod_sub(self.head, self.next_list.len() - 1);
        for _ in 0..3 {
            if dest_cup == c1 || dest_cup == c2 || dest_cup == c3 {
                dest_cup = mod_sub(dest_cup, self.next_list.len() - 1);
            }
        }

        let temp = self.next_list[dest_cup];
        self.next_list[dest_cup] = c1;
        self.next_list[self.head] = self.next_list[c3];
        self.next_list[c3] = temp;

        self.head = self.next_list[self.head];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn drive(input: &[usize]) {
        let mut cups = Cups::new(input);

        println!("Starting:  {}", cups);
        for _ in 0..100 {
            cups.turn();
        }
        println!("After 100: {}", cups);

        let mut cups = Cups::new(input);
        cups.extend_pt_2();
        for _ in 0..10000000 {
            cups.turn();
        }
        let v1 = cups.next_list[1];
        let v2 = cups.next_list[v1];
        println!("After 10000000: {} {} {}", v1, v2, v1 * v2);
    }

    #[test]
    fn it_works() {
        drive(&[3, 8, 9, 1 ,2, 5, 4, 6, 7]);
    }

    #[test]
    fn test_it() {
        drive(&[9, 1, 6, 4, 3, 8, 2, 7, 5]);
    }
}