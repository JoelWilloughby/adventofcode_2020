use std::collections::HashSet;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
enum Operation {
    Acc(isize),
    Nop(isize),
    Jmp(isize),
}

#[derive(Debug)]
pub enum RunState {
    Done,
    Infinite,
    Running,
}

#[derive(Debug)]
pub struct Handheld {
    pc: isize,
    acc: isize,

    prog: Vec<Operation>,
    history: HashSet<isize>,
}

impl Operation {
    fn from_line(line: &str) -> Option<Self> {
        lazy_static! {
            static ref LINE_RE: Regex = Regex::new(r"(?P<op>[a-z]{3}) (?P<arg>[+\-]\d+)").unwrap(); 
        }

        let caps = LINE_RE.captures(line).unwrap();
        let op = caps.name("op").unwrap().as_str();
        let arg = caps.name("arg").unwrap().as_str().parse::<isize>().unwrap();

        return match op {
            "acc" => Some(Self::Acc(arg)),
            "nop" => Some(Self::Nop(arg)),
            "jmp" => Some(Self::Jmp(arg)),
            _ => None,
        }
    }
}

impl Handheld {
    pub fn from_string(input: String) -> Self {
        let mut prog = vec![];
        for line in input.lines() {
            if let Some(op) = Operation::from_line(line) {
                prog.push(op);
            }
        }

        Self {
            pc: 0,
            acc: 0,
            prog,
            history: HashSet::new(),
        }
    }

    pub fn reset(&mut self) {
        self.pc = 0;
        self.acc = 0;
        self.history.clear();
    } 

    pub fn step(&mut self) -> RunState {
        if self.pc >= self.prog.len() as isize {
            return RunState::Done;
        }

        if self.history.contains(&self.pc) {
            return RunState::Infinite;
        }

        self.history.insert(self.pc);

        match self.prog[self.pc as usize] {
            Operation::Nop(_) => {},
            Operation::Acc(val) => {
                self.acc += val;
            },
            Operation::Jmp(val) => {
                self.pc += val - 1;
            }
        }
        self.pc += 1;

        RunState::Running
    }

    pub fn cont(&mut self) -> RunState{
        let mut state = RunState::Running;
        while let RunState::Running = state {
            state = self.step();
        }

        state
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn drive(filename: &str) {
        let input = std::fs::read_to_string(filename).unwrap();
        let mut puter = Handheld::from_string(input);

        let state = puter.cont();
        println!("Broken puter: {} {} {:?}", puter.pc, puter.acc, state);
        for i in 0..puter.prog.len() {
            puter.prog[i] = match puter.prog[i] {
                Operation::Acc(_) => {
                    continue;
                },
                Operation::Nop(val) => Operation::Jmp(val),
                Operation::Jmp(val) => Operation::Nop(val),
            };

            puter.reset();
            let state = puter.cont();

            puter.prog[i] = match puter.prog[i] {
                Operation::Acc(_) => {
                    continue;
                },
                Operation::Nop(val) => Operation::Jmp(val),
                Operation::Jmp(val) => Operation::Nop(val),
            };
            if let RunState::Done = state {
                println!("Fixed puter: {} {} {:?}", puter.pc, puter.acc, state);
                break;
            }
        }
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
