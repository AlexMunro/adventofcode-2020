use aoc2020::parse_newline_sep;

use std::collections::{HashSet, VecDeque};
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

fn recursive_combat(player_1: Vec<usize>, player_2: Vec<usize>) -> usize {
    let player_1 = VecDeque::from(player_1);
    let player_2 = VecDeque::from(player_2);

    fn player1_wins_subgame(
        mut player_1: VecDeque<usize>,
        mut player_2: VecDeque<usize>,
    ) -> (bool, usize) {
        let mut previously_played: HashSet<String> = HashSet::new();

        while !player_1.is_empty() && !player_2.is_empty() {
            if previously_played.contains(&format!("{:?} - {:?}", &player_1, &player_2)) {
                return (true, deck_score(&player_1));
            }

            previously_played.insert(format!("{:?} - {:?}", &player_1, &player_2));

            let card_1 = player_1.pop_front().unwrap();
            let card_2 = player_2.pop_front().unwrap();

            let player_1_won_round = if player_1.len() >= card_1 && player_2.len() >= card_2 {
                player1_wins_subgame(
                    player_1.clone().into_iter().take(card_1).collect(),
                    player_2.clone().into_iter().take(card_2).collect(),
                )
                .0
            } else {
                card_1 > card_2
            };

            if player_1_won_round {
                player_1.push_back(card_1);
                player_1.push_back(card_2);
            } else {
                player_2.push_back(card_2);
                player_2.push_back(card_1);
            }
        }
        if player_2.is_empty() {
            (true, deck_score(&player_1))
        } else {
            (false, deck_score(&player_2))
        }
    }

    player1_wins_subgame(player_1, player_2).1
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

pub fn part2(input: &Path) -> Result<(), Error> {
    let mut decks = parse_newline_sep::<String>(input)?.map(|s| {
        s.split("\n")
            .filter_map(|l| l.parse::<usize>().ok())
            .collect::<Vec<usize>>()
    });

    println!(
        "The answer to part two is {:?}",
        recursive_combat(decks.next().unwrap(), decks.next().unwrap())
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
fn test_combat() {
    let example_player_1 = vec![9, 2, 6, 3, 1];

    let example_player_2 = vec![5, 8, 4, 7, 10];

    assert_eq!(combat(example_player_1, example_player_2), 306);
}

#[test]
fn test_recursive_combat() {
    let example_player_1 = vec![9, 2, 6, 3, 1];

    let example_player_2 = vec![5, 8, 4, 7, 10];

    assert_eq!(recursive_combat(example_player_1, example_player_2), 291);
}
