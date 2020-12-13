use aoc2020::parse;

use std::path::Path;
use thiserror::Error;

fn earliest_departure(timestamp: usize, ids: Vec<usize>) -> usize {
    let remaining_minutes = |id: usize| id - (timestamp % id);

    let earliest_id = *ids
        .iter()
        .min_by(|x, y| remaining_minutes(**x).cmp(&remaining_minutes(**y)))
        .unwrap();

    earliest_id * remaining_minutes(earliest_id)
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let mut input_iter = parse::<String>(input)?;
    let timestamp = input_iter.next().unwrap().parse()?;
    let ids = input_iter
        .next()
        .unwrap()
        .split(',')
        .filter_map(|s| s.parse().ok())
        .collect();

    println!(
        "The answer to part one is {}",
        earliest_departure(timestamp, ids)
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
    #[error("Could not parse integer")]
    ParseInt(#[from] std::num::ParseIntError),
}

#[cfg(test)]
#[test]
fn test_earliest_departure() {
    let timestamp = 939;
    let ids = vec![7, 13, 59, 31, 19];
    assert_eq!(earliest_departure(timestamp, ids), 295);
}
