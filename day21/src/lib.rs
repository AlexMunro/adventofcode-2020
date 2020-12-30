use aoc2020::parse;

use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::str::FromStr;
use thiserror::Error;

#[derive(Clone, Debug)]
struct Food {
    ingredients: Vec<String>,
    allergens: Vec<String>,
}

impl FromStr for Food {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "" {
            return Err("Empty string, no need to parse")?;
        }

        let mut chunks = s.split(" (contains ");

        Ok(Food {
            ingredients: chunks
                .next()
                .unwrap()
                .split_whitespace()
                .map(|s| s.to_string())
                .collect(),
            allergens: chunks
                .next()
                .unwrap()
                .split(")")
                .next()
                .unwrap()
                .split(", ")
                .map(|s| s.to_string())
                .collect(),
        })
    }
}

fn build_allergen_list(foods: &Vec<Food>) -> HashMap<String, HashSet<String>> {
    let ingredient_set: HashSet<String> = foods
        .iter()
        .flat_map(|f| f.ingredients.iter().cloned())
        .collect();

    foods
        .iter()
        .flat_map(|f| f.allergens.iter().cloned())
        .unique()
        .map(|allergen| {
            let ingredients_per_food = foods
                .iter()
                .filter(|f| f.allergens.contains(&allergen))
                .map(|f| f.ingredients.iter().cloned().collect::<HashSet<String>>())
                .collect::<Vec<HashSet<String>>>();

            let mut allergenic_ingredients = ingredient_set.clone();
            allergenic_ingredients.retain(|i| ingredients_per_food.iter().all(|v| v.contains(i)));
            (allergen.clone(), allergenic_ingredients)
        })
        .collect()
}

fn non_allergen_appearances(foods: Vec<Food>) -> usize {
    let allergen_list = build_allergen_list(&foods);
    let known_allergens: HashSet<String> = allergen_list
        .into_iter()
        .flat_map(|(_, ingredients)| ingredients)
        .collect();

    let ingredients = foods
        .iter()
        .flat_map(|f| f.ingredients.clone())
        .unique()
        .filter(|i| !known_allergens.contains(i));

    ingredients
        .map(|i| foods.iter().filter(|f| f.ingredients.contains(&i)).count())
        .sum()
}

fn dangerous_ingredient_list(foods: Vec<Food>) -> String {
    let mut confirmed_allergies: HashMap<String, String> = HashMap::new();
    let mut potential_allergens = build_allergen_list(&foods);
    let mut identified_ingredients: HashSet<String> = HashSet::new();

    while potential_allergens.len() > 0 {
        let newly_discovered = potential_allergens
            .iter()
            .filter_map(|(k, v)| {
                let mut opts = (*v).clone();
                opts.retain(|i| !identified_ingredients.contains(i));
                if opts.len() == 1 {
                    Some((k.clone(), opts.iter().next().unwrap().clone()))
                } else {
                    None
                }
            })
            .collect::<HashMap<String, String>>();

        for (allergen, causing_ingredient) in newly_discovered {
            potential_allergens.remove(&allergen);
            confirmed_allergies.insert(allergen, causing_ingredient.clone());
            identified_ingredients.insert(causing_ingredient.clone());
        }
    }

    confirmed_allergies
        .keys()
        .sorted()
        .map(|k| confirmed_allergies[k].clone())
        .join(",")
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let foods: Vec<Food> = parse::<Food>(input)?.collect();
    println!(
        "The answer to part one is {}",
        non_allergen_appearances(foods)
    );
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let foods: Vec<Food> = parse::<Food>(input)?.collect();
    println!(
        "The answer to part one is {}",
        dangerous_ingredient_list(foods)
    );
    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[cfg(test)]
#[test]
fn test_non_allergen_appearances() {
    let example: Vec<Food> = [
        "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)",
        "trh fvjkl sbzzf mxmxvkd (contains dairy)",
        "sqjhc fvjkl (contains soy)",
        "sqjhc mxmxvkd sbzzf (contains fish)",
    ]
    .iter()
    .map(|s| Food::from_str(s).unwrap())
    .collect();

    assert_eq!(non_allergen_appearances(example), 5);
}

#[test]
fn test_dangerous_ingredient_list() {
    let example: Vec<Food> = [
        "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)",
        "trh fvjkl sbzzf mxmxvkd (contains dairy)",
        "sqjhc fvjkl (contains soy)",
        "sqjhc mxmxvkd sbzzf (contains fish)",
    ]
    .iter()
    .map(|s| Food::from_str(s).unwrap())
    .collect();

    assert_eq!(
        dangerous_ingredient_list(example),
        "mxmxvkd,sqjhc,fvjkl".to_string()
    );
}
