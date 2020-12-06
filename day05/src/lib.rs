use aoc2020::parse;

use std::path::Path;
use thiserror::Error;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Seat{
    row: usize,
    column: usize
}

impl FromStr for Seat{
    type Err = String;

    fn from_str(s: &str) -> Result::<Self, Self::Err>{
        let mut s_iter = s.chars();

        let mut row_lower_bound = 0;
        let mut row_upper_bound = 127;

        while row_lower_bound < row_upper_bound {
            let adjustment = (row_upper_bound - row_lower_bound + 1) / 2;
            
            match s_iter.next().unwrap(){
                'F' => row_upper_bound -= adjustment,
                'B' => row_lower_bound += adjustment,
                _ => return Err("Invalid character for finding row".to_string())
            }
        }

        let mut column_lower_bound = 0;
        let mut column_upper_bound = 7;

        while column_lower_bound < column_upper_bound {
            let adjustment = (column_upper_bound - column_lower_bound + 1) / 2;
            
            match s_iter.next().unwrap(){
                'L' => column_upper_bound -= adjustment,
                'R' => column_lower_bound += adjustment,
                _ => return Err("Invalid character finding column".to_string())
            }
        }

        Ok(
            Seat{row: row_lower_bound, column: column_lower_bound}
        )
    }
}

impl Seat {
    fn id(&self) -> usize {
        &self.row * 8 + &self.column
    }
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let max_id = parse::<String>(input)?
                    .take_while(|s| s != "")
                    .map(|s| Seat::from_str(&s).unwrap().id())
                    .max()
                    .unwrap();

    println!("The answer to part one is {}", max_id);
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
fn test_seat_from_str() {
    assert_eq!(Seat::from_str("BFFFBBFRRR").unwrap(), Seat{row: 70, column: 7});
    assert_eq!(Seat::from_str("FFFBBBFRRR").unwrap(), Seat{row: 14, column: 7});
    assert_eq!(Seat::from_str("BBFFBBFRLL").unwrap(), Seat{row: 102, column: 4});
}

#[test]
fn test_seat_id() {
    assert_eq!(Seat{row: 70, column: 7}.id(), 567);
    assert_eq!(Seat{row: 14, column: 7}.id(), 119);
    assert_eq!(Seat{row: 102, column: 4}.id(), 820);
}