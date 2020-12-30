use regex::Regex;
use lazy_static::lazy_static;
use std::collections::{HashSet, HashMap};

#[derive(Debug, Clone)]
pub struct Tile {
    id: usize,
    sides: [usize; 8],
    content: [[bool; 10]; 10],
    flipped_rows: bool,
    flipped_cols: bool,
    inverted: bool,
    oriented: bool,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "id {}", self.id)?;

        for i in 0..10 {
            for j in 0..10 {
                let b = self.get_oriented(i, j);
                write!(f, "{}", if b {'#'} else {'.'})?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

fn shift_reverse(val: usize) -> usize {
    ((val as u16).reverse_bits() >> 6) as usize
}

impl Tile {
    fn from_lines(id: usize, lines: Vec<&str>) -> Self {
        let mut up = 0;
        let mut down = 0;
        let mut left = 0;
        let mut right = 0;
        let mut content = [[false; 10]; 10];
        for (i, line) in lines.iter().enumerate() {
            for (j, c) in line.bytes().enumerate() {
                let b = if c == '#' as u8 {1} else {0};
                if b == 1 {
                    content[i][j] = true;
                }
                if i == 0 {
                    up <<= 1;
                    up += b;
                } else if i == lines.len() - 1 {
                    down <<= 1;
                    down += b;
                }
                if j == 0 {
                    left <<= 1;
                    left += b;
                } else if j == line.len() - 1 {
                    right <<= 1;
                    right += b;
                }
            }
        }

        Self {
            id,
            sides: [
                left, shift_reverse(left),
                up, shift_reverse(up),
                right, shift_reverse(right),
                down, shift_reverse(down),
            ],
            content,
            flipped_cols: false,
            flipped_rows: false,
            inverted: false,
            oriented: false,
        }
    }

    pub fn from_input<'a, 'b>(mut lines: &'a[&'b str]) -> (Option<Self>, &'a[&'b str])
    {
        lazy_static!(
            static ref TILE_RE: Regex = Regex::new(r"^Tile (\d{4}):$").unwrap();
        );

        let mut id = None;
        while !lines.is_empty() {
            if let Some(caps) = TILE_RE.captures(lines[0]) {
                // Got a valid line
                id = Some(caps[1].parse::<usize>().unwrap());
                break;
            }
            lines = &lines[1..];
        }

        (
            if let Some(id) = id {
                let rows = lines[1..11].iter().map(|s| *s).collect();
                lines = &lines[11..];
                Some(Self::from_lines(id, rows))
            } else {
                None
            },
            lines
        )
    }

    fn orient_indices(&self, mut i: usize, mut j: usize) -> (usize, usize) {
        if self.flipped_cols {
            i = self.content.len() - i - 1;
        }
        if self.flipped_rows {
            j = self.content[0].len() - j - 1;
        }
        if self.inverted {
            std::mem::swap(&mut i, &mut j)
        }

        (i, j)
    }

    fn get_oriented(&self, i: usize, j: usize) -> bool {
        let (i, j) = self.orient_indices(i, j);
        self.content[i][j]
    }

    fn is_oriented(&self, other: &Self, target_dir: usize) -> bool {
        (0..10).all(|j| match target_dir {
            0 => self.get_oriented(j, 0) == other.get_oriented(j, 9),
            1 => self.get_oriented(0, j) == other.get_oriented(9, j),
            2 => self.get_oriented(j, 9) == other.get_oriented(j, 0),
            3 => self.get_oriented(9, j) == other.get_oriented(0, j),
            _ => false,
        })
    }

    pub fn orient_to(&mut self, other: &Self, dir: usize) -> bool {
        assert!(other.oriented);
        let target_idx = (dir+2) % 4;
        if self.oriented {
            return self.is_oriented(other, target_idx);
        }

        for i in 0..8 {
            self.flipped_cols = i & 0x01 != 0;
            self.flipped_rows = i & 0x02 != 0;
            self.inverted = i & 0x04 != 0;

            if self.is_oriented(other, target_idx) {
                self.oriented = true;
                return true;
            }
        }

        false
    }
}

