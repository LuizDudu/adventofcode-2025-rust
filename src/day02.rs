use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn day_02() -> i64 {
    let file = File::open("./inputs/day02.txt").expect("File day 2 should exist");
    let mut buf_reader = BufReader::new(file);
    let mut input = String::new();
    buf_reader
        .read_line(&mut input)
        .expect("Should read first line");

    let id_ranges_text = input.split(",");
    let mut invalid_ids: Vec<i64> = vec![];
    let mut sum = 0;
    for id_range_text in id_ranges_text {
        let id_range = IdRange::new(id_range_text);
        let mut current_id = id_range.first_id;

        while current_id <= id_range.last_id {
            let current_id_as_string = current_id.to_string();
            let current_chars = current_id_as_string.chars();
            let char_count = current_chars.count();
            for current_length in 1..char_count {
                if char_count % current_length != 0 {
                    continue;
                }

                let all_parts =
                    get_string_separated_by_chunks(current_id.to_string(), current_length);

                let mut all_equal = true;

                let first_part = all_parts.first().expect("Should have first part");
                for part in all_parts.iter() {
                    if part != first_part {
                        all_equal = false;
                        break;
                    }
                }

                if all_equal {
                    sum += current_id;
                    invalid_ids.push(current_id);
                    break;
                }
            }

            current_id += 1;
        }
    }

    println!("The sum is: {}", sum);

    return sum;
}

fn get_string_separated_by_chunks(value: String, chunk: usize) -> Vec<String> {
    let all_parts: Vec<String> = value
        .chars()
        .collect::<Vec<char>>()
        .chunks(chunk)
        .map(|c| c.iter().collect::<String>())
        .collect();

    return all_parts
}

struct IdRange {
    first_id: i64,
    last_id: i64,
}

impl IdRange {
    pub fn new(range: &str) -> Self {
        let first_and_last_ids = range
            .split("-")
            .map(|id| id.trim().parse::<i64>())
            .filter_map(Result::ok)
            .collect::<Vec<i64>>();

        if first_and_last_ids.first().is_none() {
            panic!(
                "Ids should exist, here is range: {} \n And here is first_and_last_ids: {:?}",
                range, first_and_last_ids
            );
        }

        return IdRange {
            first_id: *first_and_last_ids.first().expect("First id should exist"),
            last_id: *first_and_last_ids.last().expect("Second id should exist"),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_2() {
        let result = day_02();
        assert_eq!(result, 28915664389);
    }

    #[test]
    fn test_get_string_separated_by_chunks() {
        // Arrange
        let value: String = "121212".to_string();
        let chunk: usize = 2;
        // Act
        let result = get_string_separated_by_chunks(value, chunk);

        // Assert
        assert_eq!(result.len(), 3);
    }
}
