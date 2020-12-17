use aoc2020::parse_newline_sep;

use lazy_static::lazy_static;
use regex::Regex;
use std::path::Path;
use std::str::FromStr;
use thiserror::Error;

lazy_static! {
    static ref FIELD: Regex = Regex::new(r"^[A-Za-z ]+: (\d+)\-(\d+) or (\d+)\-(\d+)$").unwrap();
}

struct TicketField {
    low_range: std::ops::RangeInclusive<usize>,
    high_range: std::ops::RangeInclusive<usize>,
}

impl TicketField {
    fn valid_for(&self, val: &usize) -> bool {
        self.low_range.contains(val) || self.high_range.contains(val)
    }
}

impl FromStr for TicketField {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = FIELD.captures(s).expect("Could not match entry");
        let capture_to_usize = |n| captures.get(n).unwrap().as_str().parse::<usize>().unwrap();

        Ok(TicketField {
            low_range: capture_to_usize(1)..=capture_to_usize(2),
            high_range: capture_to_usize(3)..=capture_to_usize(4),
        })
    }
}

fn ticket_field_validity_rate(fields: Vec<TicketField>, tickets: Vec<Vec<usize>>) -> usize {
    tickets
        .iter()
        .flat_map(|t| t.iter())
        .filter(|val| fields.iter().all(|f| !f.valid_for(val)))
        .sum()
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let chunks: Vec<Vec<String>> = parse_newline_sep::<String>(input)?
        .map(|chunk| chunk.split('\n').map(|split| split.to_string()).collect())
        .collect();

    let fields = chunks[0]
        .iter()
        .filter(|s| *s != "")
        .map(|line| line.parse::<TicketField>().unwrap())
        .collect();
    let tickets = chunks[2]
        .iter()
        .filter(|s| *s != "")
        .skip(1)
        .map(|ticket| {
            ticket
                .split(',')
                .map(|val| val.parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    println!(
        "The answer to part one is {}",
        ticket_field_validity_rate(fields, tickets)
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
fn test_ticket_field_validity_rate() {
    let example_fields = [
        "class: 1-3 or 5-7",
        "row: 6-11 or 33-44",
        "seat: 13-40 or 45-50",
    ]
    .iter()
    .map(|s| TicketField::from_str(s).unwrap())
    .collect();

    let example_tickets = vec![
        vec![7, 3, 47],
        vec![40, 4, 50],
        vec![55, 2, 20],
        vec![38, 6, 12],
    ];

    assert_eq!(
        ticket_field_validity_rate(example_fields, example_tickets),
        71
    );
}
