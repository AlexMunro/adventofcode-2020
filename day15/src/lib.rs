use aoc2020::{parse, CommaSep};

use std::collections::HashMap;
use std::path::Path;
use thiserror::Error;

fn nth_turn(initial: &Vec<usize>, n: usize) -> usize {
    if n <= initial.len() {
        return initial[n - 1];
    }

    let mut last_seen: HashMap<usize, usize> = HashMap::new();

    for (idx, val) in initial.iter().take(initial.len()).enumerate() {
        last_seen.insert(*val, idx + 1);
    }

    (initial.len() + 1..=n).fold(initial[initial.len() - 1], |acc, next| {
        let next_value = next - 1 - last_seen.get(&acc).unwrap_or(&(next - 1));
        last_seen.insert(acc, next - 1);
        next_value
    })
}

pub fn part1(input: &Path) -> Result<(), Error> {
    println!(
        "The answer to part one is {}",
        nth_turn(&parse::<CommaSep<usize>>(input)?.flatten().collect(), 2020)
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
fn test_nth_turn() {
    let example = vec![0, 3, 6];
    assert_eq!(nth_turn(&example, 1), 0);
    assert_eq!(nth_turn(&example, 2), 3);
    assert_eq!(nth_turn(&example, 3), 6);
    assert_eq!(nth_turn(&example, 4), 0);
    assert_eq!(nth_turn(&example, 5), 3);
    assert_eq!(nth_turn(&example, 6), 3);
    assert_eq!(nth_turn(&example, 7), 1);
    assert_eq!(nth_turn(&example, 8), 0);
    assert_eq!(nth_turn(&example, 9), 4);
    assert_eq!(nth_turn(&example, 10), 0);
    assert_eq!(nth_turn(&example, 2020), 436);

    let example = vec![1, 3, 2];
    assert_eq!(nth_turn(&example, 2020), 1);

    let example = vec![2, 1, 3];
    assert_eq!(nth_turn(&example, 2020), 10);

    let example = vec![1, 2, 3];
    assert_eq!(nth_turn(&example, 2020), 27);

    let example = vec![2, 3, 1];
    assert_eq!(nth_turn(&example, 2020), 78);

    let example = vec![3, 2, 1];
    assert_eq!(nth_turn(&example, 2020), 438);

    let example = vec![3, 1, 2];
    assert_eq!(nth_turn(&example, 2020), 1836);
}
