#[aoc::main(6)]
fn main(input: &str) {
    println!("Results part 1: {}", calculate_marker_position(input, 4));
    println!("Results part 2: {}", calculate_marker_position(input, 14));
}

fn calculate_marker_position(data: &str, word_size: usize) -> usize {
    let chars = data.chars().collect::<Vec<char>>();
    let mut index = 0;
    let mut has_repetition = false;
    while index < chars.len() - word_size {
        let slice = &chars[index..index + word_size];
        for x in slice {
            if slice.iter().filter(|&z| z == x).count() > 1 {
                // occurs more than once, try next slice
                has_repetition = true;
                break;
            }
        }
        if has_repetition {
            has_repetition = false;
            index += 1;
            continue;
        } else {
            break;
        }
    }
    index + word_size
}

#[cfg(test)]
mod tests {
    use super::*;   // Making functions from outer module accessible

    const TEST_INPUT: &str = include_str!("../../inputs/day6/test.in");

    #[test]
    fn solution_part_1() {
        assert_eq!(7, calculate_marker_position(TEST_INPUT, 4));
        assert_eq!(5, calculate_marker_position("bvwbjplbgvbhsrlpgdmjqwftvncz", 4));
        assert_eq!(6, calculate_marker_position("nppdvjthqldpwncqszvftbrmjlhg", 4));
        assert_eq!(10, calculate_marker_position("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4));
        assert_eq!(11, calculate_marker_position("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4));
    }

    #[test]
    fn solution_part_2() {
        assert_eq!(19, calculate_marker_position(TEST_INPUT, 14));
        assert_eq!(23, calculate_marker_position("bvwbjplbgvbhsrlpgdmjqwftvncz", 14));
        assert_eq!(23, calculate_marker_position("nppdvjthqldpwncqszvftbrmjlhg", 14));
        assert_eq!(29, calculate_marker_position("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14));
        assert_eq!(26, calculate_marker_position("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14));
    }
}
