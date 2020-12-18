use std::io::{BufReader, BufRead, Result};
use std::fs::File;
use regex::Regex;
use lazy_static::lazy_static;

type UnorderedPassword = [usize; 26];

#[derive(Debug)]
pub struct PasswordReq {
    pass: UnorderedPassword,
    target: usize,
    min: usize,
    max: usize,
    ordered_pass: String,
}

lazy_static! {
    static ref PASS_RE: Regex = Regex::new(r"(?P<min>\d+)-(?P<max>\d+) (?P<target>[a-z]): (?P<pass>[a-z]+)").unwrap();
}

pub fn char_to_usize(c: char) -> usize {
    c as usize - 'a' as usize
}

pub fn make_password(name: &str) -> UnorderedPassword {
    let mut pass: UnorderedPassword = [0; 26];
    for c in name.chars() {
        let val = char_to_usize(c);
        pass[val] += 1;
    }

    pass
}

impl PasswordReq {
    pub fn from_str(line: &str) -> Option<Self> {
        match PASS_RE.captures(line) {
            Some(caps) => {
                Some( Self {
                    pass: make_password(caps.name("pass").unwrap().as_str()),
                    min: caps.name("min").unwrap().as_str().parse::<usize>().unwrap(),
                    max: caps.name("max").unwrap().as_str().parse::<usize>().unwrap(),
                    target: char_to_usize(caps.name("target").unwrap().as_str().parse::<char>().unwrap()),
                    ordered_pass: String::from(caps.name("pass").unwrap().as_str()),
                })
            },
            None => None
        }
    }

    pub fn check(&self) -> bool {
        let target_count = self.pass[self.target];
        self.min <= target_count && target_count <= self.max
    }

    pub fn check_2(&self) -> bool {
        let min_index = self.min - 1;
        let max_index = self.max - 1;

        let mut sum: usize = if self.target == char_to_usize(self.ordered_pass.chars().nth(min_index).unwrap()) {
            1
        } else {
            0
        };

        sum += if self.target == char_to_usize(self.ordered_pass.chars().nth(max_index).unwrap()) {
            1
        } else {
            0
        };

        sum == 1
    }
}

pub fn read_it(filename: &str) -> Result<Vec<PasswordReq>> {
    let file = File::open(filename)?;
    let file_reader = BufReader::new(file);

    let mut passes: Vec<PasswordReq> = vec![];

    for line in file_reader.lines() {
        match line {
            Err(why)  => return Err(why),
            Ok(l) => match PasswordReq::from_str(&l) {
                None => {
                    println!("invalid line: {:?}", l);
                }
                Some(req) => passes.push(req),
            }
        }
    }

    Ok(passes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_re() {
        let text = "2-12 a: bzbcccczbzba";
        let text2 = "2-12 b: bbbcccczbzbb";
        let caps = PASS_RE.captures(text).unwrap();

        let pass = make_password(caps.name("pass").unwrap().as_str());
        assert_eq!(pass[1], 4);
        assert_eq!(pass[25], 3);

        let req = PasswordReq::from_str(text).unwrap();
        let req2 = PasswordReq::from_str(text2).unwrap();
        assert_eq!(req.min, 2);
        assert_eq!(req.max, 12);
        assert_eq!(req.target, char_to_usize('a'));

        assert!(!req.check());
        assert!(req2.check());

        assert!(req.check_2());
        assert!(!req2.check_2());
    }

    #[test]
    fn test_it() {
        let passes = read_it("res/02/input.txt").unwrap();
        let part_1 = passes.iter().filter(|x| {
            x.check()
        }).count();
        println!("{} part 1", part_1);

        let part_2 = passes.iter().filter(|x| {
            x.check_2()
        }).count();
        println!("{} part 2", part_2);
    }
}