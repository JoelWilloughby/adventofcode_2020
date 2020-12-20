use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub enum Rule {
    Char(char),
    Concat(Vec<usize>),
    Or(Vec<Rule>),
}

#[derive(Debug, Clone)]
pub struct Automaton {
    rules: HashMap<usize, Rule>,
}

impl Rule {
    fn parse_concat(rem: &str) -> Option<Self> {
        let nums = rem
            .split(" ")
            .map(|val| val.parse::<usize>().unwrap())
            .collect();
        Some(Self::Concat(nums))
    }

    pub fn from_line(line: &str) -> Option<(usize, Self)> {
        lazy_static! {
            static ref RULE_RE: Regex = Regex::new(r"^(\d+): (.+)$").unwrap();
            static ref OR_RE: Regex = Regex::new(r"^([\d ]+) \| ([\d ]+)$").unwrap();
            static ref CONCAT_RE: Regex = Regex::new(r"^([\d ]+)$").unwrap();
            static ref CHAR_RE: Regex = Regex::new(r#"^"([a-z])"$"#).unwrap();
        };

        if let Some(caps) = RULE_RE.captures(line) {
            let idx = caps[1].parse::<usize>().ok()?;
            if let Some(or_caps) = OR_RE.captures(&caps[2]) {
                Some ((
                    idx,
                    Self::Or(vec![
                        Self::parse_concat(&or_caps[1])?,
                        Self::parse_concat(&or_caps[2])?,
                    ]),
                ))
            } else if let Some(concat_caps) = CONCAT_RE.captures(&caps[2]) {
                Some ((
                    idx,
                    Self::parse_concat(&concat_caps[1])?,
                ))
            } else if let Some(char_caps) = CHAR_RE.captures(&caps[2]) {
                Some ((
                    idx,
                    Self::Char(char_caps[1].parse::<char>().ok()?)
                ))
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl Automaton {
    pub fn from_rules(rules: &Vec<(usize, Rule)>) -> Self {
        let mut hash_rules = HashMap::new();
        rules.iter().map(|(a, b)| hash_rules.insert(*a, b.clone())).count();
        Self { rules: hash_rules }
    }

    fn _eval<'a>(&self, input: &'a[u8], starting_rule: &Rule) -> (bool, Vec<&'a[u8]>) {
        if input.is_empty() {
            return (false, vec![]);
        }
        match starting_rule {
            Rule::Char(c) => {
                if *c == input[0] as char{
                    (true, vec![&input[1..]])
                } else {
                    (false, vec![])
                }
            },
            Rule::Concat(rs) => {
                let mut current_rems = vec![input];
                for r in rs.iter() {
                    let mut next_rems: HashSet<&[u8]> = HashSet::new();
                    for current_rem in current_rems.iter() {
                        let (_passed, mut _rem_vec) = self._eval(current_rem, &self.rules.get(r).unwrap());
                        next_rems.extend(_rem_vec);
                    }

                    if next_rems.is_empty() {
                        return (false, vec![])
                    }
                    current_rems = next_rems.drain().collect();
                }

                (true, current_rems)
            },
            Rule::Or(rs) => {
                let mut rems = vec![];
                for r in rs.iter() {
                    let (_passed, mut rem) = self._eval(input, r);
                    rems.append(&mut rem);
                }
                (!rems.is_empty(), rems)
            }
        }
    }

    pub fn eval(&self, input: &str) -> bool {
        let (passed, rems) = self._eval(&input.as_bytes(), self.rules.get(&0).unwrap());
        if !passed {
            return false;
        }

        return rems.iter().any(|l| l.is_empty());
    }

    pub fn part_2_hack(&mut self) {
        self.rules.insert(8, Rule::Or(vec![
            Rule::Concat(vec![42]),
            Rule::Concat(vec![42, 8]),
        ]));
        self.rules.insert(11, Rule::Or(vec![
            Rule::Concat(vec![42, 31]),
            Rule::Concat(vec![42, 11, 31]),
        ]));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn drive(filename: &str) {
        let input = std::fs::read_to_string(filename).unwrap();
        let mut lines = input.lines();
        let mut rules = vec![];
        while let Some(rule) = Rule::from_line(lines.next().unwrap()) {
            rules.push(rule);
        }

        let automaton = Automaton::from_rules(&rules);
        let mut automaton_2 = Automaton::from_rules(&rules);
        automaton_2.part_2_hack();
        let mut count = 0;
        let mut count_2 = 0;
        for line in lines {
            if automaton.eval(&line) {
                count += 1;
            }
            if automaton_2.eval(&line) {
                count_2 += 1;
            }
        }

        println!("Part 1: {}", count);
        println!("Part 2: {}", count_2);
    }

    #[test]
    fn test_it() {
        drive("res/19/input.txt");
    }

    #[test]
    fn it_works() {
        drive("res/19/input_simple.txt");
    }

    #[test]
    fn it_works_2() {
        drive("res/19/input_part_2.txt");
    }
}