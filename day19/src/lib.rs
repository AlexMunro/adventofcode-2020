use aoc2020::parse_newline_sep;

use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug)]
enum Rule {
    Subrule(Vec<usize>),
    Options(Box<Rule>, Box<Rule>),
    Literal(String),
}

impl FromStr for Rule {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("\"") {
            Ok(Rule::Literal(s.chars().filter(|c| *c != '"').collect()))
        } else if s.contains('|') {
            let mut rules = s.split(" | ");
            Ok(Rule::Options(
                Box::new(rules.next().unwrap().parse().unwrap()),
                Box::new(rules.next().unwrap().parse().unwrap()),
            ))
        } else {
            Ok(Rule::Subrule(
                s.split_whitespace()
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect(),
            ))
        }
    }
}

impl Rule {
    fn expand(
        &self,
        rules: &HashMap<usize, Rule>,
    ) -> Vec<String> {
        match &self {
            Rule::Subrule(rule_sequence) => rule_sequence
                .iter()
                .map(|id| rules.get(id).unwrap())
                .fold(vec!["".to_string()], |expansions, next_rule| {
                    let next_expansions = next_rule.expand(rules);

                    expansions
                        .iter()
                        .flat_map(|expansion| {
                            next_expansions.iter().map(move |next_exp| {
                                expansion
                                    .to_owned()
                                    .chars()
                                    .chain(next_exp.to_owned().chars())
                                    .collect::<String>()
                            })
                        })
                        .collect()
                }),
            Rule::Options(left, right) => {
                let mut expansions = (*left).expand(rules);
                expansions.append(&mut (*right).expand(rules));
                expansions
            }
            Rule::Literal(s) => vec![s.to_string()],
        }
    }
}

fn parse_rules(rules: Vec<String>) -> HashMap<usize, Rule> {
    rules
        .iter()
        .filter(|s| *s != "")
        .map(|rule| {
            let mut chunks = rule.split(": ");

            (
                chunks.next().unwrap().parse().unwrap(),
                chunks.next().unwrap().parse().unwrap(),
            )
        })
        .collect()
}

fn matching_rules(rules: HashMap<usize, Rule>, messages: Vec<String>) -> usize {
    let valid_strings = rules[&0].expand(&rules);

    let valid_strings: HashSet<&String> = valid_strings.iter().collect();

    messages
        .iter()
        .filter(|m| valid_strings.contains(m))
        .count()
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let mut split_input = parse_newline_sep::<String>(input)?
        .map(|chunk| chunk.split('\n').map(|split| split.to_string()).collect());

    let rules = parse_rules(split_input.next().unwrap());
    let messages = split_input.next().unwrap();

    println!(
        "The answer to part one is {}",
        matching_rules(rules, messages)
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
fn test_matching_rules() {
    let example_rules = parse_rules(vec![
        "0: 4 1 5".to_string(),
        "1: 2 3 | 3 2".to_string(),
        "2: 4 4 | 5 5".to_string(),
        "3: 4 5 | 5 4".to_string(),
        "4: \"a\"".to_string(),
        "5: \"b\"".to_string(),
    ]);

    let example_messages = vec![
        "ababbb".to_string(),
        "bababa".to_string(),
        "abbbab".to_string(),
        "aaabbb".to_string(),
        "aaaabbb".to_string(),
    ];

    assert_eq!(matching_rules(example_rules, example_messages), 2);
}
