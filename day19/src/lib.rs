use aoc2020::parse_newline_sep;

use itertools::iproduct;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq)]
enum Rule {
    Subrule(Vec<usize>),
    Options(Box<Rule>, Box<Rule>),
    Literal(String),
    Repeat(Vec<Rule>),
    RepeatedPair(Vec<String>, Vec<String>, usize),
    Regex(String, usize), // Behaves as a literal, but needs to be told its min length
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
    fn expand(&self, rules: &HashMap<usize, Rule>) -> Vec<String> {
        match self {
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
            Rule::Repeat(_) => panic!("Cannot expand repeat rule"),
            Rule::RepeatedPair(_, _, _) => panic!("Cannot expand repeat pair rule"),
            Rule::Regex(_, _) => panic!("Cannot expand regex"),
        }
    }

    // Without fully expanding the rule, we can still guarantee a minimum length
    // of the ultimate expansion, which can be used by a termination condition
    // where there is infinite grammatical looping but a finite message length
    fn min_length(&self) -> usize {
        match &self {
            Rule::Subrule(r) => r.len(),
            Rule::Options(left, right) => left.min_length() + right.min_length(),
            Rule::Literal(r) => r.len(),
            Rule::Repeat(rs) => rs.iter().map(|r| r.min_length()).sum(),
            Rule::RepeatedPair(_, _, length) => *length,
            Rule::Regex(_, n) => *n,
        }
    }

    fn fully_expanded(&self) -> bool {
        match self {
            Rule::Literal(_) => true,
            Rule::Regex(_, _) => true,
            _ => false,
        }
    }

    fn to_string(&self) -> String {
        match self {
            Rule::Literal(s) => s.to_string(),
            Rule::Regex(s, _) => s.to_string(),
            _ => panic!("Not fully expanded"),
        }
    }

    // Replaces the rule with a set of sequences of rules that expand it
    fn expand_up_to_limit(
        &self,
        rules: &HashMap<usize, Rule>,
        limit: usize,
    ) -> Option<Vec<Vec<Rule>>> {
        match &self {
            Rule::Subrule(rs) => {
                let rules: Vec<Rule> = rs.iter().map(|r| rules[r].clone()).collect();
                if rules.iter().map(|r| r.min_length()).sum::<usize>() <= limit {
                    Some(vec![rules])
                } else {
                    None
                }
            }
            Rule::Options(left, right) => {
                let mut paths: Vec<Vec<Rule>> = (**left).expand_up_to_limit(rules, limit).unwrap();
                paths.append(&mut (**right).expand_up_to_limit(rules, limit).unwrap());

                let paths: Vec<Vec<Rule>> = paths
                    .into_iter()
                    .filter(|rs| rs.iter().map(|r| r.min_length()).sum::<usize>() <= limit)
                    .collect();

                if paths.len() > 0 {
                    Some(paths)
                } else {
                    None
                }
            }
            Rule::Literal(_) => Some(vec![vec![self.clone()]]),
            Rule::Repeat(rs) => {
                let expansions = fully_expand_rule_sequence(rules, rs.clone(), limit);

                if expansions.len() == 0 {
                    return None;
                }

                Some(
                    vec![vec![Rule::Regex(
                        "(".to_string()
                            + expansions
                                .into_iter()
                                .map(|rs| {
                                    rs.into_iter()
                                        .map(|r| r.to_string())
                                        .collect::<Vec<String>>()
                                        .concat()
                                })
                                .collect::<Vec<String>>()
                                .join("|")
                                .as_str()
                            + ")+",
                        1,
                    )]], // Obviously not the best min size we could calculate, but adequate for getting the result!
                )
            }
            Rule::RepeatedPair(first, second, length) => {
                let mut res = Vec::new();

                // Expands as far as the limit will allow
                for i in (1..).take_while(|i| i * length <= limit) {
                    res.push(vec![Rule::Regex(
                        format!(
                            "({}){{{}}}({}){{{}}}",
                            first.join("|"),
                            i,
                            second.join("|"),
                            i
                        ),
                        i * length,
                    )]);
                }

                if res.len() > 0 {
                    Some(res)
                } else {
                    None
                }
            }
            Rule::Regex(_, _) => Some(vec![vec![self.clone()]]),
        }
    }
}

