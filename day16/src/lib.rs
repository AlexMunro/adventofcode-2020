use aoc2020::parse_newline_sep;

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::str::FromStr;
use thiserror::Error;

lazy_static! {
    static ref FIELD: Regex = Regex::new(r"^([A-Za-z ]+): (\d+)\-(\d+) or (\d+)\-(\d+)$").unwrap();
}

#[derive(PartialEq, Eq, Hash)]
struct TicketField {
    name: String,
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
            name: captures.get(1).unwrap().as_str().to_string(),
            low_range: capture_to_usize(2)..=capture_to_usize(3),
            high_range: capture_to_usize(4)..=capture_to_usize(5),
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

fn departure_fields(
    fields: Vec<TicketField>,
    own_ticket: Vec<usize>,
    tickets: Vec<Vec<usize>>,
) -> HashMap<String, usize> {
    let valid_tickets: Vec<&Vec<usize>> = tickets
        .iter()
        .filter(|ticket| {
            ticket
                .iter()
                .all(|val| fields.iter().any(|f| f.valid_for(val)))
        })
        .collect();

    println!("{:?}", valid_tickets);

    let mut remaining_fields: HashSet<&TicketField> = fields.iter().collect();

    let mut field_indexes = HashMap::<usize, &String>::new();

    while fields.len() != field_indexes.len() {
        let next_assignment = {
            (0..fields.len())
                .filter(|i| !field_indexes.contains_key(i))
                .filter_map(|i| {
                    let options: Vec<&TicketField> = remaining_fields
                        .iter()
                        .filter(|field| valid_tickets.iter().all(|t| field.valid_for(&(*t)[i])))
                        .cloned()
                        .collect();

                    if options.len() == 1 {
                        Some((i, options[0]))
                    } else {
                        None
                    }
                })
                .next()
                .unwrap()
        };

        remaining_fields.remove(&next_assignment.1);
        field_indexes.insert(next_assignment.0, &next_assignment.1.name);
    }

    field_indexes
        .iter()
        .map(|(k, v)| (v.to_string(), own_ticket[*k]))
        .collect()
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

pub fn part2(input: &Path) -> Result<(), Error> {
    let chunks: Vec<Vec<String>> = parse_newline_sep::<String>(input)?
        .map(|chunk| chunk.split('\n').map(|split| split.to_string()).collect())
        .collect();

    let fields = chunks[0]
        .iter()
        .filter(|s| *s != "")
        .map(|line| line.parse::<TicketField>().unwrap())
        .collect();

    let own_ticket: Vec<usize> = chunks[1]
        .iter()
        .filter(|s| *s != "")
        .skip(1)
        .map(|ticket| {
            ticket
                .split(',')
                .map(|val| val.parse::<usize>().unwrap())
                .collect()
        })
        .next()
        .unwrap();

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
        "The answer to part two is {}",
        departure_fields(fields, own_ticket, tickets)
            .iter()
            .filter(|(k, _)| k.starts_with("departure"))
            .map(|(_, v)| *v)
            .product::<usize>()
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

#[test]
fn test_departure_fields() {
    let example_fields = [
        "class: 0-1 or 4-19",
        "row: 0-5 or 8-19",
        "seat: 0-13 or 16-19",
    ]
    .iter()
    .map(|s| TicketField::from_str(s).unwrap())
    .collect();

    let example_own_ticket = vec![11, 12, 13];

    let example_tickets = vec![vec![3, 9, 18], vec![15, 1, 5], vec![5, 14, 9]];

    let result = departure_fields(example_fields, example_own_ticket, example_tickets);
    assert_eq!(*result.get("class").unwrap(), 12);
    assert_eq!(*result.get("row").unwrap(), 11);
    assert_eq!(*result.get("seat").unwrap(), 13);
}