pub fn doit(tiles: &Vec<Tile>) -> HashMap<usize, HashSet<usize>> {
    let mut assocs: Vec<Vec<usize>> = vec![vec![]; 1024];
    for (idx, tile) in tiles.iter().enumerate() {
        for i in 0..8 {
            assocs[tile.sides[i]].push(idx);
        }
    }

    let mut mingle: HashMap<usize, HashSet<usize>> = HashMap::new();
    for i in 0..1024 {
        for l in assocs[i].iter() {
            for r in assocs[i].iter() {
                if l != r {
                    mingle.entry(*l).or_insert(HashSet::new()).insert(*r);
                    mingle.entry(*r).or_insert(HashSet::new()).insert(*l);
                }
            }
        }
    }

    mingle
}

pub fn orient_tiles(tiles: &mut Vec<Tile>, mingle: &HashMap<usize, HashSet<usize>>) -> Vec<Vec<Tile>> {
    // Find first corner
    let mut corner = 0usize;
    for (i, assocs) in mingle.iter() {
        if assocs.len() <= 2 {
            corner = *i;
        }
    }

    let corner_assocs = mingle.get(&corner).unwrap().iter().map(|x| *x).collect::<Vec<usize>>();
    tiles[corner].oriented = true;
    let corner_tile = tiles[corner].clone();

    // Found a corner. Make it "top left"
    let mut neighbor_horiz = corner_assocs[0];
    let mut neighbor_vert = corner_assocs[1];
    let mut horiz_dir = (0..4).find(|i| tiles[neighbor_horiz].clone().orient_to(&corner_tile, *i)).unwrap();
    let mut vert_dir = (0..4).find(|i| tiles[neighbor_vert].clone().orient_to(&corner_tile, *i)).unwrap();

    if horiz_dir & 0x01 != 0 {
        std::mem::swap(&mut horiz_dir, &mut vert_dir);
        std::mem::swap(&mut neighbor_horiz, &mut neighbor_vert);
    }

//    println!("\n\n---------------\nHoriz {}, Vert {}", horiz_dir, vert_dir);

    let dims = (tiles.len() as f32).sqrt() as usize;
    assert_eq!(dims * dims, tiles.len());
    let mut ordered_tiles = vec![];
    let mut current = corner;
    for r in 0..dims {
        let mut row = vec![];
        // Find below neighbor
        let current_tile = tiles[current].clone();
        let next = mingle[&current].iter().find(|n| tiles[**n].orient_to(&current_tile, vert_dir));
        for c in 0..dims {
            let current_tile = tiles[current].clone();
            let next = mingle[&current].iter().find(|n| tiles[**n].orient_to(&current_tile, horiz_dir));
            row.push(current_tile);
            if let Some(thing) = next {
                current = *thing;
            } else if c != dims - 1 {
                panic!("Next not found\n");
            }
        }
        if horiz_dir == 0 {
            row.reverse();
        }
        ordered_tiles.push(row);
        if let Some(thing) = next {
            current = *thing;
        } else if r != dims - 1 {
            panic!("Next not found\n");
        }
    }

    if vert_dir == 1 {
        ordered_tiles.reverse();
    }

    ordered_tiles
}

