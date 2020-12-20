#[derive(Debug)]
pub struct CubeGridSimple {
    grid: Vec<Vec<Vec<Vec<bool>>>>,
}

impl CubeGridSimple {
    pub fn from_str(input: &str) -> Self {
        let line_length = input.lines().next().unwrap().len();
        let mut temp = vec![vec![vec![vec![false; line_length + 4]; line_length + 4]; 5]; 5];

        for (i, line) in input.lines().enumerate() {
            for (j, b) in line.bytes().enumerate() {
                if b == '#' as u8 {
                    temp[2][2][i+2][j+2] = true;
                }
            }
        }

        Self {
            grid: temp,
        }
    }

    fn count_around(&self, x: usize, y: usize, z: usize, w: usize) -> usize {
        let mut sum = 0;
        for l in (w-1)..=(w+1) {
            for k in (z-1)..=(z+1) {
                for i in (x-1)..=(x+1) {
                    for j in (y-1)..=(y+1) {
                        if i == x && j == y && k == z && l == w {
                            continue;
                        }

                        if self.grid[l][k][i][j] {
                            sum += 1;
                        }

                        if sum > 3 {
                            return sum;
                        }
                    }
                }
            }
        }
        sum
    }

    fn count_around_1(&self, x: usize, y: usize, z: usize) -> usize {
        let mut sum = 0;
        for k in (z-1)..=(z+1) {
            for i in (x-1)..=(x+1) {
                for j in (y-1)..=(y+1) {
                    if i == x && j == y && k == z {
                        continue;
                    }

                    if self.grid[2][k][i][j] {
                        sum += 1;
                    }

                    if sum > 3 {
                        return sum;
                    }
                }
            }
        }
        sum
    }

    pub fn step(&mut self) {
        let mut temp = vec![vec![vec![vec! [false; self.grid[0][0][0].len() + 2];
                                 self.grid[0][0].len() + 2];
                            self.grid[0].len() + 2];
                        self.grid.len() + 2];
        for w in 1..self.grid.len()-1 {
            for z in 1..self.grid[w].len()-1 {
                for x in 1..self.grid[w][z].len()-1 {
                    for y in 1..self.grid[w][z][x].len()-1 {
                        let sum = self.count_around(x, y, z, w);
                        if (self.grid[w][z][x][y] && (sum == 2 || sum == 3)) ||
                            (!self.grid[w][z][x][y] && sum == 3) {
                            temp[w+1][z+1][x+1][y+1] = true;
                        }
                    }
                }
            }
        }

        self.grid = temp;
    }

    pub fn step_1(&mut self) {
        let mut temp = vec![vec![vec! [false; self.grid[2][0][0].len() + 2];
                                      self.grid[2][0].len() + 2];
                                 self.grid[2].len() + 2];
        for z in 1..self.grid[2].len()-1 {
            for x in 1..self.grid[2][z].len()-1 {
                for y in 1..self.grid[2][z][x].len()-1 {
                    let sum = self.count_around_1(x, y, z);
                    if (self.grid[2][z][x][y] && (sum == 2 || sum == 3)) ||
                        (!self.grid[2][z][x][y] && sum == 3) {
                        temp[z+1][x+1][y+1] = true;
                    }
                }
            }
        }

        self.grid[2] = temp;

    }

    pub fn count(&self) -> usize {
        let mut sum = 0;
        for w in 0..self.grid.len() {
            for z in 0..self.grid[w].len() {
                for x in 0..self.grid[w][z].len() {
                    for y in 0..self.grid[w][z][x].len() {
                        if self.grid[w][z][x][y] {
                            sum += 1;
                        }
                    }
                }
            }
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn drive(filename: &str) {
        let input = std::fs::read_to_string(filename).unwrap();
        let mut cubes = CubeGridSimple::from_str(input.as_str());
        for _ in 0..6 {
            cubes.step_1();
        }
        println!("Num cubes part 1: {}", cubes.count());

        let mut second_cubes = CubeGridSimple::from_str(input.as_str());
        for _ in 0..6 {
            second_cubes.step();
        }
        println!("Num cubes part 2: {}", second_cubes.count());
    }

    #[test]
    fn it_works() {
        drive("res/17/input_simple.txt");
    }

    #[test]
    fn test_it() {
        drive("res/17/input.txt");
    }
}