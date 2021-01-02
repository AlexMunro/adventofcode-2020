use aoc2020::parse;

use std::collections::HashMap;
use std::path::Path;
use thiserror::Error;

fn simulate_moves(ordering: &mut HashMap<usize, usize>, first: usize, n: usize, max: usize) {
    let mut current_cup = first;
    for _ in 0..n {
        let cup_0 = ordering[&current_cup];
        let cup_1 = ordering[&cup_0];
        let cup_2 = ordering[&cup_1];
        let after_cups = ordering[&cup_2];

        let invalid_destinations = [current_cup, cup_0, cup_1, cup_2];

        let mut destination = current_cup - 1;
        while destination == 0 || invalid_destinations.contains(&destination) {
            if destination == 0 {
                destination = max;
            } else {
                destination -= 1;
            }
        }

        let new_after_cups = ordering[&destination];
        ordering.insert(current_cup, after_cups);
        ordering.insert(destination, cup_0);
        ordering.insert(cup_2, new_after_cups);

        current_cup = ordering[&current_cup] as usize;
    }
}

fn simulate_hundred_moves(initial_order: String) -> String {
    let mut ordering = HashMap::new();

    let from_chars = initial_order
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize);
    let to_chars = initial_order
        .chars()
        .cycle()
        .skip(1)
        .map(|c| c.to_digit(10).unwrap() as usize);

    for (from, to) in from_chars.zip(to_chars) {
        ordering.insert(from, to);
    }
    let first = initial_order.chars().next().unwrap().to_digit(10).unwrap() as usize;

    simulate_moves(&mut ordering, first, 100, 9);

    let mut final_ordering: Vec<usize> = Vec::new();
    let mut index = 1;

    for _ in 0..initial_order.len() - 1 {
        index = ordering[&index];
        final_ordering.push(index);
    }

    final_ordering
        .iter()
        .map(|d| d.to_string())
        .collect::<Vec<String>>()
        .join("")
}

fn simulate_ten_million_moves(initial_order: String) -> usize {
    let first = initial_order.chars().next().unwrap().to_digit(10).unwrap() as usize;
    let last_initial = initial_order.chars().last().unwrap().to_digit(10).unwrap() as usize;

    let mut ordering = HashMap::<usize, usize>::new();
    let from_chars = initial_order
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize);
    let to_chars = initial_order
        .chars()
        .cycle()
        .skip(1)
        .map(|c| c.to_digit(10).unwrap() as usize);

    for (from, to) in from_chars.zip(to_chars) {
        ordering.insert(from, to);
    }

    ordering.insert(last_initial, 10);

    for i in 10..1_000_000 {
        ordering.insert(i, i + 1);
    }
    ordering.insert(1_000_000, first);

    simulate_moves(&mut ordering, first, 10_000_000, 1_000_000);

    ordering[&1] * ordering[&ordering[&1]]
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let initial_order = parse::<String>(input)?.next().unwrap();

    println!(
        "The answer to part one is {:?}",
        simulate_hundred_moves(initial_order)
    );

    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let initial_order = parse::<String>(input)?.next().unwrap();

    println!(
        "The answer to part two is {:?}",
        simulate_ten_million_moves(initial_order)
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
fn test_simulate_hundred_moves() {
    let example = "389125467".to_string();
    assert_eq!(simulate_hundred_moves(example), "67384529");
}

#[test]
fn test_simulate_ten_million_moves() {
    let example = "389125467".to_string();
    assert_eq!(simulate_ten_million_moves(example), 149245887792);
}
