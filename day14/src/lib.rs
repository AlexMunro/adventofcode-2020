use aoc2020::parse;

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::path::Path;
use thiserror::Error;

lazy_static! {
    static ref BITMASK: Regex = Regex::new(r"^mask = ([01X]+)$").unwrap();
    static ref VALUE: Regex = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();
}

fn execute_bitmask_program<'a>(program: Vec<String>) -> u64 {
    let mut memory = HashMap::new();
    let mut mask: Vec<(usize, char)> = vec![];
    for instr in program {
        if BITMASK.is_match(&instr) {
            mask = BITMASK
                .captures(&instr)
                .unwrap()
                .get(1)
                .expect("Could not parse mask")
                .as_str()
                .chars()
                .rev()
                .enumerate()
                .filter(|(_, c)| *c != 'X')
                .collect();
        } else {
            let captures = VALUE.captures(&instr).expect("Could not parse instruction");
            let index = captures
                .get(1)
                .expect("String parse error")
                .as_str()
                .parse::<u64>()
                .expect("Int parse error");
            let unmasked = captures
                .get(2)
                .expect("String parse error")
                .as_str()
                .parse::<u64>()
                .expect("Int parse error");
            memory.insert(
                index,
                mask.iter().fold(unmasked, |acc, (index, c)| {
                    if *c == '1' {
                        acc | (2 as u64).pow(*index as u32)
                    } else {
                        acc & !(2 as u64).pow(*index as u32)
                    }
                }),
            );
        }
    }
    memory.values().sum::<u64>()
}

fn execute_floating_bitmask_program(program: Vec<String>) -> u64 {
    let mut memory = HashMap::new();
    let mut mask: Vec<(usize, char)> = vec![];

    for instr in program {
        if BITMASK.is_match(&instr) {
            mask = BITMASK
                .captures(&instr)
                .unwrap()
                .get(1)
                .expect("Could not parse mask")
                .as_str()
                .chars()
                .rev()
                .enumerate()
                .collect();
        } else {
            let captures = VALUE.captures(&instr).expect("Could not parse instruction");
            let index = captures
                .get(1)
                .expect("String parse error")
                .as_str()
                .parse::<u64>()
                .expect("Int parse error");
            let unmasked = captures
                .get(2)
                .expect("String parse error")
                .as_str()
                .parse::<u64>()
                .expect("Int parse error");

            let mut floating_vals = vec![];

            let base_masked = mask.iter().fold(index, |acc, (index, c)| {
                if *c == '1' {
                    acc | (2 as u64).pow(*index as u32)
                } else {
                    if *c == 'X' {
                        floating_vals.push((2 as u64).pow(*index as u32) as u64);
                    }
                    acc
                }
            });

            let mut addresses: Vec<u64> = vec![base_masked];

            for f in floating_vals {
                for a in addresses.to_owned() {
                    if a | f == a {
                        &addresses.push(a & !f);
                    } else {
                        &addresses.push(a | f);
                    }
                }
            }

            for a in addresses {
                memory.insert(a, unmasked);
            }
        }
    }

    memory.values().sum::<u64>()
}

pub fn part1(input: &Path) -> Result<(), Error> {
    println!(
        "The answer to part one is {}",
        execute_bitmask_program(parse(input)?.take_while(|s| s != "").collect())
    );
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    println!(
        "The answer to part two is {}",
        execute_floating_bitmask_program(parse(input)?.take_while(|s| s != "").collect())
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
fn test_execute_bitmask_program() {
    let example = [
        "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string(),
        "mem[8] = 11".to_string(),
        "mem[7] = 101".to_string(),
        "mem[8] = 0".to_string(),
    ]
    .to_vec();

    assert_eq!(execute_bitmask_program(example), 165 as u64);
}

#[test]
fn test_execute_floating_bitmask_program() {
    let example = [
        "mask = 000000000000000000000000000000X1001X".to_string(),
        "mem[42] = 100".to_string(),
        "mask = 00000000000000000000000000000000X0XX".to_string(),
        "mem[26] = 1".to_string(),
    ]
    .to_vec();

    assert_eq!(execute_floating_bitmask_program(example), 208 as u64);
}
