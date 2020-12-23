use aoc2020::parse;

use std::collections::HashMap;
use std::path::Path;
use thiserror::Error;

fn evaluate_simple_expression(expression: String) -> usize {
    let mut tokens = expression.split_whitespace();

    let first = tokens.next().unwrap().parse::<usize>().unwrap();
    let mut prev_operator = "";

    tokens.fold(first, |acc, next| {
        match next {
            "+" => {
                prev_operator = "+";
                acc
            },
            "*" => {
                prev_operator = "*";
                acc
            },
            val => {
                match prev_operator {
                    "+" => acc + val.parse::<usize>().unwrap(),
                    "*" => acc * val.parse::<usize>().unwrap(),
                    _ => panic!("Unrecognised operator")
                }
            }
        }
    })
}

#[derive(Debug)]
struct Subexpression {
    result: usize,
    length: usize,    
}

fn simplify_and_evalute(expression: &String, start: usize, end: usize, subexpressions: &HashMap::<usize, Subexpression>) -> usize {
    let mut i = start;
    let mut simplified = String::new();
    let mut substr_iter = expression.chars().skip(start);

    while i <= end {
        match substr_iter.next() {
            Some('(') => {
                simplified += subexpressions[&i].result.to_string().as_str();
                for _ in 0..(subexpressions[&i].length - 1) {
                    substr_iter.next();
                }
               i += subexpressions[&i].length;
            },
            Some(c) => {
                i +=1;
                simplified.push(c);
            },
            None => {
                panic!("Inaccessible");
            }
        }
    }
    evaluate_simple_expression(simplified)
}

fn evaluate_expression(expression: String) -> usize {
    let mut subexpressions = HashMap::<usize, Subexpression>::new();
    let mut open_stack: Vec<usize> = vec![];

    for (n, c) in expression.chars().enumerate() {
        match c {
            '(' => {
                open_stack.push(n)
            },
            ')' => {
                let open_pos = open_stack.pop().unwrap();
                subexpressions.insert(
                    open_pos,
                    Subexpression{
                        result: simplify_and_evalute(&expression, open_pos + 1, n - 1, &subexpressions),
                        length: n - open_pos + 1,
                    }
                );
            },
            _ => (),
        }
    }

    simplify_and_evalute(&expression, 0, expression.len() - 1, &subexpressions)
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let sum: usize = parse(input)?.take_while(|s| *s != "").map(|expression| evaluate_expression(expression)).sum();

    println!("The answer to part one is {}", sum);
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
fn test_evaluate_expression() {
    let example = "1 + 2 + 3".to_string();
    assert_eq!(evaluate_expression(example), 6);

    let example = "1 * 2 + 5".to_string();
    assert_eq!(evaluate_expression(example), 7);

    let example = "10 * 20 + 4".to_string();
    assert_eq!(evaluate_expression(example), 204);

    let example = "2 * 3 + (4 * 5)".to_string();
    assert_eq!(evaluate_expression(example), 26);

    let example = "5 + (8 * 3 + 9 + 3 * 4 * 3)".to_string();
    assert_eq!(evaluate_expression(example), 437);

    let example = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".to_string();
    assert_eq!(evaluate_expression(example), 12240);
    
    let example = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".to_string();
    assert_eq!(evaluate_expression(example), 13632);
}

#[test]
fn test_evaluate_simple_expression() {
    let example = "1 + 2 + 3".to_string();
    assert_eq!(evaluate_simple_expression(example), 6);

    let example = "1 * 2 + 5".to_string();
    assert_eq!(evaluate_simple_expression(example), 7);
}