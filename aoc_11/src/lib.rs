use std::fmt;

#[derive(Debug, Clone, PartialEq)]
enum Spot {
    Empty,
    Occupied,
    Blank,
}

#[derive(Debug)]
pub struct Board {
    cells: Vec<Vec<Spot>>,
    can_see: Vec<Vec<Vec<(usize, usize)>>>,
}

// Display for the Spot
impl fmt::Display for Spot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "L"),
            Self::Blank => write!(f, "."),
            Self::Occupied => write!(f, "#"),
        }
    }

}

// Display for the board
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.cells.iter() {
            for cell in row.iter() {
                write!(f, "{}", cell)?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

// Creates a single line of Spots
fn read_line(line: &str) -> Vec<Spot> {
    line.bytes().map(|b| 
        match b as char {
            'L' => Spot::Empty,
            _ => Spot::Blank, 
            })
    .collect()
}

// Helper function to pad a single line with .s
fn pad_inner(mut v: Vec<Spot>) -> Vec<Spot> {
    let mut ret = vec![Spot::Blank];
    ret.append(&mut v);
    ret.push(Spot::Blank);

    ret
}

// Pads a given baord with .s around the perimeter
fn pad(v: Vec<Vec<Spot>>) -> Vec<Vec<Spot>> {
    let pad_row = vec![Spot::Blank; v[0].len() + 2];
    let mut ret = vec![pad_row.clone()];
    for row in v {
        ret.push(pad_inner(row));
    }
    ret.push(pad_row);

    ret
}

// Counts the number of occupied seats around a given location
fn count_around(v: &Vec<Vec<Spot>>, can_see: &Vec<(usize, usize)>) -> usize {
    can_see.iter().fold(0, |acc, &(r, c)| {
        acc + match v[r][c] {
            Spot::Occupied => 1,
            _ => 0,
        }
    })
}

impl Board {
    // Creates a board from the input graph
    pub fn from_input(input: String) -> Option<Self> {
        let mut cells: Vec<Vec<Spot>> = input
                .lines()
                .map(|line| read_line(line))
                .collect();

        // Pad out. This helps avoid indexing outside the array
        cells = pad(cells);

        // Sanity Check input
        let sum = cells[0].len();
        for row in cells.iter() {
            if row.len() != sum {
                return None;
            }            
        }

        // Initialize can_see. By default, this is just all the space
        // directly adjacent (including diagonal) to a node
        let mut can_see = vec![];
        for r in 0..cells.len() {
            let mut temp = vec![];
            for c in 0..cells[0].len() {
                match cells[r][c] {
                    Spot::Empty => {
                        temp.push(vec![
                            (r-1, c-1), (r-1, c), (r-1, c+1),
                            (r, c-1), (r, c+1),
                            (r+1, c-1), (r+1, c), (r+1, c+1),
                        ])
                    },
                    _ => {
                        temp.push(vec![]);
                    },
                }
            }
            can_see.push(temp);
        }

        Some(Self { cells, can_see })
    }

    // Check to see if the board changed. Use by step
    fn was_changed(&self, last_time: Vec<Vec<Spot>>) -> bool {
        for r in 0..self.cells.len() {
            for c in 0..self.cells[r].len() {
                if self.cells[r][c] != last_time[r][c] {
                    return true;
                }
            }
        }

        false
    }

    // Steps the board according to the given instructions. Makes
    // use of each nodes can see vector as well as the supplied
    // num. If at least num seats that the seat can see are occupied,
    // it becomes unoccupied. If no seats it can see are occupied, it
    // becomes occupied.
    pub fn step(&mut self, num: usize) -> bool {
        let last_time = self.cells.clone();

        for r in 1..(self.cells.len()-1) {
            for c in 1..(self.cells[r].len()-1) {
                let surround_count = count_around(&last_time, &self.can_see[r][c]);
                match last_time[r][c] {
                    Spot::Occupied => {
                        if surround_count >= num {
                            self.cells[r][c] = Spot::Empty;
                        }
                    },
                    Spot::Empty => {
                        if surround_count == 0 {
                            self.cells[r][c] = Spot::Occupied;
                        }
                    }
                    _ => (),
                }
            }
        }

        // Return whether or not something changed
        self.was_changed(last_time)
    }

    // Counts the number of occupied seats on the board
    pub fn count(&self) -> usize {
        let mut accum = 0;
        for row in self.cells.iter() {
            accum += row.iter()
                .fold(0, |acc, c| {
                    acc + match c {
                        Spot::Occupied => 1,
                        _ => 0,
                    }
                })
        }

        accum
    }

    // Resets all the seats to unoccupied
    pub fn reset(&mut self) {
        for r in 0..self.cells.len() {
            for c in 0..self.cells[r].len() {
                self.cells[r][c] = match self.cells[r][c] {
                    Spot::Occupied => Spot::Empty,
                    Spot::Empty => Spot::Empty,
                    Spot::Blank => Spot::Blank,
                }
            }
        }
    }

    // Sets the as seen vector for a given seat. Traverses as 
    // far as it can in a single direction to do so until it 
    // finds a non-blank space.
    fn set_as_seen(&mut self, r: usize, c: usize) {
        let all_dirs: Vec<(isize, isize)> = 
            vec![(-1,-1), (-1,0), (-1,1),
                  (0,-1), (0,1),
                  (1,-1), (1,0), (1,1)];
        
        let mut as_seen = vec![];
        for (dir_x, dir_y) in all_dirs.iter() {
            let mut curr_r = (r as isize + dir_x) as usize;
            let mut curr_c = (c as isize + dir_y) as usize;
            while curr_r < self.cells.len() && curr_c < self.cells[0].len() {
                match self.cells[curr_r][curr_c] {
                    Spot::Empty => {
                        as_seen.push((curr_r, curr_c));
                        break;
                    },
                    _ => (),
                }

                curr_r = (curr_r as isize + dir_x) as usize;
                curr_c = (curr_c as isize + dir_y) as usize;
            }
        }

        self.can_see[r][c] = as_seen;
    }

    // Preps part 2. Sets all the can_sees to extend out.
    pub fn prep_part_2(&mut self) {
        self.reset();
        for r in 1..(self.cells.len()-1) {
            for c in 1..(self.cells[r].len()-1) {
                self.set_as_seen(r, c);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn drive(filename: &str) {
        let input = std::fs::read_to_string(filename).unwrap();
        let mut board = Board::from_input(input).unwrap();

        while board.step(4) {
        }
        println!("{}", board.count());

        board.prep_part_2();
        while board.step(5) {
            // println!("{}", board);
            // std::thread::sleep(std::time::Duration::from_millis(100));
        }
        println!("{}", board.count());

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
