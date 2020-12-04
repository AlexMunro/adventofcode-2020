use aoc2020::parse;

use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;
use std::path::Path;
use thiserror::Error;

lazy_static!{
    static ref RE: Regex = Regex::new(r"^(\d+)-(\d+) ([A-Za-z]): ([A-Za-z]+)$").unwrap();
}

struct PasswordEntry {
    min: usize,
    max: usize,
    letter: char,
    password: String,
}

impl FromStr for PasswordEntry {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "" {
            Err("Empty string, no need to parse")?;
        }

        let capture_groups = RE.captures(s).unwrap();

        Ok(
            PasswordEntry{
                min: capture_groups[1].parse().unwrap(),
                max: capture_groups[2].parse().unwrap(),
                letter: capture_groups[3].chars().next().unwrap(),
                password: String::from(&capture_groups[4])
            }
        )
    }
}

fn valid_password(entry: &PasswordEntry) -> bool {
    let count = entry.password
                    .chars()
                    .filter(|c| *c == entry.letter)
                    .count();

    count >= entry.min && count <= entry.max
}

fn valid_password_count<I>(entries: I) -> usize
where
    I: Iterator<Item = PasswordEntry>,
{
 
    entries.filter(|e| valid_password(e)).count()
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let input_iter = parse::<PasswordEntry>(input)?;
    let count = valid_password_count(input_iter);

    println!("The answer to part one is: {}", count);
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
mod tests {
    use super::*;

    #[test]
    fn test_valid_password(){
        let good_password = &PasswordEntry{min: 1, max: 3, letter: 'a', password: "abba".to_string()};
        assert!(valid_password(good_password));

        let bad_password = &PasswordEntry{min: 2, max: 4, letter: 'c', password: "cat".to_string()};
        assert!(!valid_password(bad_password));
    }

    #[test]
    fn test_valid_password_count(){
        let passwords = vec![ 
            "1-3 a: abcde",
            "1-3 b: cdefg",
            "2-9 c: ccccccccc",
        ].into_iter().map(|s| s.parse::<PasswordEntry>().unwrap());
        let expected = 2;

        assert_eq!(valid_password_count(passwords), expected)
    }
}