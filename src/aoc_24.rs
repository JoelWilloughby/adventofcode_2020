use std::collections::{HashSet, HashMap};
use lazy_static::lazy_static;

#[derive(Debug)]
pub struct Tiles {
    tiles: HashSet<(isize, isize, isize)>,
}

fn add_tuple(lhs: (isize, isize, isize), rhs: (isize, isize, isize)) -> (isize, isize, isize) {
    (lhs.0 + rhs.0, lhs.1 + rhs.1, lhs.2 + rhs.2)
}

impl Tiles {
    pub fn new() -> Self {
        Self { tiles: HashSet::new(), }
    }

    pub fn read_line(&mut self, line: &str) -> (isize, isize, isize) {
        let mut line_bytes = line.as_bytes();
        let mut dir = (0, 0, 0);
        while !line_bytes.is_empty() {
            let mut j = 1;
            match line_bytes[0] as char {
                's' => {
                    j += 1;
                    match line_bytes[1] as char {
                        'e' => {
                            dir.1 += 1;
                            dir.2 -= 1;
                        },
                        'w' => {
                            dir.0 -= 1;
                            dir.1 += 1;
                        },
                        _ => panic!("Bad input"),
                    };
                },
                'n' => {
                    j += 1;
                    match line_bytes[1] as char {
                        'e' => {
                            dir.0 += 1;
                            dir.1 -= 1;
                        },
                        'w' => {
                            dir.1 -= 1;
                            dir.2 += 1;
                        },
                        _ => panic!("Bad input"),
                    };
                }
                'e' => {
                    dir.0 += 1;
                    dir.2 -= 1;
                },
                'w' => {
                    dir.0 -= 1;
                    dir.2 += 1;
                }
                _ => panic!("Bad input"),
            }
            line_bytes = &line_bytes[j..];
        }

        if self.tiles.contains(&dir) {
            self.tiles.remove(&dir);
        } else {
            self.tiles.insert(dir);
        }

        dir
    }

    pub fn step(&mut self) {
        lazy_static!(
            static ref NEIGHBORS: [(isize, isize, isize); 6] = [(1, -1, 0), (1, 0, -1), (-1, 1, 0), (-1, 0, 1), (0, 1, -1), (0, -1, 1)];
        );
        let mut white_tiles_black_neighbors = HashMap::new();
        let mut new_tiles = HashSet::new();
        for tile in self.tiles.iter() {
            let num_black_neighbors = NEIGHBORS.iter().fold(0, |acc, x| acc + if self.tiles.contains(&add_tuple(*x, *tile)) {1} else {0});
            if num_black_neighbors == 1 || num_black_neighbors == 2 {
                // It stays black
                new_tiles.insert(*tile);
            }

            for neighbor in NEIGHBORS.iter() {
                *white_tiles_black_neighbors.entry(add_tuple(*neighbor, *tile)).or_insert(0) += 1;
            }
        }

        for (white_tile, count) in white_tiles_black_neighbors {
            if count == 2 {
                new_tiles.insert(white_tile);
            }
        }

        self.tiles = new_tiles;
    }

    pub fn count(&self) -> usize {
        self.tiles.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn drive(filename: &str) {
        let input = std::fs::read_to_string(filename).unwrap();
        let mut tiles = Tiles::new();

        for line in input.lines() {
            tiles.read_line(line);
        }

        println!("Part 1: {}", tiles.count());

        for _ in 0..100 {
            tiles.step();
        }

        println!("Part 2: {}", tiles.count());
    }

    #[test]
    fn it_works() {
        drive("res/24/input_simple.txt");
    }

    #[test]
    fn test_it() {
        drive("res/24/input.txt");
    }
}