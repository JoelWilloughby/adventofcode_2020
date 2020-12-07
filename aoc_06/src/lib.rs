
pub struct Group {
    any: u32,
    all: u32,
}

impl Group {
    pub fn new() -> Self {
        Self { any: 0, all: !0 }
    }

    pub fn from_line(line: &str) -> Self {
        let mut g = Self::new();
        g.read_line(line);
        g
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

    fn drive(filename: &str) {
        let file = std::fs::read_to_string(filename).unwrap();

        let mut group: Option<Group> = None;
        let mut any_sum = 0;
        let mut all_sum = 0;
        for li in file.lines() {
            if li.len() > 0 {
                match group {
                    None => { 
                        group = Some(Group::from_line(li)); 
                    },
                    Some(ref mut g) => { 
                        g.read_line(&li); 
                    },
                }
            } else if let Some(g) = group {
                any_sum += g.any.count_ones();
                all_sum += g.all.count_ones();
                group = None;
            }
        }

        if let Some(g) = group {
            any_sum += g.any.count_ones();
            all_sum += g.all.count_ones();
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
