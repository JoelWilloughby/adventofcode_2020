use std::collections::{HashMap, HashSet};
use regex::Regex;
use lazy_static::lazy_static;

pub fn doit(input: String) {
    lazy_static!(
        static ref LINE_RE: Regex = Regex::new(r"^([a-z ]+) \(contains ([a-z, ]+)\)$").unwrap();
    );

    let mut allergen_recipe_map: HashMap<String, Vec<usize>> = HashMap::new();
    let mut ingredient_recipe_map: HashMap<String, Vec<usize>> = HashMap::new();
    let mut recipe_list: Vec<HashSet<String>> = vec![];
    let mut all_ingredients = HashSet::new();
    for (i, line) in input.lines().enumerate() {
        if let Some(caps) = LINE_RE.captures(line) {
            let ingredients = caps[1].split(" ").map(|s| s.to_string()).collect::<HashSet<String>>();
            all_ingredients = all_ingredients.union(&ingredients).cloned().collect();
            let allergens = caps[2].split(",").map(|s| s.trim().to_string()).collect::<HashSet<String>>();
            recipe_list.push(ingredients.clone());
            for allergen in allergens.iter() {
                allergen_recipe_map.entry(allergen.clone()).or_insert(vec![]).push(i);
            }
            for ingredient in ingredients.iter() {
                ingredient_recipe_map.entry(ingredient.clone()).or_insert(vec![]).push(i);
            }
        }
    }

    let mut can_be = HashSet::new();
    let mut allergen_to_ingredient_possibilities: HashMap<String, HashSet<String>> = HashMap::new();
    for (allergen, recipes) in allergen_recipe_map {
        let mut intersects = all_ingredients.clone();
        for r in recipes.iter() {
            let recipe = &recipe_list[*r];
            intersects = intersects.intersection(recipe).cloned().collect();
        }

        can_be = can_be.union(&intersects).cloned().collect();
        allergen_to_ingredient_possibilities.insert(allergen, intersects);
    }

    let cant_be: HashSet<String> = all_ingredients.difference(&can_be).cloned().collect();
    println!("{}", cant_be.iter().fold(0, |acc, ingred| acc + ingredient_recipe_map[ingred].len()));

    let mut solution = vec![];
    while !allergen_to_ingredient_possibilities.is_empty() {
        let (allergen, candidates) =
            allergen_to_ingredient_possibilities
                .iter()
                .find(|(_, possibles)| {
                    possibles.len() == 1
                }).unwrap();
        let allergen = allergen.clone();
        let candidate = candidates.iter().nth(0).unwrap().clone();
        solution.push((allergen.clone(), candidate.clone()));
        allergen_to_ingredient_possibilities.remove(&allergen);

        for (_, other) in allergen_to_ingredient_possibilities.iter_mut() {
            other.remove(&candidate);
        }
    }

    solution.sort();
    for (_, ingred) in solution {
        print!("{},", ingred);
    }
    println!("");
}

#[cfg(test)]
mod tests {
    use super::*;
    fn drive(filename: &str) {
        let input = std::fs::read_to_string(filename).unwrap();
        doit(input.clone());
    }

    #[test]
    fn it_works() {
        drive("res/21/input_simple.txt");
    }

    #[test]
    fn test_it() {
        drive("res/21/input.txt");
    }
}