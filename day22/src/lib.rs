use aoc2020::parse_newline_sep;

use std::collections::VecDeque;
use std::path::Path;
use thiserror::Error;

fn deck_score(deck: &VecDeque<usize>) -> usize {
    deck.iter()
        .rev()
        .enumerate()
        .map(|(n, card)| (n + 1) * card)
        .sum()
}

fn combat(player_1: Vec<usize>, player_2: Vec<usize>) -> usize {
    let mut player_1 = VecDeque::from(player_1);
    let mut player_2 = VecDeque::from(player_2);

    while !player_1.is_empty() && !player_2.is_empty() {
        let card_1 = player_1.pop_front().unwrap();
        let card_2 = player_2.pop_front().unwrap();

        if card_1 > card_2 {
            player_1.push_back(card_1);
            player_1.push_back(card_2);
        } else {
            player_2.push_back(card_2);
            player_2.push_back(card_1);
        }
    }
    if player_1.is_empty() {
        deck_score(&player_2)
    } else {
        deck_score(&player_1)
    }
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let mut decks = parse_newline_sep::<String>(input)?.map(|s| {
        s.split("\n")
            .filter_map(|l| l.parse::<usize>().ok())
            .collect::<Vec<usize>>()
    });

    println!(
        "The answer to part one is {:?}",
        combat(decks.next().unwrap(), decks.next().unwrap())
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
fn test_combat() {
    let example_player_1 = vec![9, 2, 6, 3, 1];

    let example_player_2 = vec![5, 8, 4, 7, 10];

    assert_eq!(combat(example_player_1, example_player_2), 306);
}
