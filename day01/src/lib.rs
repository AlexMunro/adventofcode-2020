use aoc2020::parse;

use std::iter::FromIterator;
use std::collections::HashSet;
use std::path::Path;
use thiserror::Error;

fn product_of_2020_pair(entries: HashSet<usize>) -> Option<usize>{
    let first = entries.iter().find(|n| entries.contains(&(2020-*n)));

    match first {
        Some(n) => Some(n * (2020 - n)),
        None => None,
    }
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let entries: HashSet<usize> = parse(input)?.collect();

    match product_of_2020_pair(entries) {
        Some(n) => println!("{}", n),
        None => println!("Solution not found.")
    }
    Ok(())
}

pub fn part2(_input: &Path) -> Result<(), Error> {
    unimplemented!()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn product_of_2020_entries() {
        let entries = [1721, 979, 366, 299, 675, 1456].iter().cloned().collect();
        let expected = Some(514579);
        assert_eq!(product_of_2020_pair(entries), expected);
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
