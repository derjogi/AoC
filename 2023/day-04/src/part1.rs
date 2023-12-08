use crate::custom_error::AocError;
use itertools::Itertools;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<usize, AocError> {
    let result = input.lines()
        .map(|line| line.split_once(": ").unwrap().1.split_once(" | ").unwrap())
        .map(|(winning, actual)| {
            let winning_numbers: Vec<usize> = winning.split_whitespace().map(|num| num.parse::<usize>().unwrap()).collect();
            let actual_numbers: Vec<usize> = actual.split_whitespace().map(|num| num.parse::<usize>().unwrap()).collect();
            let winners: Vec<&usize> = winning_numbers.iter().filter(|win| actual_numbers.contains(win)).collect();

            let count = winners.len();
            if count == 0 { 0 } else { usize::pow(2, (count - 1) as u32) }
        }).sum();
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test.txt");
        assert_eq!(13, process(input)?);
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
