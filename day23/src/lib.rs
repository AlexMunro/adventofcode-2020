use aoc2020::parse;

use std::collections::VecDeque;
use std::path::Path;
use thiserror::Error;

fn simulate_moves(initial_order: String, n: usize) -> String {
    let mut ordering = VecDeque::new();
    for c in initial_order.chars() {
        ordering.push_back(c.to_digit(10).unwrap());
    }

    for _ in 0..n {
        let current_cup = ordering.pop_front().unwrap();
        ordering.push_back(current_cup);

        let cup_0 = ordering.pop_front().unwrap();
        let cup_1 = ordering.pop_front().unwrap();
        let cup_2 = ordering.pop_front().unwrap();

        let destination = *ordering
            .iter()
            .filter(|c| **c < current_cup)
            .max()
            .unwrap_or(ordering.iter().max().unwrap());

        let mut next_cup = ordering.pop_front().unwrap();

        let mut steps_to_return = 0;

        while next_cup != destination {
            ordering.push_back(next_cup);
            next_cup = ordering.pop_front().unwrap();
            steps_to_return += 1;
        }

        ordering.push_front(cup_2);
        ordering.push_front(cup_1);
        ordering.push_front(cup_0);
        ordering.push_front(next_cup);

        for _ in 0..steps_to_return {
            let temp = ordering.pop_back().unwrap();
            ordering.push_front(temp);
        }
    }

    // Getting the correct position
    let mut removed_cup = ordering.pop_front().unwrap();

    while removed_cup != 1 {
        ordering.push_back(removed_cup);
        removed_cup = ordering.pop_front().unwrap();
    }

    ordering
        .into_iter()
        .take(initial_order.len() - 1)
        .map(|d| d.to_string())
        .collect::<Vec<String>>()
        .join("")
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let initial_order = parse::<String>(input)?.next().unwrap();

    println!(
        "The answer to part one is {:?}",
        simulate_moves(initial_order, 100)
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
fn test_simulate_moves() {
    let example = "389125467".to_string();
    assert_eq!(simulate_moves(example, 100), "67384529");
}
