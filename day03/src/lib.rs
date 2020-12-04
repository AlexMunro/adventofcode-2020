use aoc2020::parse;

use std::path::Path;
use thiserror::Error;

fn trees_in_path<I>(mut path: I) -> usize
where
    I: Iterator<Item = String>
{
    let line_length = path.next().unwrap().chars().count();

    let mut count = 0;
    let mut pos = 0;

    loop {
        match path.next(){
            Some(s) => {
                if s == "" {
                    return count
                }
                pos = (pos + 3) % line_length;
                if s.chars().nth(pos).unwrap() == '#' {
                    count += 1;
                }
                println!("{} @ {} % {} [{}]", s, pos, line_length, count);
            }
            None => return count
        }
    }
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let path = parse::<String>(input)?;
    println!("The answer to part one is {}", trees_in_path(path));
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    unimplemented!()
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

// TODO: make the type system maybe not hate my test input
//
// #[cfg(test)]

// #[test]
// fn test_trees_in_path() {
//     let mut example = [
//         "..##.........##.........##.........##.........##.........##.......".to_string(),
//         "#..O#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..".to_string(),
//         ".#....X..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.".to_string(),
//         "..#.#...#O#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#".to_string(),
//         ".#...##..#..X...##..#..#...##..#..#...##..#..#...##..#..#...##..#.".to_string(),
//         "..#.##.......#.X#.......#.##.......#.##.......#.##.......#.##.....".to_string(),
//         ".#.#.#....#.#.#.#.O..#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#".to_string(),
//         ".#........#.#........X.#........#.#........#.#........#.#........#".to_string(),
//         "#.##...#...#.##...#...#.X#...#...#.##...#...#.##...#...#.##...#...".to_string(),
//         "#...##....##...##....##...#X....##...##....##...##....##...##....#".to_string(),
//         ".#..#...#.#.#..#...#.#.#..#...X.#.#..#...#.#.#..#...#.#.#..#...#.#".to_string()
//     ].into_iter();

//     let expected = 7;
//     assert_eq!(trees_in_path(example), expected);
// }   