pub struct Board {
    grid: Vec<Vec<bool>>,
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.grid.iter() {
            for b in row.iter() {
                write!(f, "{}", if *b {'#'} else {'.'})?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

impl Board {
    pub fn from_oriented_tiles(tiles: &Vec<Vec<Tile>>) -> Self {
        let mut grid = vec![];
        for row in tiles.iter() {
            for i in 1..9 {
                let mut temp_row = vec![];
                for tile in row.iter() {
                    for j in 1..9 {
                        temp_row.push(tile.get_oriented(i, j));
                    }
                }
                grid.push(temp_row);
            }
        }

        Self { grid }
    }

    pub fn search_350(&self) -> HashSet<(usize, usize)> {
        lazy_static!(
            // Representation of
            //                   #
            // #    ##    ##    ###
            //  #  #  #  #  #  #
            static ref INDICES: [(usize, usize); 15] = [
                (0, 18),
                (1, 0), (1, 5), (1, 6), (1, 11), (1, 12), (1, 17), (1, 18), (1, 19),
                (2, 1), (2, 4), (2, 7), (2, 10), (2, 13), (2, 16),
            ];
        );

        let mut count = vec![vec![]; 8];
        for i in 0..self.grid.len() {
            for j in 0..self.grid.len() {
                for k in 0..8 {
                    if INDICES.iter().all(|(mut li, mut lj)| {
                        // Dimensions are 3x20
                        if k & 1 != 0 {
                            li = 2 - li;
                        }
                        if k & 2 != 0 {
                            lj = 19 - lj;
                        }
                        if k & 4 != 0 {
                            std::mem::swap(&mut li, &mut lj);
                        }

                        if li + i >= self.grid.len() {
                            return false;
                        }
                        if lj + j >= self.grid.len() {
                            return false;
                        }
                        self.grid[li + i][lj + j]
                    }) {
                        count[k].push((i, j));
                    }
                }
            }
        }

        let mut temp = HashSet::new();
        for (k, cs) in count.iter().enumerate() {
            for (i, j) in cs {
                for (mut li, mut lj) in INDICES.iter() {
                    if k & 1 != 0 {
                        li = 2 - li;
                    }
                    if k & 2 != 0 {
                        lj = 19 - lj;
                    }
                    if k & 4 != 0 {
                        std::mem::swap(&mut li, &mut lj);
                    }
                    temp.insert((li + i, lj + j));
                }
            }
        }

        temp
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn drive(filename: &str) {
        let input = std::fs::read_to_string(filename).unwrap();
        let mut last_lines: & [&str] = &mut input.lines().collect::<Vec<&str>>();

        let mut tiles = vec![];
        while !last_lines.is_empty() {
            let (tile, lines) = Tile::from_input(&mut last_lines);
            if let Some(tile) = tile {
//                println!("{}", tile);
                tiles.push(tile);
            }
            last_lines = lines;
        }

        let mingle = doit(&tiles);
        let mut acc = 1usize;
        for (i, assocs) in mingle.iter() {
            if assocs.len() <= 2 {
//                println!("Corner tile {} ", tiles[*i]);
                acc *= tiles[*i].id;
            }
        }

        let tiles = orient_tiles(&mut tiles, &mingle);
//        for row in tiles.iter() {
//            for tile in row.iter() {
//                println!("{}", tile);
//            }
//        }

        let board = Board::from_oriented_tiles(&tiles);
//        println!("{}", board);
        let used_indices = board.search_350();
//        for i in 0..board.grid.len() {
//            for j in 0..board.grid.len() {
//                if used_indices.contains(&(i, j)) {
//                    print!("O");
//                } else if board.grid[i][j] {
//                    print!("#");
//                } else {
//                    print!(" ");
//                }
//            }
//            println!("");
//        }
        println!("Part 1: {}", acc);
        let acc = board.grid.iter().fold(0, |acc, row| acc + row.iter().fold(0, |ac, x| ac + if *x {1} else {0}));
        println!("Part 2: {}", acc - used_indices.len());
    }

    #[test]
    fn can_i_doit() {
        // Use however many vars to print an array flipped around.
        let input = [
            [1, 2, 3, 4,],
            [5, 6, 7, 8,],
            [9, 10, 11, 12,],
            [13, 14, 15, 16,],
        ];

        for control in 0..8 {
            println!("{:03b}", control);
            for x in 0..input.len() {
                for y in 0..input[0].len() {
                    let mut i = x;
                    let mut j = y;
                    if control & 0x01 != 0 {
                        i = input.len() - i - 1;
                    }
                    if control & 0x02 != 0 {
                        j = input[0].len() - j - 1;
                    }
                    if control & 0x04 != 0 {
                        // Flip i and j directions
                        std::mem::swap(&mut i, &mut j);
                    }
                    print!("{:02} ", input[i][j]);
                }
                println!();
            }
            println!();
        }
    }

    #[test]
    fn it_works() {
        drive("res/20/input_simple.txt");
    }

    #[test]
    fn test_it() {
        drive("res/20/input.txt");
    }
}