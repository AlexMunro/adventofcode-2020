use aoc2020::parse;

use std::collections::HashSet;
use std::path::Path;
use thiserror::Error;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static!{
    static ref DIRECTIONS: Regex = Regex::new("(e|se|sw|w|nw|ne)").unwrap();
}

type Tile = (isize, isize);

fn parse_tile(directions: String) -> Tile {
    let mut north = 0;
    let mut east = 0;

    for dir in DIRECTIONS.find_iter(&directions) {
        match dir.as_str() {
            "e" => east += 2,
            "se" => {
                east += 1;
                north -=1;
            },
            "sw" => {
                east -= 1;
                north -= 1;
            },
            "w" => {
                east -= 2;
            },
            "nw" => {
                east -= 1;
                north += 1;
            },
            "ne" => {
                east += 1;
                north += 1;
            },
            _ => panic!("Invalid direction")
        }
    }

    (north, east)
}

fn count_flipped_tiles<I>(tiles: I) -> usize 
where 
    I: Iterator<Item = Tile>, {
        let mut flipped = HashSet::<Tile>::new();
        for tile in tiles {
            if flipped.contains(&tile) {
                flipped.remove(&tile);
            } else {
                flipped.insert(tile);
            }
        }
        flipped.len()
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let tiles = parse::<String>(input)?.take_while(|s| s != "").map(|s| parse_tile(s));

    println!(
        "The answer to part one is {:?}",
        count_flipped_tiles(tiles)
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
fn test_flipped_tiles() {
    let example = [
        "sesenwnenenewseeswwswswwnenewsewsw",
        "neeenesenwnwwswnenewnwwsewnenwseswesw",
        "seswneswswsenwwnwse",
        "nwnwneseeswswnenewneswwnewseswneseene",
        "swweswneswnenwsewnwneneseenw",
        "eesenwseswswnenwswnwnwsewwnwsene",
        "sewnenenenesenwsewnenwwwse",
        "wenwwweseeeweswwwnwwe",
        "wsweesenenewnwwnwsenewsenwwsesesenwne",
        "neeswseenwwswnwswswnw",
        "nenwswwsewswnenenewsenwsenwnesesenew",
        "enewnwewneswsewnwswenweswnenwsenwsw",
        "sweneswneswneneenwnewenewwneswswnese",
        "swwesenesewenwneswnwwneseswwne",
        "enesenwswwswneneswsenwnewswseenwsese",
        "wnwnesenesenenwwnenwsewesewsesesew",
        "nenewswnwewswnenesenwnesewesw",
        "eneswnwswnwsenenwnwnwwseeswneewsenese",
        "neswnwewnwnwseenwseesewsenwsweewe",
        "wseweeenwnesenwwwswnew",
    ].iter().map(|s| parse_tile(s.to_string()));

    assert_eq!(count_flipped_tiles(example), 10);
}

