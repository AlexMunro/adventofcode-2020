use aoc2020::parse;
use std::collections::BTreeSet;
use std::collections::HashMap;
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
    println!("The answer to part one is {}", one_diffs * three_diffs);
    Ok(())
}

fn arrangements_count(joltages: BTreeSet<usize>) -> usize {
    let mut joltages_iter = joltages.iter().rev();
    let mut adapter_combo_map = HashMap::new();

    // This is the final connection to the device - the base case from the end!
    adapter_combo_map.insert(joltages_iter.next().unwrap(), 1);

    for n in joltages_iter.chain(iter::once(&0)) {
        adapter_combo_map.insert(
            n,
            (1..=3)
                .map(|i| adapter_combo_map.get(&(n + i)).unwrap_or(&0))
                .sum(),
        );
    }

    *adapter_combo_map.get(&0).unwrap()
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let joltages = parse(input)?.collect();
    println!("The answer to part two is {}", arrangements_count(joltages));
    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[cfg(test)]
#[test]
fn test_joltage_difference() {
    let example = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4]
        .into_iter()
        .collect();
    assert_eq!(joltage_difference_counts(&example), (7, 5));

    let example = vec![
        28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8,
        17, 7, 9, 4, 2, 34, 10, 3,
    ]
    .into_iter()
    .collect();
    assert_eq!(joltage_difference_counts(&example), (22, 10));
}

#[test]
fn test_arrangements_count() {
    let example = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4]
        .into_iter()
        .collect();
    assert_eq!(arrangements_count(example), 8);

    let example = vec![
        28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8,
        17, 7, 9, 4, 2, 34, 10, 3,
    ]
    .into_iter()
    .collect();
    assert_eq!(arrangements_count(example), 19208);
}