// Used to expand the internals of a repeat
fn fully_expand_rule_sequence(
    rules: &HashMap<usize, Rule>,
    rs: Vec<Rule>,
    limit: usize,
) -> Vec<Vec<Rule>> {
    let expansions_per_rule: Vec<Vec<Vec<Rule>>> = rs
        .into_iter()
        .map(|r| {
            let mut expanded = vec![];
            let mut to_expand = vec![vec![r]];

            while to_expand.len() > 0 {
                let mut next_to_expand = Vec::new();
                for alternative in to_expand {
                    let (mut newly_expanded, mut still_to_expand) = alternative
                        .into_iter()
                        .fold(vec![vec![]], |base: Vec<Vec<Rule>>, next_rule| {
                            let next_expansions = next_rule
                                .expand_up_to_limit(rules, limit)
                                .unwrap_or(vec![vec![]])
                                .into_iter();

                            iproduct!(base.into_iter(), next_expansions)
                                .map(|(mut base, mut next)| {
                                    base.append(&mut next);
                                    base
                                })
                                .collect()
                        })
                        .iter()
                        .cloned()
                        .partition(|rs| rs.iter().all(|r| r.fully_expanded()));

                    expanded.append(&mut newly_expanded);
                    next_to_expand.append(&mut still_to_expand);
                }
                to_expand = next_to_expand
            }
            expanded
        })
        .collect();

    expansions_per_rule.into_iter().fold(
        vec![vec![]],
        |base_seqs: Vec<Vec<Rule>>, next_rule_seqs: Vec<Vec<Rule>>| {
            iproduct![base_seqs.into_iter(), next_rule_seqs.into_iter()]
                .map(|(mut base, mut next)| {
                    base.append(&mut next);
                    base
                })
                .collect()
        },
    )
}

fn parse_rules<'a>(rules: Vec<String>) -> HashMap<usize, Rule> {
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

