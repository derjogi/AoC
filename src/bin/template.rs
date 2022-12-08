// Todo: Replace this number with the day you want to run!
#[aoc::main(1)]
fn main(input: &str) {
    println!("Results: {}", calculate_something(input));
}

pub fn calculate_something(data: &str) -> u32 {
    data.parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;   // Making functions from outer module accessible

    // Todo: replace the number in this path
    const TEST_INPUT: &str = include_str!("../../inputs/day1/test.in");

    #[test]
    fn solution_part_1() {
        assert_eq!(12345, calculate_something(TEST_INPUT))
    }

    #[test]
    fn solution_part_2() {
        assert_eq!(98765, calculate_something(TEST_INPUT))
    }
}
