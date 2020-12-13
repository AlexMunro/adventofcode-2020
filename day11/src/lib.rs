use aoc2020::parse;

use itertools::enumerate;
use itertools::Itertools;
use std::path::Path;
use thiserror::Error;

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

fn progress_state_immediate(state: &Vec<Vec<SpaceType>>) -> Vec<Vec<SpaceType>> {
    let column_length = state.len();
    let row_length = state.get(0).unwrap().len();

    let adjacent = |x: usize, y: usize| {
        let lower_y = if y > 0 { y - 1 } else { 0 };
        let upper_y = if y < column_length - 1 { y + 1 } else { y };

        let lower_x = if x > 0 { x - 1 } else { 0 };
        let upper_x = if x < row_length - 1 { x + 1 } else { x };

        (lower_x..=upper_x)
            .cartesian_product(lower_y..=upper_y)
            .filter(move |(adj_x, adj_y)| *adj_x != x || *adj_y != y)
    };

    enumerate(state)
        .map(|(y, row)| {
            enumerate(row)
                .map(|(x, space)| match space {
                    SpaceType::EmptySeat => {
                        if adjacent(x, y)
                            .map(|(x, y)| state[y][x])
                            .filter(|s| *s == SpaceType::OccupiedSeat)
                            .count()
                            == 0
                        {
                            SpaceType::OccupiedSeat
                        } else {
                            SpaceType::EmptySeat
                        }
                    }
                    SpaceType::OccupiedSeat => {
                        if adjacent(x, y)
                            .map(|(x, y)| state[y][x])
                            .filter(|s| *s == SpaceType::OccupiedSeat)
                            .count()
                            >= 4
                        {
                            SpaceType::EmptySeat
                        } else {
                            SpaceType::OccupiedSeat
                        }
                    }
                    SpaceType::Floor => SpaceType::Floor,
                })
                .collect::<Vec<SpaceType>>()
        })
        .collect::<Vec<Vec<SpaceType>>>()
}

struct Direction {
    x_dir: Box<dyn Fn(usize) -> Option<usize>>,
    y_dir: Box<dyn Fn(usize) -> Option<usize>>,
}

fn progress_state_linear(state: &Vec<Vec<SpaceType>>) -> Vec<Vec<SpaceType>> {
    let column_length = state.len();
    let row_length = state.get(0).unwrap().len();

    let visible_occupied = |x: usize, y: usize| -> usize {
        let occupied_in_direction = |dir: &Direction| {
            let mut current_x = x;
            let mut current_y = y;

            loop {
                current_x = match (dir.x_dir)(current_x) {
                    Some(n) => {
                        if n < row_length {
                            n
                        } else {
                            return false;
                        }
                    }
                    None => return false,
                };

                current_y = match (dir.y_dir)(current_y) {
                    Some(n) => {
                        if n < column_length {
                            n
                        } else {
                            return false;
                        }
                    }
                    None => return false,
                };

                match state[current_y][current_x] {
                    SpaceType::OccupiedSeat => return true,
                    SpaceType::EmptySeat => return false,
                    SpaceType::Floor => (),
                }
            }
        };

        [
            Direction {
                // Up-left
                x_dir: Box::new(|n| n.checked_sub(1)),
                y_dir: Box::new(|n| n.checked_sub(1)),
            },
            Direction {
                // Left
                x_dir: Box::new(|n| n.checked_sub(1)),
                y_dir: Box::new(|n| Some(n)),
            },
            Direction {
                // Down-left
                x_dir: Box::new(|n| n.checked_sub(1)),
                y_dir: Box::new(|n| n.checked_add(1)),
            },
            Direction {
                // Down
                x_dir: Box::new(|n| Some(n)),
                y_dir: Box::new(|n| n.checked_add(1)),
            },
            Direction {
                // Down-right
                x_dir: Box::new(|n| n.checked_add(1)),
                y_dir: Box::new(|n| n.checked_add(1)),
            },
            Direction {
                // Right
                x_dir: Box::new(|n| n.checked_add(1)),
                y_dir: Box::new(|n| Some(n)),
            },
            Direction {
                // Up-right
                x_dir: Box::new(|n| n.checked_add(1)),
                y_dir: Box::new(|n| n.checked_sub(1)),
            },
            Direction {
                // Up
                x_dir: Box::new(|n| Some(n)),
                y_dir: Box::new(|n| n.checked_sub(1)),
            },
        ]
        .iter()
        .filter(|dir| occupied_in_direction(dir))
        .count()
    };

    enumerate(state)
        .map(|(y, row)| {
            enumerate(row)
                .map(|(x, space)| match space {
                    SpaceType::EmptySeat => {
                        if visible_occupied(x, y) == 0 {
                            SpaceType::OccupiedSeat
                        } else {
                            SpaceType::EmptySeat
                        }
                    }
                    SpaceType::OccupiedSeat => {
                        if visible_occupied(x, y) >= 5 {
                            SpaceType::EmptySeat
                        } else {
                            SpaceType::OccupiedSeat
                        }
                    }
                    SpaceType::Floor => SpaceType::Floor,
                })
                .collect::<Vec<SpaceType>>()
        })
        .collect::<Vec<Vec<SpaceType>>>()
}

fn steady_state(
    initial_state: Vec<Vec<SpaceType>>,
    progress_state: &dyn Fn(&Vec<Vec<SpaceType>>) -> Vec<Vec<SpaceType>>,
) -> usize {
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
    let initial_state: Vec<Vec<SpaceType>> = parse(input)?
        .take_while(|s| s != "")
        .map(|s: String| s.chars().filter_map(|c| SpaceType::from_char(c)).collect())
        .collect();
    println!(
        "The answer to part one is {}",
        steady_state(initial_state, &progress_state_immediate)
    );
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let initial_state: Vec<Vec<SpaceType>> = parse(input)?
        .take_while(|s| s != "")
        .map(|s: String| s.chars().filter_map(|c| SpaceType::from_char(c)).collect())
        .collect();
    println!(
        "The answer to part two is {}",
        steady_state(initial_state, &progress_state_linear)
    );
    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[cfg(test)]
#[test]
fn test_immediate_steady_state() {
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

    assert_eq!(steady_state(example, &progress_state_immediate), 37);
}

#[test]
fn test_linear_steady_state() {
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

    assert_eq!(steady_state(example, &progress_state_linear), 26);
}
