use std::io::BufRead;
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref PAIR_RE: Regex = Regex::new(r"([a-z]+:\S+)").unwrap();
}

#[derive(Debug)]
struct CheckedString {
    value: String,
    valid: bool,
}

#[derive(Debug)]
struct Id {
    byr: Option<CheckedString>,
    iyr: Option<CheckedString>,
    eyr: Option<CheckedString>,
    hgt: Option<CheckedString>,
    hcl: Option<CheckedString>,
    ecl: Option<CheckedString>,
    pid: Option<CheckedString>,
    cid: Option<String>,
}

fn valid_year(input: &str, min: usize, max: usize) -> CheckedString {
    let valid = match input.parse::<usize>() {
        Ok(year) => if year <= max && year >= min {
            true
        } else {
            false
        },
        _ => false,
    };

    CheckedString {
        value: String::from(input),
        valid,
    }
}

fn valid_height(input: &str) -> CheckedString {
    lazy_static! {
        static ref HEIGHT_RE: Regex = Regex::new(r"^(\d+)(cm|in)$").unwrap();
    }

    let valid = match HEIGHT_RE.captures(input) {
        Some(cap) => {
            match cap[1].parse::<usize>() {
                Ok(height) => {
                    match &cap[2] {
                        "cm" => height >= 150 && height <= 193,
                        "in" => height >= 59 && height <= 76,
                        _ => false,
                    }
                },
                _ => false,
            }
        },
        None => {
            false
        }
    };
    CheckedString {
        value: String::from(input),
        valid,
    }
}

fn valid_pid(input: &str) -> CheckedString {
    lazy_static! {
        static ref PID_RE: Regex = Regex::new(r"^\d{9}$").unwrap();
    }

    CheckedString {
        value: String::from(input),
        valid: PID_RE.is_match(input),
    }
}

fn valid_hair_color(input: &str) -> CheckedString {
    lazy_static! {
        static ref COLOR_RE: Regex = Regex::new(r"^#[a-f0-9]{6}$").unwrap();
    }

    CheckedString {
        value: String::from(input),
        valid: COLOR_RE.is_match(input),
    }
}

fn valid_eye_color(input: &str) -> CheckedString {
    lazy_static! {
        static ref COLORS: Vec<&'static str> = vec![
            "amb", "blu", "brn", "gry", "grn", "hzl", "oth"
        ];
    }

    CheckedString {
        value: String::from(input),
        valid: COLORS.contains(&input),
    }
}

fn check_entry(entry: &Option<CheckedString>) -> bool {
    match entry {
        None => false,
        Some(cs) => cs.valid,
    }
}

impl Id {
    fn new() -> Self {
        Self {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        }
    }

    fn set_item(&mut self, key: &str, value: &str) {
        match key {
            "byr" => { self.byr = Some(valid_year(value, 1920, 2002)); },
            "iyr" => { self.iyr = Some(valid_year(value, 2010, 2020)); },
            "eyr" => { self.eyr = Some(valid_year(value, 2020, 2030)); },
            "hgt" => { self.hgt = Some(valid_height(value)); },
            "pid" => { self.pid = Some(valid_pid(value)); },
            "hcl" => { self.hcl = Some(valid_hair_color(value)); },
            "ecl" => { self.ecl = Some(valid_eye_color(value)); },
            "cid" => { self.cid = Some(String::from(value)); },
            _ => {}
        }
    }

    fn read_helper<R: BufRead>(line: &str, reader: &mut R) -> Self {
        let mut temp = Self::new();
        let mut iter = reader.lines();
        let mut current = String::from(line);
        
        loop {
            for cap in PAIR_RE.captures_iter(&current) {
                let strs: Vec<&str> = cap[0].split(':').collect();
                temp.set_item(strs[0], strs[1]);
            }

            match iter.next() {
                Some(thing) => {
                    match thing {
                        Err(_) => {
                            break;
                        },
                        Ok(li) => {
                            if li.len() == 0 {
                                break;
                            }

                            current = String::from(li);
                        }
                    }
                },
                None => {
                    break;
                }
            }
        }

        temp
    }

    pub fn from_reader<R: BufRead>(reader: &mut R) -> Option<Self> {
        for line in reader.lines() {
            match line {
                Ok(li) => {
                    if li.len() > 0 {
                        return Some(Self::read_helper(&li, reader));
                    } 
                },
                Err(_) => {
                    return None;
                }
            }
        }

        None
    }

    pub fn check_present(&self) -> bool {
        self.byr.is_some() && self.iyr.is_some() && self.eyr.is_some() && self.hgt.is_some() && self.hcl.is_some() && self.ecl.is_some() && self.pid.is_some()
    }

    pub fn check_valid(&self) -> bool {
        check_entry(&self.byr) &&
        check_entry(&self.iyr) &&
        check_entry(&self.eyr) &&
        check_entry(&self.hgt) &&
        check_entry(&self.hcl) &&
        check_entry(&self.ecl) &&
        check_entry(&self.pid) 
    }
}


#[cfg(test)]
mod tests {    use super::*;
    use std::fs::File;
    use std::io::{BufReader};

    fn drive(filename: &str) {
        let file = File::open(filename).unwrap();
        let mut reader = BufReader::new(file);

        let mut valid_count = 0;
        let mut present_count = 0;
        let mut ids = vec![];

        loop {
            let passport = Id::from_reader(&mut reader);
            match passport {
                None => {
                    break;
                },
                Some(id) => {
                    let valid = id.check_valid();
                    if id.check_valid() {
                        valid_count += 1;
                    } 
                    if id.check_present() {
                        present_count += 1;
                    }

                    ids.push(id);
                }
            }
        }

        println!("present: {}, Total: {}", present_count, ids.len());
        println!("valid: {}, Total: {}", valid_count, ids.len());
    }


    #[test]
    fn it_works() {
        drive("res/input_simple.txt");
    }

    #[test]
    fn test_it() {
        drive("res/input.txt");
    }
}
