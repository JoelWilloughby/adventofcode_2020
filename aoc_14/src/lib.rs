use regex::Regex;
use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub enum Command {
    Mask ( usize, usize, usize ),
    Mem ( usize, usize,)
}

pub trait Computer {
    fn new() -> Self;
    fn exec(&mut self, cmd: &Command);
    fn sum(&self) -> usize;
}

#[derive(Debug)]
pub struct Computer1 {
    mem: HashMap<usize, usize>,
    set_bits: usize,
    clear_bits: usize,
}

#[derive(Debug)]
pub struct Computer2 {
    // addr, x-mask, val
    mem: HashMap<usize, usize>,
    set_bits: usize,
    clear_bits: usize,
    x_bits: usize,
}

struct Twister {
    current: usize,
    mask_bits: Vec<usize>,
}

impl Twister {
    fn new(mut mask: usize) -> Self {
        let mut mask_bits = vec![];
        let mut i = 0;
        while mask > 0 {
            if mask & 1 != 0 {
                mask_bits.push(i);
            }
            i += 1;
            mask >>= 1;
        }

        Self {
            current: 0,
            mask_bits,
        }
    } 
}

impl Iterator for Twister {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        if self.current & (1 << self.mask_bits.len()) != 0 {
            return None;    
        }

        let mut acc = 0;
        let mut temp = self.current;
        for i in 0..self.mask_bits.len() {
            if temp & 1 != 0 {
                acc |= 1 << self.mask_bits[i];
            }
            temp >>= 1;
        }

        self.current += 1;

        Some(acc)
    }
}

impl Command {
    fn parse_mask(mask: &str) -> Option<Self> {
        let mut set_bits = 0;
        let mut clear_bits = 0;
        let mut x_bits = 0;

        for b in mask.bytes() {
            set_bits <<= 1;
            clear_bits <<= 1;
            x_bits <<= 1;
            match b as char {
                '0' => {
                    clear_bits += 1;
                },
                '1' => {
                    set_bits += 1;
                },
                'X' => {
                    x_bits += 1;
                },
                _ => { return None; }
            }
        }

        Some(Self::Mask(set_bits, clear_bits, x_bits))
    }

    pub fn from_line(line: &str) -> Option<Self> {
        lazy_static! {
            static ref MASK_RE: Regex = Regex::new(r"^mask = ([X01]{36})$").unwrap();
            static ref MEM_RE: Regex = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();
        }

        if let Some(caps) = MASK_RE.captures(line) {
            Self::parse_mask(&caps[1])
        } else if let Some(caps) = MEM_RE.captures(line) {
            Some(
                Self::Mem(
                    caps[1].parse::<usize>().unwrap(),
                    caps[2].parse::<usize>().unwrap(),
                )
            )    
        } else {
            None
        }
    }
}

impl Computer for Computer1 {
    fn new() -> Self {
        Self {
            mem: HashMap::new(),
            set_bits: 0,
            clear_bits: 0,
        }
    }

    fn exec(&mut self, cmd: &Command) {
        match cmd {
            Command::Mask(set_bits, clear_bits, _) => {
                self.set_bits = *set_bits;
                self.clear_bits = *clear_bits;
            },
            Command::Mem(addr, val) => {
                let val = val | self.set_bits;
                let val = val & (!self.clear_bits);
                self.mem.insert(*addr, val);
            }
        }
    }

    fn sum(&self) -> usize {
        self.mem.iter().fold(0, |acc, (_, val)| acc + val)
    }
}

impl Computer for Computer2 {
    fn new() -> Self {
        Self {
            mem: HashMap::new(),
            set_bits: 0,
            clear_bits: 0,
            x_bits: 0,
        }
    }

    fn exec(&mut self, cmd: &Command) {
        match cmd {
            Command::Mask(set_bits, clear_bits, x_bits) => {
                self.set_bits = *set_bits;
                self.clear_bits = *clear_bits;
                self.x_bits = *x_bits;
            },
            Command::Mem(addr, val) => {
                // println!("{:08b} {:08b}", *addr, self.set_bits);
                let mut addr = *addr | self.set_bits;
                addr &= !self.x_bits;

                for t in Twister::new(self.x_bits) {
                    // println!("Tiwsting {:08b} {:08b} {:08b} {:08b}", self.x_bits, t, addr, addr | t);
                    self.mem.insert(addr | t, *val);
                }
            }
        }
    }

    fn sum(&self) -> usize {
        self.mem.iter().fold(0, |acc, (_, val)| acc + val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn drive<C: Computer + std::fmt::Debug>(filename: &str) {
        let input = std::fs::read_to_string(filename).unwrap();

        let mut puter = C::new();
        for line in input.lines() {
            puter.exec(&Command::from_line(line).unwrap());
        }

        println!("{}", puter.sum());
    }

    #[test]
    fn twister_works() {
        assert_eq!(
            vec![ 0b0000, 0b0001, 0b0100, 0b0101 ],
            Twister::new(0b0101).collect::<Vec<usize>>(),
        );
        assert_eq!(
            vec![ 0b000000, 0b000001, 0b000100, 0b000101,
                  0b100000, 0b100001, 0b100100, 0b100101,  ],
            Twister::new(0b100101).collect::<Vec<usize>>(),
        );
    }

    #[test]
    fn it_works() {
        drive::<Computer1>("res/input_simple.txt");
        drive::<Computer2>("res/input_trival.txt");
    }

    #[test]
    fn test_it() {
        drive::<Computer1>("res/input.txt");
        drive::<Computer2>("res/input.txt");
    }
}
