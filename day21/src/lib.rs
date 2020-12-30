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

pub fn part1(input: &Path) -> Result<(), Error> {
    let foods: Vec<Food> = parse::<Food>(input)?.collect();
    println!(
        "The answer to part one is {}",
        non_allergen_appearances(foods)
    );
    Ok(())
}

pub fn part2(_input: &Path) -> Result<(), Error> {
    unimplemented!()
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
