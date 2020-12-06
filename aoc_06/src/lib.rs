
pub struct Group {
    any: u32,
    all: u32,
}

impl Group {
    pub fn new() -> Self {
        Self { any: 0, all: !0 }
    }

    pub fn read_line(&mut self, line: &str) {
        let bytes = String::from(line).into_bytes();

        let mut temp: u32 = 0;
        for i in bytes {
            temp |= 1 << (i - 'a' as u8);
        }
     
        self.any |= temp;
        self.all &= temp;
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::{BufReader, BufRead};

    fn drive(filename: &str) {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        let mut group = Group::new();
        let mut working = false;
        let mut any_sum = 0;
        let mut all_sum = 0;
        for line in reader.lines() {
            match line {
                Err(_) => (),
                Ok(li) => {
                    if working && li.len() == 0 {
                        working = false;
                        any_sum += group.any.count_ones();
                        all_sum += group.all.count_ones();
                        group = Group::new();
                    } else if li.len() > 0 {
                        working = true;
                        group.read_line(&li);
                    }
                }
            }
        }

        if working {
            any_sum += group.any.count_ones();
            all_sum += group.all.count_ones();
        }

        println!("Anybody is {}, Allbody is {}", any_sum, all_sum);
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