fn matching_rules_substituted(rules: &mut HashMap<usize, Rule>, messages: Vec<String>) -> usize {
    rules.insert(8, Rule::Repeat(vec![rules[&42].clone()]));

    let repeat_left = rules[&42].expand(&rules);
    let repeat_right = rules[&31].expand(&rules);
    let length = repeat_left.iter().map(|r| r.len()).min().unwrap()
        + repeat_right.iter().map(|r| r.len()).min().unwrap();

    rules.insert(11, Rule::RepeatedPair(repeat_left, repeat_right, length));

    let mut expansions = vec![vec![rules[&0].clone()]];
    let mut regex_expansions: HashSet<String> = HashSet::new();
    let max_length = messages.iter().map(|m| m.len()).max().unwrap_or(0);

    while expansions.len() > 0 {
        let next = expansions.pop().unwrap();

        if next.iter().all(|r| r.fully_expanded()) {
            let string_rep = next
                .iter()
                .map(|s| match s {
                    Rule::Literal(cs) => cs.to_string(),
                    Rule::Repeat(rs) => {
                        "(".to_string()
                            + &rs.iter().fold("".to_string(), |s, r| match r {
                                Rule::Literal(cs) => s + cs,
                                Rule::Regex(r, _) => s + r,
                                _ => panic!(),
                            })
                            + ")+"
                    }
                    Rule::Regex(r, _) => r.to_string(),
                    _ => panic!("Rule has not been fully expanded"),
                })
                .collect::<String>();

            regex_expansions.insert("^".to_string() + &string_rep + "$");
        } else {
            let (idx, rule) = next
                .iter()
                .enumerate()
                .filter(|(_, r)| !r.fully_expanded())
                .next()
                .unwrap();

            for rs in rule
                .expand_up_to_limit(&rules, max_length)
                .unwrap_or(Vec::new())
            {
                let mut rule_sequence = next.clone();
                rule_sequence.remove(idx);
                for r in rs.iter().rev() {
                    rule_sequence.insert(idx, r.clone());
                }
                expansions.push(rule_sequence);
            }
        }
    }

    let regexps: Vec<Regex> = regex_expansions
        .iter()
        .map(|s| Regex::new(s).unwrap())
        .collect();

    messages
        .iter()
        .filter(|m| regexps.iter().any(|re| re.is_match(m)))
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

pub fn part2(input: &Path) -> Result<(), Error> {
    let mut split_input = parse_newline_sep::<String>(input)?
        .map(|chunk| chunk.split('\n').map(|split| split.to_string()).collect());

    let mut rules = parse_rules(split_input.next().unwrap());
    let messages = split_input.next().unwrap();

    println!(
        "The answer to part two is {}",
        matching_rules_substituted(&mut rules, messages)
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

#[test]
fn test_matching_rules_substituted() {
    let mut example_rules = parse_rules(vec![
        "42: 9 14 | 10 1".to_string(),
        "9: 14 27 | 1 26".to_string(),
        "10: 23 14 | 28 1".to_string(),
        "1: \"a\"".to_string(),
        "11: 42 31".to_string(),
        "5: 1 14 | 15 1".to_string(),
        "19: 14 1 | 14 14".to_string(),
        "12: 24 14 | 19 1".to_string(),
        "16: 15 1 | 14 14".to_string(),
        "31: 14 17 | 1 13".to_string(),
        "6: 14 14 | 1 14".to_string(),
        "2: 1 24 | 14 4".to_string(),
        "0: 8 11".to_string(),
        "13: 14 3 | 1 12".to_string(),
        "15: 1 | 14".to_string(),
        "17: 14 2 | 1 7".to_string(),
        "23: 25 1 | 22 14".to_string(),
        "28: 16 1".to_string(),
        "4: 1 1".to_string(),
        "20: 14 14 | 1 15".to_string(),
        "3: 5 14 | 16 1".to_string(),
        "27: 1 6 | 14 18".to_string(),
        "14: \"b\"".to_string(),
        "21: 14 1 | 1 14".to_string(),
        "25: 1 1 | 1 14".to_string(),
        "22: 14 14".to_string(),
        "8: 42".to_string(),
        "26: 14 22 | 1 20".to_string(),
        "18: 15 15".to_string(),
        "7: 14 5 | 1 21".to_string(),
        "24: 14 1".to_string(),
    ]);

    let example_messages = vec![
        "abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa".to_string(),
        "bbabbbbaabaabba".to_string(),
        "babbbbaabbbbbabbbbbbaabaaabaaa".to_string(),
        "aaabbbbbbaaaabaababaabababbabaaabbababababaaa".to_string(),
        "bbbbbbbaaaabbbbaaabbabaaa".to_string(),
        "bbbababbbbaaaaaaaabbababaaababaabab".to_string(),
        "ababaaaaaabaaab".to_string(),
        "ababaaaaabbbaba".to_string(),
        "baabbaaaabbaaaababbaababb".to_string(),
        "abbbbabbbbaaaababbbbbbaaaababb".to_string(),
        "aaaaabbaabaaaaababaa".to_string(),
        "aaaabbaaaabbaaa".to_string(),
        "aaaabbaabbaaaaaaabbbabbbaaabbaabaaa".to_string(),
        "babaaabbbaaabaababbaabababaaab".to_string(),
        "aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba".to_string(),
    ];

    assert_eq!(
        matching_rules_substituted(&mut example_rules, example_messages),
        12
    );
}

#[test]
fn test_expand_up_to_limt() {
    let mut rules = HashMap::<usize, Rule>::new();
    rules.insert(1, Rule::Literal("a".to_string()));
    rules.insert(2, Rule::Literal("b".to_string()));
    rules.insert(3, Rule::Literal("c".to_string()));

    let example = Rule::Subrule(vec![1, 2, 3]);
    assert_eq!(
        example.expand_up_to_limit(&rules, 5),
        Some(vec![vec![
            Rule::Literal("a".to_string()),
            Rule::Literal("b".to_string()),
            Rule::Literal("c".to_string())
        ]])
    );
    assert_eq!(example.expand_up_to_limit(&rules, 2), None);

    let example = Rule::RepeatedPair(
        vec!["hello".to_string(), "world".to_string()],
        vec!["merry".to_string(), "x-mas".to_string()],
        10,
    );
    assert_eq!(
        example.expand_up_to_limit(&rules, 20),
        Some(vec![
            vec![Rule::Regex(
                "(hello|world){1}(merry|x-mas){1}".to_string(),
                10
            )],
            vec![Rule::Regex(
                "(hello|world){2}(merry|x-mas){2}".to_string(),
                20
            )]
        ])
    );
    assert_eq!(example.expand_up_to_limit(&rules, 5), None);
}
