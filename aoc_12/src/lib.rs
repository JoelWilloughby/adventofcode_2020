use regex::Regex;
use lazy_static::lazy_static;

pub trait Ship {
    fn new() -> Self;
    fn do_command(&mut self, cmd: &str);
    fn l1(&self) -> usize;
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x : isize,
    y : isize,
}

#[derive(Debug, Clone, Copy)]
pub struct Ship1 {
    pos: Point,
    vel: Point,
}

#[derive(Debug, Clone, Copy)]
pub struct Ship2 {
    pos: Point,
    way: Point,
}

impl std::ops::Add for Point {
    type Output = Self;
    fn add(self, _rhs: Self) -> Self {
        Self {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y, 
        }
    }
}

impl std::ops::AddAssign for Point {
    fn add_assign(&mut self, _rhs: Self) {
        *self = *self + _rhs;
    }
}

impl std::ops::Mul<isize> for Point {
    type Output = Self;

    fn mul(self, _rhs: isize) -> Self {
        Self {
            x: _rhs * self.x,
            y: _rhs * self.y,
        }
    }
}

impl std::ops::Mul<Point> for isize {
    type Output = Point;
    fn mul(self, _rhs: Point) -> Point {
        _rhs * self
    }
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self {
            x, y
        }
    }

    fn l1(&self) -> usize {
        self.x.abs() as usize + self.y.abs() as usize
    }
}

impl Ship for Ship1 {
    fn new() -> Self {
        Self {
            pos: Point::new(0,0),
            vel: Point::new(1,0),
        }
    }

    fn do_command(&mut self, cmd: &str) {
        lazy_static!{
            static ref CMD_RE: Regex = Regex::new(r"([A-Z])(\d+)").unwrap();
        }

        let caps = CMD_RE.captures(cmd).unwrap();
        let cmd_type = &caps[1];
        let mut cmd_arg = caps[2].parse::<isize>().unwrap();

        match cmd_type {
            "N" => { self.pos.y += cmd_arg; }
            "S" => { self.pos.y -= cmd_arg; }
            "E" => { self.pos.x += cmd_arg; }
            "W" => { self.pos.x -= cmd_arg; }
            "F" => { self.pos += cmd_arg * self.vel; }
            "L" | "R" => {
                if cmd_type == "R" {
                    cmd_arg = 360 - cmd_arg;
                } 
                match cmd_arg {
                    90 => {
                        let temp = self.vel.x;
                        self.vel.x = -1 * self.vel.y;
                        self.vel.y = temp;
                    },
                    180 => {
                        self.vel = -1 * self.vel;
                    },
                    270 => {
                        let temp = self.vel.x;
                        self.vel.x = self.vel.y;
                        self.vel.y = -1 * temp;
                    },
                    _ => {panic!("Oh noes!"); }
                }
            }
            _ => (),
        }
    }

    fn l1(&self) -> usize {
        self.pos.l1()
    }
}

impl Ship for Ship2 {
    fn new() -> Self {
        Self {
            pos: Point::new(0,0),
            way: Point::new(10,1),
        }
    }

    fn do_command(&mut self, cmd: &str) {
        lazy_static!{
            static ref CMD_RE: Regex = Regex::new(r"([A-Z])(\d+)").unwrap();
        }

        let caps = CMD_RE.captures(cmd).unwrap();
        let cmd_type = &caps[1];
        let mut cmd_arg = caps[2].parse::<isize>().unwrap();

        match cmd_type {
            "N" => { self.way.y += cmd_arg; }
            "S" => { self.way.y -= cmd_arg; }
            "E" => { self.way.x += cmd_arg; }
            "W" => { self.way.x -= cmd_arg; }
            "F" => { self.pos += cmd_arg * self.way; }
            "L" | "R" => {
                if cmd_type == "R" {
                    cmd_arg = 360 - cmd_arg;
                } 
                while cmd_arg > 0 {
                    let temp = self.way.x;
                    self.way.x = -1 * self.way.y;
                    self.way.y = temp;
                    cmd_arg -= 90; 
                }
            }
            _ => (),
        }
    }

    fn l1(&self) -> usize {
        self.pos.l1()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn drive<S: Ship + std::fmt::Debug>(filename: &str) {
        let input = std::fs::read_to_string(filename).unwrap();

        let mut ship = S::new();
        for line in input.lines() {
            // println!("{:?}", ship);
            ship.do_command(line);
        }
        // println!("{:?}", ship);

        println!("{} - {:?}", ship.l1(), ship);

    }

    #[test]
    fn point_stuff() {
        let p1 = Point::new(1,2);
        let p2 = Point::new(3,4);

        assert_eq!(p1 + p2, Point::new(4,6));
        assert_eq!(-3 * p1, Point::new(-3,-6));
        assert_eq!(p1 * 3, Point::new(3,6));

        let mut p3 = Point::new(1,1);
        p3 += -1 * Point::new(1,1);
        assert_eq!(p3, Point::new(0,0));
    }

    #[test]
    fn it_works() {
        drive::<Ship1>("res/input_simple.txt");
        drive::<Ship2>("res/input_simple.txt");
    }

    #[test]
    fn test_it() {
        drive::<Ship1>("res/input.txt");
        drive::<Ship2>("res/input.txt");
    }
}
