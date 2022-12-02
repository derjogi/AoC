use crate::utils;

pub fn day1() {
    use utils::utils::*;

    let calories_for_elves = lines_from_file("day1/day1.txt");
    println!("Elf with the most calories has {} calories!", day1::highest_calory_elf(&calories_for_elves));

    println!("The top three Elves have {} calories together.", day1::top_three(&calories_for_elves));


}

mod day1 {
    pub fn highest_calory_elf(elf_calories: &Vec<String>) -> u32 {
        let mut highest: u32 = 0;
        let mut sum: u32 = 0;
        for line in elf_calories {
            if line.is_empty() {
                highest = if highest > sum { highest } else { sum };
                sum = 0
            } else {
                sum = sum + line.parse::<u32>().expect("Oh, this wasn't a number");
            }
        }
        highest
    }

    pub fn top_three(elf_calories: &Vec<String>) -> u32 {
        let mut highest_three: [u32; 3] = [0, 0, 0];
        highest_three.sort();
        let mut sum: u32 = 0;
        for line in elf_calories {
            if line.is_empty() {
                if highest_three[0] < sum {
                    highest_three[0] = sum;
                    highest_three.sort()
                }
                sum = 0
            } else {
                sum = sum + line.parse::<u32>().expect("Oh, this wasn't a number");
            }
        }
        if highest_three[0] < sum {
            highest_three[0] = sum;
            highest_three.sort()
        }
        highest_three.iter().sum()
    }

    #[cfg(test)]
    mod tests {
        use crate::utils::utils::lines_from_file;
        use super::*;   // Making functions from outer module accessible?

        #[test]
        fn get_highest_calory_elf() {
            let vec = lines_from_file("./day1/day1_test.txt");
            assert_eq!(24000, highest_calory_elf(&vec))
        }

        #[test]
        fn get_top_three() {
            let vec = lines_from_file("./day1/day1_test.txt");
            assert_eq!(45000, top_three(&vec))
        }
    }
}
