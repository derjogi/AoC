use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::HashMap;
use nom::AsChar;
use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<usize, AocError> {

    let scores = HashMap::from([
        ('A', 13),
        ('K', 12),
        ('Q', 11),
        ('J', 10),
        ('T', 9),
        ('9', 8),
        ('8', 7),
        ('7', 6),
        ('6', 5),
        ('5', 4),
        ('4', 3),
        ('3', 2),
        ('2', 1)
    ]);

    let mut hands_and_bids: Vec<(&str, &str)> = input.lines().map(|line| line.split_once(" ").unwrap()).collect();
    hands_and_bids.sort_by(|set1, set2| {
        let hand1 = set1.0;
        let hand2 = set2.0;
        let chars1: Vec<char> = hand1.chars().collect();
        let chars2: Vec<char> = hand2.chars().collect();
        let mut counts1: Vec<usize> = chars1.iter()
            .map(|c| hand1.matches(c.as_char()).count())
            .collect();
        let mut counts2: Vec<usize> = chars2.iter()
            .map(|c| hand2.matches(c.as_char()).count())
            .collect();
        counts1.sort();
        counts1.reverse();
        counts2.sort();
        counts2.reverse();

        if counts1[0] > counts2[0] {
            return Greater
        } else if counts1[0] < counts2[0] {
            return Less
        }

        // Both have the same number of highest.
        // There are two special cases: full house (3+2) trumps triplet (3+1)
        // Two pairs (2+2) trump one pair (2+1);
        // check for those first, after that we can just compare score by position:

        // if there are 3 or 2 of the same, then the first 3 or 2 will all show the same number,
        // and only the char at index 2 or 3 will show the frequency of the next highest.
        // And then, if it's [0]=2, then [1]=2! and if [2]==2 -> [3]=2!
        // So we can just check [3] for both cases.
        if (counts1[0] == 3 || counts1[0] == 2)  && counts1[3] != counts2[3] {
            return counts1[3].cmp(&counts2[3])
        }

        for (i, c) in chars1.iter().enumerate() {
            let score1 = scores.get(&c).unwrap();
            let score2 = scores.get(&chars2[i]).unwrap();
            if score1 != score2 {
                return score1.cmp(score2)
            }
        }

        Equal
    });

    hands_and_bids.iter().enumerate().rev().for_each(|(index, (hand, _))| println!("{hand} -> {}", index+1));

    let result:usize = hands_and_bids.iter().enumerate()
        .map(|(i, (_, bid))| bid.parse::<usize>().unwrap() * (i+1))
        .sum();

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test.txt");
        assert_eq!(6440, process(input)?);
        Ok(())
    }

    #[test]
    fn test_real() -> miette::Result<()> {
        let input = include_str!("../input.txt");
        let result = process(input)?;
        println!("Result: {result}");
        assert!(248600399 < result);
        Ok(())
    }
}
