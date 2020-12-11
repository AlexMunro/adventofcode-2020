use aoc2020::parse;
use std::collections::BTreeSet;
use std::path::Path;
use thiserror::Error;

fn summed_by_preamble(n: usize, preamble: &BTreeSet<usize>) -> bool{
    let mut min_iter = preamble.iter();
    let mut max_iter = preamble.iter().rev();

    let mut min = min_iter.next().unwrap();
    let mut max = max_iter.next().unwrap();

    while max > min {
        let result = min + max;
        if result == n {
            return true
        } else if result < n {
            min = min_iter.next().unwrap();
        } else {
            max = max_iter.next().unwrap();
        }
    }
    false
}

fn first_non_sum(xs: Vec<usize>, preamble_length: usize) -> usize {
    let mut preamble: BTreeSet<usize> = xs[..preamble_length].iter().map(|n| *n).collect();
    for n in preamble_length..xs.len(){
        if !summed_by_preamble(xs[n], &preamble){
            return xs[n]
        }
        preamble.remove(&xs[n - preamble_length]);
        preamble.insert(xs[n]);
    }
    panic!("There are no invalid entries")
}

pub fn part1(input: &Path) -> Result<(), Error> {
    println!("The answer to part one is {}", first_non_sum(parse(input)?.collect(), 25));
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
fn test_first_non_sum() {
    let example = vec![
        35,
        20,
        15,
        25,
        47,
        40,
        62,
        55,
        65,
        95,
        102,
        117,
        150,
        182,
        127,
        219,
        299,
        277,
        309,
        576,
    ];

    assert_eq!(first_non_sum(example, 5), 127);
}