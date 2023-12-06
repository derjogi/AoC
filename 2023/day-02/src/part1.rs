use std::fmt::format;
use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<usize, AocError> {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let result = input
        .lines()
        .map(|line| {
            // let mut sum_red = 0;
            // let mut sum_green = 0;
            // let mut sum_blue = 0;
            let (game, results) = line.split_once(":").unwrap();
            let game_is_valid = results.split(";")
                .map(|pick| {
                    let picks_are_valid = pick.split(",")
                        .map(|color| {
                            match color.trim().split_once(" ").unwrap() {
                                (amnt, "red") => amnt.parse::<u8>().unwrap() <= max_red,
                                (amnt, "green") => amnt.parse::<u8>().unwrap() <= max_green,
                                (amnt, "blue") => amnt.parse::<u8>().unwrap() <= max_blue,
                                _ => false
                            }
                        })
                        .all(|x| x);
                    picks_are_valid
                }).all(|x| x);
            // if sum_red > max_red || sum_green > max_green || sum_blue > max_blue {
            if game_is_valid {
                game.split_once(" ").unwrap().1.parse().unwrap()
            } else {
                0
            }
        })
        .sum();
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test1.txt");
        assert_eq!(8, process(input)?);
        Ok(())
    }
}
