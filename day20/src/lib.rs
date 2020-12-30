use aoc2020::parse_newline_sep;

use std::collections::{HashMap, HashSet};
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Clone)]
struct Tile {
    id: usize,
    contents: Vec<String>,
}

impl Tile {
    fn edges(&self) -> [String; 4] {
        [
            self.contents[0].clone(),
            self.contents[&self.contents.len() - 1].clone(),
            self.contents
                .iter()
                .fold(String::new(), |mut string, next_line| {
                    string.push(next_line.chars().next().unwrap());
                    string
                })
                .clone(),
            self.contents
                .iter()
                .fold(String::new(), |mut string, next_line| {
                    string.push(next_line.chars().last().unwrap());
                    string
                })
                .clone(),
        ]
    }
}

fn parse_tile(mut lines: Vec<String>) -> Tile {
    Tile {
        id: lines
            .remove(0)
            .chars()
            .filter(|c| (*c).is_digit(10))
            .collect::<String>()
            .parse()
            .unwrap(),
        contents: lines,
    }
}

fn reverse_string(string: &String) -> String {
    string.chars().rev().collect()
}

fn stored_string(string: &String) -> String {
    let opts = [&string, &reverse_string(&string)];
    opts.iter().min().unwrap().to_string()
}

fn find_corner_pieces(tiles: Vec<Tile>) -> Vec<Tile> {
    let mut edge_map = HashMap::<String, HashSet<usize>>::new();

    for t in &tiles {
        for edge in t.edges().iter() {
            let opts = [&edge, &reverse_string(&edge)];
            let unique_edge = opts.iter().min().unwrap().to_string();

            let existing = edge_map
                .entry(unique_edge.clone())
                .or_insert(HashSet::new());
            existing.insert(t.id);
        }
    }

    tiles
        .into_iter()
        .filter(|t| {
            t.edges()
                .iter()
                .filter(|e| {
                    edge_map
                        .get(&stored_string(e))
                        .unwrap()
                        .difference(&[t.id].iter().cloned().collect::<HashSet<usize>>())
                        .collect::<Vec<&usize>>()
                        .len()
                        == 0
                })
                .count()
                == 2
        })
        .collect()
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let tiles: Vec<Tile> = parse_newline_sep::<String>(input)?
        .map(|s| {
            s.split("\n")
                .filter(|l| *l != "")
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        })
        .map(|lines| parse_tile(lines))
        .collect();

    println!(
        "The answer to part one is {}",
        find_corner_pieces(tiles)
            .iter()
            .map(|t| t.id)
            .product::<usize>()
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
fn test_find_corner_pieces() {
    let example_tiles: Vec<Tile> = vec![
        vec![
            "Tile 2311:".to_string(),
            "..##.#..#.".to_string(),
            "##..#.....".to_string(),
            "#...##..#.".to_string(),
            "####.#...#".to_string(),
            "##.##.###.".to_string(),
            "##...#.###".to_string(),
            ".#.#.#..##".to_string(),
            "..#....#..".to_string(),
            "###...#.#.".to_string(),
            "..###..###".to_string(),
        ],
        vec![
            "Tile 1951:".to_string(),
            "#.##...##.".to_string(),
            "#.####...#".to_string(),
            ".....#..##".to_string(),
            "#...######".to_string(),
            ".##.#....#".to_string(),
            ".###.#####".to_string(),
            "###.##.##.".to_string(),
            ".###....#.".to_string(),
            "..#.#..#.#".to_string(),
            "#...##.#..".to_string(),
        ],
        vec![
            "Tile 1171:".to_string(),
            "####...##.".to_string(),
            "#..##.#..#".to_string(),
            "##.#..#.#.".to_string(),
            ".###.####.".to_string(),
            "..###.####".to_string(),
            ".##....##.".to_string(),
            ".#...####.".to_string(),
            "#.##.####.".to_string(),
            "####..#...".to_string(),
            ".....##...".to_string(),
        ],
        vec![
            "Tile 1427:".to_string(),
            "###.##.#..".to_string(),
            ".#..#.##..".to_string(),
            ".#.##.#..#".to_string(),
            "#.#.#.##.#".to_string(),
            "....#...##".to_string(),
            "...##..##.".to_string(),
            "...#.#####".to_string(),
            ".#.####.#.".to_string(),
            "..#..###.#".to_string(),
            "..##.#..#.".to_string(),
        ],
        vec![
            "Tile 1489:".to_string(),
            "##.#.#....".to_string(),
            "..##...#..".to_string(),
            ".##..##...".to_string(),
            "..#...#...".to_string(),
            "#####...#.".to_string(),
            "#..#.#.#.#".to_string(),
            "...#.#.#..".to_string(),
            "##.#...##.".to_string(),
            "..##.##.##".to_string(),
            "###.##.#..".to_string(),
        ],
        vec![
            "Tile 2473:".to_string(),
            "#....####.".to_string(),
            "#..#.##...".to_string(),
            "#.##..#...".to_string(),
            "######.#.#".to_string(),
            ".#...#.#.#".to_string(),
            ".#########".to_string(),
            ".###.#..#.".to_string(),
            "########.#".to_string(),
            "##...##.#.".to_string(),
            "..###.#.#.".to_string(),
        ],
        vec![
            "Tile 2971:".to_string(),
            "..#.#....#".to_string(),
            "#...###...".to_string(),
            "#.#.###...".to_string(),
            "##.##..#..".to_string(),
            ".#####..##".to_string(),
            ".#..####.#".to_string(),
            "#..#.#..#.".to_string(),
            "..####.###".to_string(),
            "..#.#.###.".to_string(),
            "...#.#.#.#".to_string(),
        ],
        vec![
            "Tile 2729:".to_string(),
            "...#.#.#.#".to_string(),
            "####.#....".to_string(),
            "..#.#.....".to_string(),
            "....#..#.#".to_string(),
            ".##..##.#.".to_string(),
            ".#.####...".to_string(),
            "####.#.#..".to_string(),
            "##.####...".to_string(),
            "##..#.##..".to_string(),
            "#.##...##.".to_string(),
        ],
        vec![
            "Tile 3079:".to_string(),
            "#.#.#####.".to_string(),
            ".#..######".to_string(),
            "..#.......".to_string(),
            "######....".to_string(),
            "####.#..#.".to_string(),
            ".#...#.##.".to_string(),
            "#.#####.##".to_string(),
            "..#.###...".to_string(),
            "..#.......".to_string(),
            "..#.###...".to_string(),
        ],
    ]
    .into_iter()
    .map(|lines| parse_tile(lines))
    .collect();

    assert_eq!(
        find_corner_pieces(example_tiles)
            .iter()
            .map(|t| t.id)
            .product::<usize>(),
        20899048083289
    );
}
