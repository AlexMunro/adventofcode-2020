use aoc2020::parse;

use std::path::Path;
use thiserror::Error;

const MODULO: usize = 20201227;

fn transform(key: usize, times: usize) -> usize {
    (0..times).fold(1, |acc, _| (acc * key) % MODULO)
}

fn get_key(card_key: usize, door_key: usize) -> usize {
    let mut subj = 1;
    let mut card_priv = 0;
    while subj != card_key {
        subj = (subj * 7) % MODULO;
        card_priv += 1;
    }
    
    transform(door_key, card_priv)
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let mut input = parse::<usize>(input)?.take(2);

    println!(
        "The answer to part one is {:?}",
        get_key(input.next().unwrap(), input.next().unwrap())
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
fn test_get_key() {
    assert_eq!(get_key(17807724, 5764801), 14897079);
}

#[test]
fn test_transform(){
    assert_eq!(transform(17807724, 8), 14897079);
    assert_eq!(transform(5764801, 11), 14897079);
}