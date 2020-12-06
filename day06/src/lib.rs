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

fn total_completely_yessed(group_response: String) -> usize {
    let mut individual_responses = group_response.split_whitespace();

    let first_response = individual_responses.next().unwrap();

    first_response.chars().unique().filter(
        |c| individual_responses.clone().all(|r| r.contains(*c))
    ).count()
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let group_responses_count: usize = parse_newline_sep::<String>(input)
                                        .unwrap()
                                        .map(|group_response| total_yessed(group_response))
                                        .sum();
    println!("The answer to part one is {}", group_responses_count);
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let group_responses_count: usize = parse_newline_sep::<String>(input)
                                        .unwrap()
                                        .map(|group_response| total_completely_yessed(group_response))
                                        .sum();
    println!("The answer to part two is {}", group_responses_count);
    Ok(())
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

#[test]
fn test_total_completely_yessed(){
    assert_eq!(total_completely_yessed("abc".to_string()), 3);
    assert_eq!(total_completely_yessed("a\na".to_string()), 1);
    assert_eq!(total_completely_yessed("a\nb".to_string()), 0);
    assert_eq!(total_completely_yessed("b\nab".to_string()), 1);
}