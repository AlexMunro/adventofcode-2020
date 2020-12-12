use aoc2020::parse;

use std::path::Path;
use thiserror::Error;
use itertools::Itertools;
use itertools::enumerate;

#[derive(PartialEq, Eq, Clone, Copy)]
enum SpaceType {
    EmptySeat,
    OccupiedSeat,
    Floor,
}

impl SpaceType {
    fn from_char(c: char) -> Option<Self> {
        match c {
            'L' => Some(Self::EmptySeat),
            '#' => Some(Self::OccupiedSeat),
            '.' => Some(Self::Floor),
            _ => None,
        }
    }
}

fn progress_state(state: &Vec<Vec<SpaceType>>) -> Vec<Vec<SpaceType>> {
    let column_length = state.len();
    let row_length = state.get(0).unwrap().len();

    let adjacent = |x: usize, y: usize| {
        let lower_y = if y > 0 { y - 1 } else { 0 };
        let upper_y = if y < column_length - 1 { y + 1 } else { y };

        let lower_x = if x > 0 { x - 1 } else { 0 };
        let upper_x = if x < row_length - 1 { x + 1 } else { x };

        (lower_x..=upper_x).cartesian_product(lower_y..=upper_y).filter(move |(adj_x, adj_y)| *adj_x != x || *adj_y != y )
    };

    enumerate(state).map(
        |(y, row)| enumerate(row).map(
            |(x, space)| {
                let neighbours = adjacent(x, y).map(|(x, y)| state[y][x]);

                match space {
                    SpaceType::EmptySeat => {
                        if neighbours.filter(|s| *s == SpaceType::OccupiedSeat).count() == 0 {
                            SpaceType::OccupiedSeat
                        } else {
                            SpaceType::EmptySeat
                        }
                    },
                    SpaceType::OccupiedSeat => {
                        if neighbours.filter(|s| *s == SpaceType::OccupiedSeat).count() >= 4 {
                            SpaceType::EmptySeat
                        } else {
                            SpaceType::OccupiedSeat
                        }
                    }
                    SpaceType::Floor => SpaceType::Floor
                }
            }
        ).collect::<Vec<SpaceType>>()
    ).collect::<Vec<Vec<SpaceType>>>()
}

fn steady_state(initial_state: Vec<Vec<SpaceType>>) -> usize {
    fn occupied_chairs(state: &Vec<Vec<SpaceType>>) -> usize {
        state
            .iter()
            .map(|row| {
                row.iter()
                    .filter(|s| *s == &SpaceType::OccupiedSeat)
                    .count()
            })
            .sum()
    }

    let mut prev_state = initial_state;
    let mut current_state = progress_state(&prev_state);

    while occupied_chairs(&prev_state) != occupied_chairs(&current_state) {
        prev_state = current_state;
        current_state = progress_state(&prev_state);
    }

    occupied_chairs(&current_state)
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let initial_state: Vec<Vec<SpaceType>> = parse(input)?.take_while(|s| s != "")
        .map(|s: String| s.chars().filter_map(|c| SpaceType::from_char(c)).collect())
        .collect();
    println!("The answer to part one is {}", steady_state(initial_state));
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
fn test_steady_state() {
    let example: Vec<Vec<SpaceType>> = [
        "L.LL.LL.LL",
        "LLLLLLL.LL",
        "L.L.L..L..",
        "LLLL.LL.LL",
        "L.LL.LL.LL",
        "L.LLLLL.LL",
        "..L.L.....",
        "LLLLLLLLLL",
        "L.LLLLLL.L",
        "L.LLLLL.LL",
    ]
    .iter()
    .map(|s| s.chars().filter_map(|c| SpaceType::from_char(c)).collect())
    .collect();

    assert_eq!(steady_state(example), 37);
}
