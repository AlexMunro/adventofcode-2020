use aoc2020::parse;

use std::collections::HashSet;
use std::path::Path;
use thiserror::Error;

// Handling of arbitrary dimensions adapted from https://rosettacode.org/wiki/Cartesian_product_of_two_or_more_lists#Rust
fn neighbours(point: &Vec<isize>) -> HashSet<Vec<isize>> {
    point
        .iter()
        .fold(
            vec![Vec::<isize>::new()],
            |points: Vec<Vec<isize>>, new_dimension| {
                points
                    .iter()
                    .flat_map(|p| {
                        let neighbouring_points: Vec<Vec<isize>> = (-1..=1)
                            .map(|i| {
                                let mut new_points = p.clone();
                                new_points.push(new_dimension + i);
                                new_points
                            })
                            .collect();
                        neighbouring_points
                    })
                    .collect()
            },
        )
        .iter()
        .filter(|p| *p != point)
        .cloned()
        .collect()
}

fn cycle_cubes(cubes: HashSet<Vec<isize>>, n: usize) -> HashSet<Vec<isize>> {
    fn cycle(cubes: HashSet<Vec<isize>>) -> HashSet<Vec<isize>> {
        (&cubes)
            .iter()
            .fold(cubes.clone(), |set, next_point| {
                set.union(&neighbours(&(*next_point).to_vec()))
                    .cloned()
                    .collect()
            })
            .iter()
            .filter(|c| {
                let previously_active_neighbours = neighbours(*c)
                    .iter()
                    .filter(|c| (&cubes).contains(*c))
                    .count();

                if (&cubes).contains(*c) {
                    previously_active_neighbours == 2 || previously_active_neighbours == 3
                } else {
                    previously_active_neighbours == 3
                }
            })
            .cloned()
            .collect()
    }

    (0..n).fold(cubes, |prev_cubes, _| cycle(prev_cubes))
}

fn parse_initial_state(lines: Vec<String>) -> HashSet<Vec<isize>> {
    let mut point_set = HashSet::new();
    for i in 0..lines.len() {
        let active_in_line = lines[i]
            .chars()
            .enumerate()
            .filter(|(_, c)| *c == '#')
            .map(|(idx, _)| idx);

        for j in active_in_line {
            point_set.insert(vec![j as isize, i as isize, 0]);
        }
    }
    point_set
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let initial_state = parse_initial_state(parse(input)?.take_while(|s| *s != "").collect());
    println!(
        "The answer to part one is {}",
        cycle_cubes(initial_state, 6).len()
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
fn test_neighbours() {
    let example = &vec![0, 0];

    assert_eq!(
        neighbours(example),
        vec![
            vec![-1, -1],
            vec![-1, 0],
            vec![-1, 1],
            vec![0, -1],
            vec![0, 1],
            vec![1, -1],
            vec![1, 0],
            vec![1, 1],
        ]
        .iter()
        .cloned()
        .collect()
    )
}

#[test]
fn test_cycle_cubes() {
    let example = parse_initial_state(vec![
        ".#.".to_string(),
        "..#".to_string(),
        "###".to_string(),
    ]);

    assert_eq!(cycle_cubes(example, 6).len(), 112);
}
