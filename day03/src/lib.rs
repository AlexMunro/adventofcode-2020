use aoc2020::parse;

use std::path::Path;
use thiserror::Error;

struct Slope {
    down: usize,
    right: usize
}

impl Slope {
    fn trees_in_map(&self, map: &Vec<String>) -> usize
    {
        let mut map_iter = map.into_iter();
        let line_length = map_iter.next().unwrap().chars().count();

        let mut count = 0;
        let mut pos = 0;

        loop {
            for _ in 1..self.down {
                map_iter.next();
            }

            match map_iter.next(){

                Some(s) => {
                    if s == "" {
                        return count
                    }
                    pos = (pos + self.right) % line_length;
                    if s.chars().nth(pos as usize).unwrap() == '#' {
                        count += 1;
                    }
                }
                None => return count
            }
        }
    }
}

fn product_of_trees_in_maps(map: Vec<String>) -> usize
{
    let slopes = [
        Slope{down: 1, right: 1},
        Slope{down: 1, right: 3},
        Slope{down: 1, right: 5},
        Slope{down: 1, right: 7},
        Slope{down:2, right: 1}
    ];

    slopes.iter().map(|s| s.trees_in_map(&map)).product()
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let map: Vec<String> = parse::<String>(input)?.take_while(|s| s != "").collect();
    println!("The answer to part one is {}", Slope{down:1, right: 3}.trees_in_map(&map));
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let map: Vec<String> = parse::<String>(input)?.take_while(|s| s != "").collect();
    println!("The answer to part one is {}", product_of_trees_in_maps(map));
    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[cfg(test)]

#[test]
fn test_trees_in_map() {
    let example = &vec![
        "..##.........##.........##.........##.........##.........##.......".to_string(),
        "#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..".to_string(),
        ".#....#..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.".to_string(),
        "..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#".to_string(),
        ".#...##..#..#...##..#..#...##..#..#...##..#..#...##..#..#...##..#.".to_string(),
        "..#.##.......#.##.......#.##.......#.##.......#.##.......#.##.....".to_string(),
        ".#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#".to_string(),
        ".#........#.#........#.#........#.#........#.#........#.#........#".to_string(),
        "#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...".to_string(),
        "#...##....##...##....##...##....##...##....##...##....##...##....#".to_string(),
        ".#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#".to_string()
    ];

    let expected = 7;
    assert_eq!(Slope{down:1, right: 3}.trees_in_map(example), expected);
}   

#[test]
fn test_product_of_trees_in_maps() {
    let example = vec![
        "..##.........##.........##.........##.........##.........##.......".to_string(),
        "#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..".to_string(),
        ".#....#..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.".to_string(),
        "..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#".to_string(),
        ".#...##..#..#...##..#..#...##..#..#...##..#..#...##..#..#...##..#.".to_string(),
        "..#.##.......#.##.......#.##.......#.##.......#.##.......#.##.....".to_string(),
        ".#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#".to_string(),
        ".#........#.#........#.#........#.#........#.#........#.#........#".to_string(),
        "#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...".to_string(),
        "#...##....##...##....##...##....##...##....##...##....##...##....#".to_string(),
        ".#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#".to_string()
    ];

    let expected = 336;
    assert_eq!(product_of_trees_in_maps(example), expected);
}