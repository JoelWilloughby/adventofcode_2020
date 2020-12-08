use std::collections::{HashMap, HashSet};
use regex::Regex;
use lazy_static::lazy_static;
use std::collections::hash_map::Entry::{Occupied, Vacant};

#[derive(Debug, Clone)]
struct Bag {
    description: String,

    sub_bags: Vec<(String, usize)>,
    super_bags: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Baggage {
    bags: HashMap<String, Bag>,
}

impl Bag {
    fn new(description: &str) -> Self {
        Self {
            description: String::from(description),
            sub_bags: Vec::new(),
            super_bags: Vec::new(),
        }
    }
}

impl Baggage {
    pub fn new() -> Self {
        Self {
            bags: HashMap::new(),
        }
    }

    pub fn read_line(&mut self, line: &str) {
        lazy_static! {
            static ref OUTER_BAG_RE: Regex = Regex::new(r"([a-z]+ [a-z]+) bags contain").unwrap();
            static ref INNER_BAG_RE: Regex = Regex::new(r" *([0-9a-z]+ [a-z]+ (?:[a-z]+)?) bags?[.,]").unwrap();
        }

        let mat = OUTER_BAG_RE.find(line).unwrap();
        let outer_captures = OUTER_BAG_RE.captures(&line[mat.start()..mat.end()]).unwrap();
        let current_bag_name = outer_captures.get(1).unwrap().as_str();

        let mut current_bag = match self.bags.entry(current_bag_name.to_string()) {
            Vacant(entry) => entry.insert(Bag::new(current_bag_name)),
            Occupied(entry) => entry.into_mut(),
        }.sub_bags.clone();

        for caps in INNER_BAG_RE.captures_iter(&line[mat.end()..]) {
            let desc = caps.get(1).unwrap().as_str();
            if desc == "no other" {
                continue;
            }

            lazy_static! {
                static ref BAG_DESC_RE: Regex = Regex::new(r"^(\d+) ([a-z]+ [a-z]+)$").unwrap();
            }

            let desc_caps = BAG_DESC_RE.captures(desc).unwrap();
            let desc_name = desc_caps.get(2).unwrap().as_str();
            let desc_count = desc_caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
            current_bag.push((desc_name.to_string(), desc_count));

            let bag = match self.bags.entry(desc_name.to_string()) {
                Vacant(entry) => entry.insert(Bag::new(desc_name)),
                Occupied(entry) => entry.into_mut(),
            };

            bag.super_bags.push(current_bag_name.to_string());
        }

        self.bags.get_mut(&current_bag_name.to_string()).unwrap().sub_bags = current_bag;
    }

    fn traverse_helper(&self, current: String, visited: &mut HashSet<String>) {
        if visited.contains(&current) {
            return;
        }

        visited.insert(current.clone());

        let bag = self.bags.get(&current).unwrap();

        for other in bag.super_bags.iter() {
            self.traverse_helper(other.to_string(), visited);
        }
    }

    pub fn traverse(&self, start: &str) -> HashSet<String> {
        let mut ret = HashSet::new();
        self.traverse_helper(start.to_string(), &mut ret);
        ret
    }

    fn traverse_2_helper(&self, current: String, memo: &mut HashMap<String, usize>) -> usize {
        if let Some(val) = memo.get(&current) {
            return *val;
        }

        let mut accum = 1;
        for (sub, count) in self.bags.get(&current).unwrap().sub_bags.iter() {
            accum += count * self.traverse_2_helper(sub.clone(), memo);
        }

        memo.insert(current, accum);

        accum
    }

    pub fn traverse_2(&self, start: &str) -> usize {
        self.traverse_2_helper(start.to_string(), &mut HashMap::new())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    fn drive(filename: &str) {
        let input = std::fs::read_to_string(filename).unwrap();

        let mut baggage = Baggage::new();

        for line in input.lines() {
            baggage.read_line(line);
        }

        let trav = baggage.traverse("shiny gold");
        // println!("{:?}", trav);
        println!("Total {} ", baggage.bags.len());
        println!("shiny gold search: {}", trav.len() - 1);
        println!("Harder search: {}", baggage.traverse_2("shiny gold") - 1);
    }

    #[test]
    fn it_works() {
        drive("res/input_simple.txt");
    }

    #[test]
    fn test_it() {
        drive("res/input_part_2.txt");
        drive("res/input.txt");
    }
}
