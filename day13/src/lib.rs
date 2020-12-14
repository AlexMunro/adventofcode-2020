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

fn earliest_contiguous_departure(id_list: String) -> usize {
    let mut ids: Vec<(usize, usize)> = id_list
        .split(",")
        .enumerate()
        .filter_map(|(x, y)| match y.parse::<usize>().ok() {
            Some(n) => Some((x, n)),
            None => None,
        })
        .collect();

    // Largest ID first
    ids.sort_by(|(_, id_a), (_, id_b)| id_b.cmp(id_a));

    (1..)
        .map(|n| (n * ids[0].1) - ids[0].0)
        .find(|n| ids.iter().all(|(pos, id)| (n + pos) % id == 0))
        .unwrap()
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

pub fn part2(input: &Path) -> Result<(), Error> {
    println!(
        "The answer to part two is {}",
        earliest_contiguous_departure(parse(input)?.nth(1).unwrap())
    );
    Ok(())
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

#[test]
fn test_earliest_contiguous_departures() {
    let ids = "17,x,13,19".to_string();
    assert_eq!(earliest_contiguous_departure(ids), 3417);

    let ids = "67,7,59,61".to_string();
    assert_eq!(earliest_contiguous_departure(ids), 754018);

    let ids = "67,x,7,59,61".to_string();
    assert_eq!(earliest_contiguous_departure(ids), 779210);

    let ids = "67,7,x,59,61".to_string();
    assert_eq!(earliest_contiguous_departure(ids), 1261476);

    let ids = "1789,37,47,1889".to_string();
    assert_eq!(earliest_contiguous_departure(ids), 1202161486);
}
