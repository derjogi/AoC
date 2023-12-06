use itertools::Itertools;
use crate::custom_error::AocError;
use nom::{character::complete::char, ToUsize};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<usize, AocError> {
    let spelled_out = vec![
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9"
    ];

    let output = input
        .lines()
        .map(|line| {

            let first_match = spelled_out.iter().enumerate().map(|(word_index, word)| {
                match line.find(word) {
                  None => None,
                  Some(index) => Some((word_index, index)),
                }
              }).filter_map(|x| x).sorted_by(|(a_idx, a_pos), (b_idx, b_pos)| a_pos.cmp(&b_pos))
                .next();

            // result should be (8, 10) for finding 'eight' (the 8th word) at index 10 in line

            let last_match = spelled_out.iter().enumerate().map(|(word_index, word)| {
                match line.rfind(word) {
                    None => None,
                    Some(index) => Some((word_index, index)),
                }
            }).filter_map(|x| x).sorted_by(|(a_idx, a_pos), (b_idx, b_pos)| b_pos.cmp(&a_pos))
                .next();

            // result should be (9, 21) for finding 'nine' (the 9th word) at index 21 in line

            let first = first_match.unwrap_or((0,0)).0%10;
            let last = last_match.unwrap_or((0,0)).0%10;
            let result = 10 * first + last;
            println!("{result}");
            result
        })
        .sum::<usize>();

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "
two1nine
4nineeightseven2
zoneight234
nine2onelpzhtrbninexkgtjqg
eightwothree
abcone2threexyz
xtwone3four
7pqrstsixteen";
        assert_eq!(281+99, process(input)?);
        Ok(())
    }

    #[test]
    fn test_real_data_higher_than() -> miette::Result<()> {
        let file = include_str!("../input2.txt");
        let result = process(file)?;
        println!("{result}");
        assert!(55882 < result);
        Ok(())
    }
}
