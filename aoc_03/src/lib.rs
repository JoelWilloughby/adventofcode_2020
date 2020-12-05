use std::io::BufRead;

#[derive(Debug)]
pub struct Forest {
    trees: Vec<Vec<u8>>,
    width: usize,
}

impl Forest {
    pub fn from_reader<R: BufRead> (reader: &mut R) -> Self {
        let mut rows: Vec<Vec<u8>> = vec![];
        for line in reader.lines() {
            match line {
                Err(why) => {
                    println!("Problem line: why: {:?}", why);
                },
                Ok(l) => rows.push(l.into_bytes()),
            }
        }

        let width = rows[0].len();
        for row in rows.iter() {
            if row.len() != width {
                println!("Row bad length");
            }
        }

        Self {
            trees: rows,
            width: width,
        }
    }

    pub fn check(&self, right: usize, down: usize) -> usize {
        let mut col = 0;
        let mut count = 0;
        for row in (0..self.trees.len()).step_by(down) {
            if self.trees[row][col] == '#' as u8 {
                count += 1;
            }
            col = (col + right) % self.width; 
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::{BufReader};

    fn drive(filename: &str) {
        let file = File::open(filename).unwrap();
        let mut reader = BufReader::new(file);

        let forest = Forest::from_reader(&mut reader);

        println!("Num rows: {}", forest.trees.len());
        println!("Part 1: {}", forest.check(3, 1));

        for (x, y) in &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)] {
            println!("Trees for ({}, {}): {}", x, y, forest.check(*x, *y));
        }
    }

    #[test]
    fn simple() {
        drive("res/input_simple.txt");
    }

    #[test]
    fn do_it() {
        drive("res/input.txt");
    }
}
