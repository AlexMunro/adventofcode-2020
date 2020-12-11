use aoc2020::parse;
use std::collections::HashSet;
use std::iter;
use std::path::Path;
use thiserror::Error;

fn joltage_difference_counts(joltages: &HashSet<usize>) -> (usize, usize) {
    let mut one_jolt_jumps = 0;
    let mut three_jolt_jumps = 0;

    let joltage_iter = joltages.iter();

    for j in iter::once(&0).chain(joltage_iter.take(joltages.len())) {
        if joltages.contains(&(j + 1)) {
            one_jolt_jumps += 1
        } else {
            three_jolt_jumps += 1
        }
    }

    (one_jolt_jumps, three_jolt_jumps)
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let (one_diffs, three_diffs) = joltage_difference_counts(&parse(input)?.collect());
    println!(
        "The answer to part one is {}",
        one_diffs * three_diffs
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
fn test_joltage_difference() {
    let example = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4].into_iter().collect();
    assert_eq!(joltage_difference_counts(&example), (7, 5));

    let example = vec![
        28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8,
        17, 7, 9, 4, 2, 34, 10, 3,
    ].into_iter().collect();
    assert_eq!(joltage_difference_counts(&example), (22, 10));
}
