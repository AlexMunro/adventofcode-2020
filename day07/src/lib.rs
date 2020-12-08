use aoc2020::parse;

use lazy_static::lazy_static;
use regex::Regex;
use std::cell::RefCell;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::path::Path;
use std::rc::Rc;
use std::str::FromStr;
use thiserror::Error;

lazy_static! {
    static ref OUTER_BAG: Regex = Regex::new(r"^([a-z]+ [a-z]+) bags contain (.*)$").unwrap();
    static ref INNER_BAGS: Regex = Regex::new(r"(\d)+ ([a-z]+ [a-z]+)").unwrap();
}

fn parse_bag(s: &str) -> (usize, String) {
    let captures = INNER_BAGS.captures(s).unwrap();
    (
        captures.get(1).unwrap().as_str().parse::<usize>().unwrap(),
        captures.get(2).unwrap().as_str().to_string(),
    )
}

struct ParsedEntry {
    outer: String,
    inner: Vec<(usize, String)>,
}

impl FromStr for ParsedEntry {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "" {
            return Err("No entries to parse".to_string());
        }

        let captures = OUTER_BAG.captures(s).expect("Could not parse line");
        let key = captures
            .get(1)
            .expect("Could not identify outer bag")
            .as_str()
            .to_string();
        let values = captures
            .get(2)
            .expect("Could not find any inner bags")
            .as_str()
            .split(",")
            .filter(|s| *s != "no other bags.")
            .map(|s| parse_bag(s))
            .collect();

        Ok(Self {
            outer: key,
            inner: values,
        })
    }
}

type BagMap = HashMap<String, Vec<(usize, String)>>;

impl FromIterator<ParsedEntry> for BagMap {
    fn from_iter<I: IntoIterator<Item = ParsedEntry>>(iter: I) -> Self {
        let mut bag_map = HashMap::new();

        for i in iter {
            bag_map.insert(i.outer, i.inner);
        }

        bag_map
    }
}

fn total_containing_shiny_gold(bag_map: BagMap) -> usize {
    fn contains_golden_rec(
        bag_map: &BagMap,
        contains_golden: Rc<RefCell<HashMap<String, bool>>>,
        outer_colour: String,
    ) -> bool {
        bag_map
            .get(&outer_colour)
            .unwrap()
            .iter()
            .map(|(_, colour)| {
                (
                    colour,
                    Rc::clone(&contains_golden).borrow().get(colour).cloned(),
                )
            })
            .any(|(colour, recorded)| {
                if colour == "shiny gold" {
                    true
                } else {
                    match recorded {
                        Some(recorded_boolean) => recorded_boolean,
                        None => {
                            let child_has_golden = contains_golden_rec(
                                bag_map,
                                Rc::clone(&contains_golden),
                                colour.to_string(),
                            );
                            Rc::clone(&contains_golden)
                                .borrow_mut()
                                .insert(colour.to_string(), child_has_golden);
                            child_has_golden
                        }
                    }
                }
            })
    }

    let contains_golden: Rc<RefCell<HashMap<String, bool>>> = Rc::new(RefCell::new(HashMap::new()));
    Rc::clone(&contains_golden)
        .borrow_mut()
        .insert("shiny gold".to_string(), false);

    for (outer_colour, _) in &bag_map {
        let recorded_value = Rc::clone(&contains_golden)
            .borrow()
            .get::<str>(&outer_colour)
            .cloned();

        if let Some(_) = recorded_value {
            continue;
        } else {
            let contains = contains_golden_rec(
                &bag_map,
                Rc::clone(&contains_golden),
                outer_colour.to_string(),
            );
            contains_golden
                .borrow_mut()
                .insert(outer_colour.to_string(), contains);
        }
    }

    bag_map
        .keys()
        .filter(|k| *contains_golden.borrow().get(*k).unwrap_or(&false))
        .count()
}

fn total_contained_by_shiny_gold(bag_map: BagMap) -> usize {
    fn contained_by_rec(bag_map: &BagMap, outer_colour: String) -> usize {
        bag_map
            .get(&outer_colour)
            .unwrap()
            .iter()
            .map(|(quantity, colour)| {
                quantity + (quantity * contained_by_rec(bag_map, colour.to_string()))
            })
            .sum()
    }

    contained_by_rec(&bag_map, "shiny gold".to_string())
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let bag_map: BagMap = parse::<ParsedEntry>(input)?.collect();
    println!("The answer to part one is: {}", total_containing_shiny_gold(bag_map));
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let bag_map: BagMap = parse::<ParsedEntry>(input)?.collect();
    println!("The answer to part two is: {}", total_contained_by_shiny_gold(bag_map));
    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[cfg(test)]
#[test]
fn test_total_containing_shiny_gold() {
    let example: BagMap = vec![
        "light red bags contain 1 bright white bag, 2 muted yellow bags.".to_string(),
        "dark orange bags contain 3 bright white bags, 4 muted yellow bags.".to_string(),
        "bright white bags contain 1 shiny gold bag.".to_string(),
        "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.".to_string(),
        "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.".to_string(),
        "dark olive bags contain 3 faded blue bags, 4 dotted black bags.".to_string(),
        "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.".to_string(),
        "faded blue bags contain no other bags.".to_string(),
        "dotted black bags contain no other bags.".to_string(),
    ]
    .iter()
    .map(|s| s.parse::<ParsedEntry>().unwrap())
    .collect();

    assert_eq!(total_containing_shiny_gold(example), 4);
}

#[test]
fn test_total_contained_by_shiny_gold() {
    let example: BagMap = vec![
        "light red bags contain 1 bright white bag, 2 muted yellow bags.".to_string(),
        "dark orange bags contain 3 bright white bags, 4 muted yellow bags.".to_string(),
        "bright white bags contain 1 shiny gold bag.".to_string(),
        "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.".to_string(),
        "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.".to_string(),
        "dark olive bags contain 3 faded blue bags, 4 dotted black bags.".to_string(),
        "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.".to_string(),
        "faded blue bags contain no other bags.".to_string(),
        "dotted black bags contain no other bags.".to_string(),
    ]
    .iter()
    .map(|s| s.parse::<ParsedEntry>().unwrap())
    .collect();

    assert_eq!(total_contained_by_shiny_gold(example), 32);

    let example: BagMap = vec![
        "shiny gold bags contain 2 dark red bags.".to_string(),
        "dark red bags contain 2 dark orange bags.".to_string(),
        "dark orange bags contain 2 dark yellow bags.".to_string(),
        "dark yellow bags contain 2 dark green bags.".to_string(),
        "dark green bags contain 2 dark blue bags.".to_string(),
        "dark blue bags contain 2 dark violet bags.".to_string(),
        "dark violet bags contain no other bags.".to_string(),
    ]
    .iter()
    .map(|s| s.parse::<ParsedEntry>().unwrap())
    .collect();

    assert_eq!(total_contained_by_shiny_gold(example), 126);
}
