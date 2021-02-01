use aoc2020::parse_newline_sep;

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use thiserror::Error;

lazy_static! {
    static ref MONSTER: Regex = Regex::new(r"#....##....##....###").unwrap();
    static ref MONSTER_BOTTOM: Regex = Regex::new(r".#..#..#..#..#..#...").unwrap();
}

const MONSTER_HEAD_RELATIVE_INDEX: usize = 18;
const MAGIC_MONSTER_NUMBER: usize = 15; // The number of tiles a monster lives on

#[derive(Debug, Clone, PartialEq)]
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

    fn trim(&mut self) {
        self.contents.pop();
        self.contents.remove(0);
        for l in &mut self.contents {
            l.pop();
            l.remove(0);
        }
    }

    fn top_edge(&self) -> String {
        self.contents[0].clone()
    }

    fn bottom_edge(&self) -> String {
        self.contents[self.contents.len() - 1].clone()
    }

    fn left_edge(&self) -> String {
        self.contents
            .iter()
            .map(|l| l.chars().next().unwrap())
            .collect()
    }

    fn right_edge(&self) -> String {
        self.contents
            .iter()
            .map(|l| l.chars().last().unwrap())
            .collect()
    }

    fn flip(&self) -> Self {
        Self {
            id: self.id,
            contents: self
                .contents
                .iter()
                .map(|line| reverse_string(line))
                .collect(),
        }
    }

    fn rotate(&self, times: usize) -> Self {
        // Performs times clockwise rotations
        (0..times).fold(self.clone(), |tile, _| {
            let mut next = Vec::new();
            for _ in 0..self.contents.len() {
                next.push(String::new());
            }

            for i in 0..self.contents.len() {
                for j in 0..self.contents.len() {
                    next[j].push(tile.contents[i].chars().nth(j).unwrap())
                }
            }

            next = next.iter().map(|s| reverse_string(s)).collect();

            Self {
                id: self.id,
                contents: next,
            }
        })
    }

    fn orientations(&self) -> Vec<Tile> {
        vec![
            self.clone(),
            self.rotate(1),
            self.rotate(2),
            self.rotate(3),
            self.flip(),
            self.flip().rotate(1),
            self.flip().rotate(2),
            self.flip().rotate(3),
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

fn edge_map(tiles: &Vec<Tile>) -> HashMap<String, HashSet<usize>> {
    let mut edge_map = HashMap::new();

    for t in tiles {
        for edge in t.edges().iter() {
            let opts = [&edge, &reverse_string(&edge)];
            let unique_edge = opts.iter().min().unwrap().to_string();

            let existing = edge_map
                .entry(unique_edge.clone())
                .or_insert(HashSet::new());
            existing.insert(t.id);
        }
    }

    edge_map
}

fn find_corner_pieces(tiles: &Vec<Tile>) -> Vec<Tile> {
    let edge_map = edge_map(tiles);

    tiles
        .iter()
        .cloned()
        .filter(|t| {
            t.edges()
                .iter()
                .filter(|e| {
                    edge_map[&stored_string(e)]
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

fn assemble_jigsaw(tiles: Vec<Tile>) -> Vec<String> {
    let tile_map: HashMap<usize, Tile> = tiles.iter().map(|t| (t.id, t.clone())).collect();

    let edge_map = edge_map(&tiles);

    // These closures are needed because the edge-map is direction agnostic.
    // It may be worth changing the edge-map to redundantly store both directions
    // for each edge in case this is more performant.
    let get_top_edge = |t: &Tile| {
        let rev_edge: String = reverse_string(&t.top_edge());
        edge_map
            .get(&t.top_edge())
            .unwrap_or(edge_map.get(&rev_edge).unwrap_or(&HashSet::new()))
            .clone()
    };

    let get_left_edge = |t: &Tile| {
        let rev_edge: String = reverse_string(&t.left_edge());
        edge_map
            .get(&t.left_edge())
            .unwrap_or(edge_map.get(&rev_edge).unwrap_or(&HashSet::new()))
            .clone()
    };

    let get_right_edge = |t: &Tile| {
        let rev_edge: String = reverse_string(&t.right_edge());
        edge_map
            .get(&t.right_edge())
            .unwrap_or(edge_map.get(&rev_edge).unwrap_or(&HashSet::new()))
            .clone()
    };

    let get_bottom_edge = |t: &Tile| {
        let rev_edge: String = reverse_string(&t.bottom_edge());
        edge_map
            .get(&t.bottom_edge())
            .unwrap_or(edge_map.get(&rev_edge).unwrap_or(&HashSet::new()))
            .clone()
    };

    let top_left = tiles
        .iter()
        .cloned()
        .find(|t| {
            t.edges()
                .iter()
                .filter(|e| {
                    edge_map[&stored_string(e)]
                        .difference(&[t.id].iter().cloned().collect::<HashSet<usize>>())
                        .collect::<Vec<&usize>>()
                        .len()
                        == 0
                })
                .count()
                == 2
        })
        .unwrap()
        .orientations()
        .into_iter()
        .find(|t| get_top_edge(t).len() == 1 && get_left_edge(t).len() == 1)
        .unwrap();

    let tile_right_of = |tile: &Tile| {
        let tile_id = get_right_edge(tile)
            .iter()
            .find(|t| *t != &tile.id)
            .unwrap()
            .clone();

        tile_map[&tile_id]
            .orientations()
            .into_iter()
            .find(|orientation| orientation.left_edge() == tile.right_edge())
            .unwrap()
    };

    let tile_below = |tile: &Tile| {
        let tile_id = get_bottom_edge(tile)
            .iter()
            .find(|t| *t != &tile.id)
            .unwrap()
            .clone();
        tile_map[&tile_id]
            .orientations()
            .into_iter()
            .find(|orientation| orientation.top_edge() == tile.bottom_edge())
            .unwrap()
    };

    let tile_row_length = (tiles.len() as f64).sqrt() as usize;

    let mut assembled: Vec<Vec<Tile>> = (0..tile_row_length).map(|_| Vec::new()).collect();

    assembled[0].push(top_left);

    for row in 1..tile_row_length {
        let above = tile_below(&assembled[row - 1][0]);
        assembled[row].push(above);
    }

    for row in 0..tile_row_length {
        for column in 1..tile_row_length {
            let right = tile_right_of(&assembled[row][column - 1]);
            assembled[row].push(right);
        }
    }

    for row in 0..tile_row_length {
        for column in 0..tile_row_length {
            assembled[row][column].trim();
        }
    }

    let mut result = vec![];

    for tile_line in 0..tile_row_length {
        for row in 0..assembled[tile_line][0].contents.len() {
            result.push(
                assembled[tile_line]
                    .iter()
                    .map(|tile| tile.contents[row].clone())
                    .collect::<String>(),
            )
        }
    }

    result
}

fn has_any_sea_monsters(tile: &Tile) -> bool {
    tile.contents
        .iter()
        .skip(1)
        .any(|line| MONSTER.is_match(line))
}

fn water_roughness(tiles: Vec<Tile>) -> usize {
    let jigsaw = Tile {
        contents: assemble_jigsaw(tiles),
        id: 0,
    };

    let hash_tile_count: usize = jigsaw
        .contents
        .iter()
        .map(|line| line.chars().filter(|c| c == &'#').count())
        .sum();

    let correct_orientation = jigsaw
        .orientations()
        .into_iter()
        // Technically might yield false positives, but doesn't for given or test inputs
        .find(|tile| has_any_sea_monsters(tile))
        .unwrap();

    let monster_tile_count: usize = (1..correct_orientation.contents.len() - 1)
        .map(|line| {
            MONSTER
                .find_iter(&correct_orientation.contents[line])
                .filter(|monster_match| {
                    let index = monster_match.start();
                    MONSTER_BOTTOM.is_match_at(&correct_orientation.contents[line + 1], index)
                        && correct_orientation.contents[line - 1]
                            .chars()
                            .nth(index + MONSTER_HEAD_RELATIVE_INDEX)
                            .unwrap()
                            == '#'
                })
                .count()
        })
        .sum::<usize>()
        * MAGIC_MONSTER_NUMBER;
    hash_tile_count - monster_tile_count
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
        find_corner_pieces(&tiles)
            .iter()
            .map(|t| t.id)
            .product::<usize>()
    );

    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let tiles: Vec<Tile> = parse_newline_sep::<String>(input)?
        .map(|s| {
            s.split("\n")
                .filter(|l| *l != "")
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        })
        .map(|lines| parse_tile(lines))
        .collect();

    println!("The answer to part two is {}", water_roughness(tiles));

    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[cfg(test)]
fn example_tiles() -> Vec<Tile> {
    vec![
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
    .collect()
}

#[test]
fn test_find_corner_pieces() {
    assert_eq!(
        find_corner_pieces(&example_tiles())
            .iter()
            .map(|t| t.id)
            .product::<usize>(),
        20899048083289
    );
}

#[test]
fn test_water_roughness() {
    assert_eq!(water_roughness(example_tiles()), 273);
}

#[test]
fn test_assemble_jigsaw() {
    let expected = vec![
        ".#.#..#.##...#.##..#####".to_string(),
        "###....#.#....#..#......".to_string(),
        "##.##.###.#.#..######...".to_string(),
        "###.#####...#.#####.#..#".to_string(),
        "##.#....#.##.####...#.##".to_string(),
        "...########.#....#####.#".to_string(),
        "....#..#...##..#.#.###..".to_string(),
        ".####...#..#.....#......".to_string(),
        "#..#.##..#..###.#.##....".to_string(),
        "#.####..#.####.#.#.###..".to_string(),
        "###.#.#...#.######.#..##".to_string(),
        "#.####....##..########.#".to_string(),
        "##..##.#...#...#.#.#.#..".to_string(),
        "...#..#..#.#.##..###.###".to_string(),
        ".#.#....#.##.#...###.##.".to_string(),
        "###.#...#..#.##.######..".to_string(),
        ".#.#.###.##.##.#..#.##..".to_string(),
        ".####.###.#...###.#..#.#".to_string(),
        "..#.#..#..#.#.#.####.###".to_string(),
        "#..####...#.#.#.###.###.".to_string(),
        "#####..#####...###....##".to_string(),
        "#.##..#..#...#..####...#".to_string(),
        ".#.###..##..##..####.##.".to_string(),
        "...###...##...#...#..###".to_string(),
    ];

    let expectable_orientations = Tile {
        id: 0,
        contents: expected,
    }
    .orientations();

    let assembled = assemble_jigsaw(example_tiles());

    assert!(expectable_orientations
        .into_iter()
        .any(|orientation| { assembled == orientation.contents }))
}

#[test]
fn test_rotate() {
    let example = Tile {
        id: 0,
        contents: vec!["ABC".to_string(), "DEF".to_string(), "GHI".to_string()],
    };

    let expected = Tile {
        id: 0,
        contents: vec!["GDA".to_string(), "HEB".to_string(), "IFC".to_string()],
    };

    assert_eq!(example.rotate(1), expected);
    assert_eq!(example.rotate(4), example);
}

#[test]
fn test_flip() {
    let example = Tile {
        id: 0,
        contents: vec!["ABC".to_string(), "DEF".to_string(), "GHI".to_string()],
    };
    let expected = Tile {
        id: 0,
        contents: vec!["CBA".to_string(), "FED".to_string(), "IHG".to_string()],
    };
    assert_eq!(example.flip(), expected);
    assert_eq!(example.flip().flip(), example);
}

#[test]
fn test_has_any_sea_monsters() {
    let monstery_tile = Tile {
        id: 0,
        contents: vec![
            ".####...#####..#...###..".to_string(),
            "#####..#..#.#.####..#.#.".to_string(),
            ".#.#...#.###...#.##.##..".to_string(),
            "#.#.##.###.#.##.##.#####".to_string(),
            "..##.###.####..#.####.##".to_string(),
            "...#.#..##.##...#..#..##".to_string(),
            "#.##.#..#.#..#..##.#.#..".to_string(),
            ".###.##.....#...###.#...".to_string(),
            "#.####.#.#....##.#..#.#.".to_string(),
            "##...#..#....#..#...####".to_string(),
            "..#.##...###..#.#####..#".to_string(),
            "....#.##.#.#####....#...".to_string(),
            "..##.##.###.....#.##..#.".to_string(),
            "#...#...###..####....##.".to_string(),
            ".#.##...#.##.#.#.###...#".to_string(),
            "#.###.#..####...##..#...".to_string(),
            "#.###...#.##...#.######.".to_string(),
            ".###.###.#######..#####.".to_string(),
            "..##.#..#..#.#######.###".to_string(),
            "#.#..##.########..#..##.".to_string(),
            "#.#####..#.#...##..#....".to_string(),
            "#....##..#.#########..##".to_string(),
            "#...#.....#..##...###.##".to_string(),
            "#..###....##.#...##.##.#".to_string(),
        ],
    };

    assert!(has_any_sea_monsters(&monstery_tile));
    assert!(!has_any_sea_monsters(&monstery_tile.rotate(1)));
}
