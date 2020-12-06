use aoc2020::parse_newline_sep;

use std::path::Path;
use thiserror::Error;
use itertools::Itertools;

fn total_yessed(group_response: String) -> usize {
    group_response
        .chars()
        .filter(|c| *c != '\n')
        .unique()
        .count()
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let group_responses_count: usize = parse_newline_sep::<String>(input)
                                        .unwrap()
                                        .map(|group_response| total_yessed(group_response))
                                        .sum();
    println!("The answer to part one is {}", group_responses_count);
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
fn test_total_yessed() {
    assert_eq!(total_yessed("abc".to_string()), 3);
    assert_eq!(total_yessed("abac".to_string()), 3);
    assert_eq!(total_yessed("a".to_string()), 1);
    assert_eq!(total_yessed("a\na".to_string()), 1);
}