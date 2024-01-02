use crate::custom_error::AocError;
use std::str::FromStr;
use itertools::Itertools;

#[derive(Debug, PartialEq)]
struct Data {
    nums: Vec<usize>,
}

impl FromStr for Data {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums = s.split_whitespace()
            .into_iter()
            .skip(1)
            .map(|n| n.parse::<usize>().unwrap())
            .collect_vec();
        Ok(Data { nums })
    }
}


#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<usize, AocError> {
    let mut line_iterator = input.lines();
    let race_times = line_iterator.next().unwrap().parse::<Data>().unwrap().nums;
    let record_distance = line_iterator.next().unwrap().parse::<Data>().unwrap().nums;
    let result = race_times.iter().enumerate()
        .map(|(i, time)| {
            let mut ways_to_win = 0;
            let previous_record = record_distance[i];
            for j in 0..*time {
                if j * (time - j) > previous_record {
                    ways_to_win += 1
                }
            }
            return ways_to_win
        })
        .fold(1, |acc, x| acc * x);

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test.txt");

        assert_eq!(288, process(input)?);
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
