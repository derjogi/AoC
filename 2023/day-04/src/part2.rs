use std::collections::HashMap;
use std::ops::Deref;
use crate::custom_error::AocError;
use itertools::Itertools;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<usize, AocError> {
    let num_games = input.lines().count();
    let mut num_cards_per_game = vec![1; num_games];

    input.lines().for_each(|line| {
        let (game, nums) = line.split_once(": ").unwrap();
        let game_num = game.split_once(" ").unwrap().1.trim().parse::<usize>().unwrap()-1;
        let (winning, actual) = nums.split_once(" | ").unwrap();
        let winning_numbers: Vec<usize> = winning.split_whitespace().map(|num| num.parse::<usize>().unwrap()).collect();
        let actual_numbers: Vec<usize> = actual.split_whitespace().map(|num| num.parse::<usize>().unwrap()).collect();
        let winners: Vec<&usize> = winning_numbers.iter().filter(|win| actual_numbers.contains(win)).collect();
        let wins = winners.len();

        for i in 1..wins+1 {
            let num_new_cards = num_cards_per_game[game_num];
            let old = num_cards_per_game[game_num+i];
            num_cards_per_game[game_num+i] = old + num_new_cards;
        }
    });
    let result = num_cards_per_game.iter().sum();

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test.txt");
        assert_eq!(30, process(input)?);
        Ok(())
    }

    #[test]
    fn real_process() -> miette::Result<()> {
        let input = include_str!("../input.txt");
        let result = process(input)?;
        println!("Result: {result}");
        Ok(())
    }
}
