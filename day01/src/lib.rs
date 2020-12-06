use aoc2020::parse;

use std::collections::HashSet;
use std::path::Path;
use thiserror::Error;

fn product_of_2020_pair(entries: HashSet<usize>) -> Option<usize> {
    let first = entries.iter().find(|n| entries.contains(&(2020 - *n)));

    match first {
        Some(n) => Some(n * (2020 - n)),
        None => None,
    }
}

fn product_of_2020_triple(entries: HashSet<usize>) -> Option<usize> {
    // Cartesian product of the entries set with itself
    let mut zipped = entries
        .iter()
        .flat_map(|a| entries.iter().map(move |b| (a, b)))
        .filter(|(a, b)| a != b);

    // The first check prevents unsigned int underflow
    let first_pair = zipped.find(|(a, b)| 2020 > *a + *b && entries.contains(&(2020 - *a - *b)));

    match first_pair {
        Some((a, b)) => Some(a * b * (2020 - a - b)),
        None => None,
    }
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let entries: HashSet<usize> = parse(input)?.collect();

    match product_of_2020_pair(entries) {
        Some(n) => println!("{}", n),
        None => println!("Solution not found."),
    }
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let entries: HashSet<usize> = parse(input)?.collect();

    match product_of_2020_triple(entries) {
        Some(n) => println!("{}", n),
        None => println!("Solution not found."),
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_product_of_2020_pair() {
        let entries = [1721, 979, 366, 299, 675, 1456].iter().cloned().collect();
        let expected = Some(514579);
        assert_eq!(product_of_2020_pair(entries), expected);
    }

    #[test]
    fn test_product_of_2020_triple() {
        let entries = [1721, 979, 366, 299, 675, 1456].iter().cloned().collect();
        let expected = Some(241861950);
        assert_eq!(product_of_2020_triple(entries), expected);
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
