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
        ('T', 10),
        ('9', 9),
        ('8', 8),
        ('7', 7),
        ('6', 6),
        ('5', 5),
        ('4', 4),
        ('3', 3),
        ('2', 2),
        ('J', 1)
    ]);

    let mut hands_and_bids: Vec<(&str, &str)> = input.lines().map(|line| line.split_once(" ").unwrap()).collect();
    hands_and_bids.sort_by(|set1, set2| {
        let hand1 = set1.0;
        let hand2 = set2.0;
        let chars1: Vec<char> = hand1.chars().collect();
        let chars2: Vec<char> = hand2.chars().collect();
        // Todo: count J's extra. AND: exclude it from normal count??? Or not?
        let mut counts1: Vec<usize> = chars1.iter()
            .map(|c| if c.as_char() == 'J' {0} else {hand1.matches(c.as_char()).count()})
            .collect();
        let mut counts2: Vec<usize> = chars2.iter()
            .map(|c| if c.as_char() == 'J' {0} else {hand2.matches(c.as_char()).count()})
            .collect();
        let j1_count = hand1.matches('J').count();
        let j2_count = hand2.matches('J').count();
        counts1.sort();
        counts1.reverse();
        counts2.sort();
        counts2.reverse();

        if counts1[0] + j1_count > counts2[0] + j2_count {
            return Greater
        } else if counts1[0] + j1_count < counts2[0] + j2_count {
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

        // Joker:
        // ✅ AJTJA --> [2,2,1,0,0] --> 4 + 1 (covered above)
        // ✅ KAAKK --> [3,3,3,2,2]
        // AJTAT --> [2,2,2,2,0] --> 3+2
        // AJTA3 --> [2,2,1,1,0] --> 3+1+1
        // AJTJ3 --> [1,1,1,0,0] --> 3+1+1
        // AQAQT --> [2.2.2.2.1] --> 2+2+1

        // Handle full house (vs triplet):
        let full_house1 = is_full_house(&mut counts1, j1_count);
        let full_house2 = is_full_house(&mut counts2, j2_count);
        if full_house1 != full_house2 {
            return if full_house1 { Greater } else { Less }
        }

        // handle triplet (vs 2 pairs):
        let triplet1 = has_triplet(&mut counts1, j1_count);
        let triplet2 = has_triplet(&mut counts2, j2_count);
        if triplet1 != triplet2 {
            return if triplet1 { Greater } else { Less }
        }

        // handle 2 pairs:
        let pairs1 = has_two_pairs(&mut counts1, j1_count);
        let pairs2 = has_two_pairs(&mut counts2, j2_count);
        if pairs1 != pairs2 {
            return if pairs1 { Greater } else { Less }
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

fn has_triplet(mut hand: &mut Vec<usize>, jokers: usize) -> bool {
    match jokers {
        2 => hand[0] == 1,
        1 => hand[0] == 2,
        0 => hand[0] == 3,
        _ => false,
    }
}

fn is_full_house(mut hand: &mut Vec<usize>, jokers: usize) -> bool {
    match jokers {
        2 => false,  // any pair would be elevated to four of a kind
        1 => hand[2] == 2,
        0 => hand[3] == 2,
        _ => false
    }
}

fn has_two_pairs(mut hand: &mut Vec<usize>, jokers: usize) -> bool {
    match jokers {
        1 => false,  // with one joker, if you have a pair then you have a triplet, so never two pairs.
        0 => hand[0] == 2 && hand[2] == 2,
        _ => false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test.txt");
        assert_eq!(5905, process(input)?);
        Ok(())
    }

    #[test]
    fn test_real() -> miette::Result<()> {
        let input = include_str!("../input.txt");
        let result = process(input)?;
        println!("Result: {result}");
        assert!(250585045 > result);
        assert!(250131387 < result);

        Ok(())
    }
}
