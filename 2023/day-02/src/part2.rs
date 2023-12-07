use std::cmp::{max, min};
use std::fmt::format;
use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<usize, AocError> {
    let result = input
        .lines()
        .map(|line| {
            let mut min_red = 0;
            let mut min_green = 0;
            let mut min_blue = 0;
            let (_game, results) = line.split_once(":").unwrap();
            results.split(";")
                .for_each(|pick| {
                    pick.split(",")
                        .for_each(|color| {
                            match color.trim().split_once(" ").unwrap() {
                                (amnt, "red") => min_red = max(min_red, amnt.parse::<usize>().unwrap()),
                                (amnt, "green") => min_green = max(min_green, amnt.parse::<usize>().unwrap()),
                                (amnt, "blue") => min_blue = max(min_blue, amnt.parse::<usize>().unwrap()),
                                _ => ()
                            }
                        });
                });
            min_red * min_green * min_blue
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
        assert_eq!(2286, process(input)?);
        Ok(())
    }
}
