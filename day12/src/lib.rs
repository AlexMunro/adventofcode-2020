use aoc2020::parse;

use std::path::Path;
use std::str::FromStr;
use thiserror::Error;

#[derive(Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

impl Direction {
    fn next_left(&self) -> Direction {
        match &self {
            Self::North => Self::West,
            Self::West => Self::South,
            Self::South => Self::East,
            Self::East => Self::North,
            _ => panic!("Invalid use of next_left on a non-cardinal direction"),
        }
    }

    fn next_right(&self) -> Direction {
        match &self {
            Self::North => Self::East,
            Self::West => Self::North,
            Self::South => Self::West,
            Self::East => Self::South,
            _ => panic!("Invalid use of next_right on a non-cardinal direction"),
        }
    }
}

struct Instruction {
    dir: Direction,
    amount: usize,
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut string_iter = s.chars();

        let dir = match string_iter.next() {
            Some('N') => Direction::North,
            Some('S') => Direction::South,
            Some('E') => Direction::East,
            Some('W') => Direction::West,
            Some('L') => Direction::Left,
            Some('F') => Direction::Forward,
            Some('R') => Direction::Right,
            _ => return Err("Unable to parse direction".to_string()),
        };

        let amount = string_iter
            .collect::<String>()
            .parse::<usize>()
            .expect("Unable to parse amount");

        Ok(Self {
            dir: dir,
            amount: amount,
        })
    }
}

fn manhattan_dist(instrs: impl Iterator<Item = Instruction>) -> usize {
    let mut x = 0 as isize;
    let mut y = 0 as isize;
    let mut current_dir = Direction::East;

    let mut go_dir = |i: Instruction| {
        match i.dir {
            Direction::North => y += i.amount as isize,
            Direction::South => y -= i.amount as isize,
            Direction::East => x += i.amount as isize,
            Direction::West => x -= i.amount as isize,
            _ => {} // not reachable
        }
    };

    for i in instrs {
        match i.dir {
            Direction::Forward => go_dir(Instruction {
                dir: current_dir,
                amount: i.amount,
            }),
            Direction::Left => {
                for _ in 0..(i.amount / 90) {
                    current_dir = current_dir.next_left()
                }
            }
            Direction::Right => {
                for _ in 0..(i.amount / 90) {
                    current_dir = current_dir.next_right()
                }
            }
            _ => go_dir(i),
        }
    }
    (x.abs() + y.abs()) as usize
}

pub fn part1(input: &Path) -> Result<(), Error> {
    println!(
        "The answer to part one is {}",
        manhattan_dist(parse(input)?)
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
fn test_manhattan_dist() {
    let example = ["F10", "N3", "F7", "R90", "F11"]
        .iter()
        .map(|s| Instruction::from_str(s).unwrap());

    assert_eq!(manhattan_dist(example), 25);
}
