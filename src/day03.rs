use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn day_03_part_one() -> i64 {
    let file = File::open("./inputs/day03.txt").expect("File day 3 should exist");
    let buf_reader = BufReader::new(file);

    let mut total_output_joltage = 0;
    for line in buf_reader.lines() {
        let bank_of_batteries = line.expect("Should read line");
        let joltages = bank_of_batteries.chars().collect::<Vec<char>>();
        let mut largest_joltage: u8 = 0;

        let length = bank_of_batteries.chars().count();

        for i in 0..length {
            for j in i + 1..length {
                let joltage_candidate = format!("{}{}", joltages[i], joltages[j])
                    .parse::<u8>()
                    .expect("Should be able to parse two number strings concatenated");

                if joltage_candidate > largest_joltage {
                    largest_joltage = joltage_candidate;
                }
            }
        }

        total_output_joltage += largest_joltage as i64;
    }

    return total_output_joltage;
}

pub fn day_03_part_two() -> i64 {
    let file = File::open("./inputs/day03_test.txt").expect("File day 3 should exist");
    let buf_reader = BufReader::new(file);

    let mut total_output_joltage = 0;
    for line in buf_reader.lines() {
        let bank_of_batteries = line.expect("Should read line");
        let joltages = bank_of_batteries.chars().collect::<Vec<char>>();
        let mut largest_joltage: u64 = 0;

        let length = bank_of_batteries.chars().count();

        for i in 0..length {
            let mut current_turned_on_batteries = String::new();
            current_turned_on_batteries.push(joltages[i]);
            for j in i + 1..12 {
                current_turned_on_batteries.push(joltages[j]);
                let joltage_candidate = format!("{}{}", joltages[i], joltages[j])
                    .parse::<u64>()
                    .expect("Should be able to parse two number strings concatenated");

            }

            let joltage_candidate = current_turned_on_batteries.parse::<u64>().expect("Should be able to parse string to u64");
            if joltage_candidate > largest_joltage {
                println!("Found new largest joltage candidate: {}", joltage_candidate);
                largest_joltage = joltage_candidate;
            }
        }

        total_output_joltage += largest_joltage as i64;
    }

    return total_output_joltage;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_03_part_one() {
        let result = day_03_part_one();
        assert_eq!(result, 17324);
    }

    #[test]
    fn test_day_03_part_two() {
        let result = day_03_part_two();
        assert_eq!(result, 17324);
    }
}
