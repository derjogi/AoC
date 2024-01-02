use crate::custom_error::AocError;
use std::str::FromStr;
use itertools::Itertools;

#[derive(Debug, PartialEq)]
struct Data {
    num: usize,
}

impl FromStr for Data {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let num = s.split_whitespace()
            .dropping(1)
            .join("")
            .parse::<usize>().unwrap();
        Ok(Data { num })
    }
}


#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<usize, AocError> {
    let mut line_iterator = input.lines();
    let time = line_iterator.next().unwrap().parse::<Data>().unwrap().num;
    let previous_record = line_iterator.next().unwrap().parse::<Data>().unwrap().num;

    let mut ways_to_win = 0;
    for j in 0..time {
        if j * (time - j) > previous_record {
            ways_to_win += 1
        }
    }
    Ok(ways_to_win)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test.txt");

        assert_eq!(71503, process(input)?);
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
