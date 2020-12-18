use regex::Regex;
use lazy_static::lazy_static;

#[derive(Debug)]
struct Range {
    upper: usize,
    lower: usize,
}

impl Range {
    fn in_range(&self, val: usize) -> bool {
        val <= self.upper && val >= self.lower
    }
}

#[derive(Debug)]
pub struct Restriction {
    name: String,
    ranges: Vec<Range>,
}

impl Restriction {
    pub fn from_line(line: &str) -> Option<Self> {
        lazy_static! {
            static ref LINE_RE: Regex = Regex::new(r"([\w ]+): (\d+)-(\d+)").unwrap();
            static ref MORE_RE: Regex = Regex::new(r"or (\d+)-(\d+)").unwrap();
        }
        let mat = LINE_RE.find(line)?;
        let caps = LINE_RE.captures(&line[..mat.end()])?;

        let mut ranges = vec![Range {
            upper: caps[3].parse::<usize>().ok()?,
            lower: caps[2].parse::<usize>().ok()?,
        }];

        let mut rem = &line[mat.end()..];
        while let Some(mat) = MORE_RE.find(rem) {
            let caps = MORE_RE.captures(rem)?;

            ranges.push(Range {
                upper: caps[2].parse::<usize>().ok()?,
                lower: caps[1].parse::<usize>().ok()?,
            });

            rem = &rem[mat.end()..]
        }

        Some(Self {
            name: caps[1].to_string(),
            ranges,
        })
    }

    pub fn satisfied_by(&self, num: usize) -> bool {
        self.ranges.iter().any(|r| r.in_range(num))
    }
}

pub fn do_it(restrictions: &Vec<Restriction>, tickets: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut possibilities = restrictions
        .iter()
        .map(|_| (1 << restrictions.len()) - 1)
        .collect::<Vec<usize>>();

    // Seed the bitmap. We are basically playing sudoku
    for ticket in tickets.iter() {
        for (n, num) in ticket.iter().enumerate() {
            for (r, res) in restrictions.iter().enumerate() {
                if !res.satisfied_by(*num) {
                    possibilities[n] &= !(1 << r);
                }
            }
        }
    }

    // Need to essentially relax each edge at least once. This outer loop
    // will ensure that we cover all possibilities
    for _i in 0..possibilities.len() {
        for j in 0..possibilities.len() {
            if possibilities[j].count_ones() == 1 {
                for k in 0..possibilities.len() {
                    if j != k {
                        possibilities[k] &= !(possibilities[j]);
                    }
                }
            }
        }
    }

    possibilities
}

#[cfg(test)]
mod tests {
    use super::*;

    fn drive(filename: &str) {
        let input = std::fs::read_to_string(filename).unwrap();
        let mut lines = input.lines();
        let mut restrictions = vec![];
        while let Some(restriction) = Restriction::from_line(lines.next().unwrap()) {
            restrictions.push(restriction);
        }

        let mut lines = lines.skip_while(|line| line.trim() != "your ticket:");
        lines.next();

        let my_input = lines.next().unwrap()
            .split(",")
            .map(|val| val.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let mut lines = lines.skip_while(|line| line.trim() != "nearby tickets:");
        lines.next();

        let mut bad_num_sum = 0;
        let mut tickets = vec![];
        for line in lines {
            let nums = line
                .split(",")
                .map(|val| val.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            let mut bad = false;
            for num in nums.iter() {
                if restrictions.iter().find(|r| r.satisfied_by(*num)).is_none() {
                    bad_num_sum += num;
                    bad = true;
                }
            }

            if !bad {
                tickets.push(nums);
            }
        }

        println!("--- KEY ---");
        let possibilities = do_it(&restrictions, &tickets);
        let mut new_possibilties =  vec![];
        for p in possibilities.iter() {
//            println!("{:024b} {}", p, p.trailing_zeros());
            new_possibilties.push(restrictions[p.trailing_zeros() as usize].name.to_string());
        }

        let mut acc = 1;
        for (i, r) in new_possibilties.iter().enumerate() {
            println!("{} - {}", r, my_input[i]);
            if r.starts_with("departure") {
                acc *= my_input[i];
            }
        }
        println!("--- KEY ---\n\n");
        println!("Part 1:  {}", bad_num_sum);
        println!("Part 2: {}", acc);

    }

    #[test]
    fn water() {
        drive("res/16/input_2.txt");
    }

    #[test]
    fn it_works() {
        drive("res/16/input_simple.txt");
    }

    #[test]
    fn test_it() {
        drive("res/16/input.txt");
    }
}
