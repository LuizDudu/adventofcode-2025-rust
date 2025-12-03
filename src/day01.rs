use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn day_01() -> i16 {
    let file = File::open("./inputs/day01.txt").expect("File day 1 doesn't exist");
    let buf_reader = BufReader::new(file);

    let mut dial: Dial = Dial::new();
    for line in buf_reader.lines() {
        let instruction = Instruction::new(line.unwrap().to_string());
        dial.rotate(instruction);
    }

    println!(
        "Dial times pointing at zero: {}",
        dial.times_pointing_at_zero()
    );

    return dial.times_pointing_at_zero();
}

pub struct Instruction {
    direction: char,
    value: i16,
}

impl Instruction {
    pub fn new(_instruction: String) -> Self {
        let (direction, value) = _instruction.split_at(1);
        return Self {
            direction: direction.parse().unwrap(),
            value: value.parse().unwrap(),
        };
    }
}

#[derive(Debug)]
pub struct Dial {
    value: i16,
    zero_frequency_on_rotations: i16,
}

impl Dial {
    pub fn new() -> Self {
        return Self {
            value: 50,
            zero_frequency_on_rotations: 0,
        };
    }

    pub fn rotate(&mut self, _instruction: Instruction) {
        match _instruction.direction {
            'R' => {
                let mut pending_rotations = _instruction.value;
                while pending_rotations > 0 {
                    if self.value == 99 {
                        self.value = 0;
                    } else {
                        self.value += 1;
                    }

                    if self.value == 0 {
                        self.zero_frequency_on_rotations += 1;
                    }
                    pending_rotations -= 1;
                }
            }
            'L' => {
                let mut pending_rotations = _instruction.value;
                while pending_rotations > 0 {
                    if self.value == 0 {
                        self.value = 99;
                        if self.value == 0 {
                            self.zero_frequency_on_rotations += 1;
                        }
                    } else {
                        self.value -= 1;
                        if self.value == 0 {
                            self.zero_frequency_on_rotations += 1;
                        }
                    }

                    pending_rotations -= 1;
                }
            }
            _ => {
                panic!("Direction not treated!");
            }
        }
    }

    pub fn times_pointing_at_zero(&self) -> i16 {
        return self.zero_frequency_on_rotations;
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day01() {
        let result = day_01();
        assert_eq!(result, 5831)
    }

    #[test]
    fn test_instruction_struct() {
        let instruction = Instruction::new("R2253".to_string());
        assert_eq!(instruction.direction, 'R');
        assert_eq!(instruction.value, 2253)
    }

    #[test]
    fn test_dial_rotate_right() {
        let instruction1 = Instruction::new("R151".to_string());
        let mut dial = Dial::new();
        dial.rotate(instruction1);

        assert_eq!(dial.value, 1);
        assert_eq!(dial.times_pointing_at_zero(), 2);
    }

    #[test]
    fn test_dial_rotate_left() {
        let instruction1 = Instruction::new("L151".to_string());
        let mut dial = Dial::new();
        dial.rotate(instruction1);

        assert_eq!(dial.value, 99)
    }

    #[test]
    fn test_dial_rotate_bigger_than_one_hundred() {
        // Arrange
        let instruction1 = Instruction::new("L200".to_string());
        let instruction2 = Instruction::new("L250".to_string());
        let instruction3 = Instruction::new("L1000".to_string());

        let instruction_right_1 = Instruction::new("R100".to_string());
        let instruction_right_2 = Instruction::new("R250".to_string());
        let instruction_right_3 = Instruction::new("R1000".to_string());

        // Act
        let mut dial1 = Dial::new();
        dial1.rotate(instruction1);
        dial1.rotate(instruction2);
        dial1.rotate(instruction3);

        let mut dial2 = Dial::new();
        dial2.rotate(instruction_right_1);
        dial2.rotate(instruction_right_2);
        dial2.rotate(instruction_right_3);

        // Assert
        assert_eq!(dial1.value, 0);
        assert_eq!(dial2.value, 0);
    }

    #[test]
    fn test_dial_advent_of_code_example() {
        // Arrange
        let instruction_1 = Instruction::new("L68".to_string());
        let instruction_2 = Instruction::new("L30".to_string());
        let instruction_3 = Instruction::new("R48".to_string());
        let instruction_4 = Instruction::new("L5".to_string());
        let instruction_5 = Instruction::new("R60".to_string());
        let instruction_6 = Instruction::new("L55".to_string());
        let instruction_7 = Instruction::new("L1".to_string());
        let instruction_8 = Instruction::new("L99".to_string());
        let instruction_9 = Instruction::new("R14".to_string());
        let instruction_10 = Instruction::new("L82".to_string());
        let instruction_11 = Instruction::new("L100".to_string());
        let instruction_12 = Instruction::new("R100".to_string());

        // Act - Assert
        let mut dial: Dial = Dial::new();
        assert_eq!(dial.value, 50);

        dial.rotate(instruction_1);
        assert_eq!(dial.value, 82);
        assert_eq!(dial.zero_frequency_on_rotations, 1, "instruction 1");

        dial.rotate(instruction_2);
        assert_eq!(dial.value, 52);
        assert_eq!(dial.zero_frequency_on_rotations, 1, "instruction 2");

        dial.rotate(instruction_3);
        assert_eq!(dial.value, 0);
        assert_eq!(dial.zero_frequency_on_rotations, 2, "instruction 3");

        dial.rotate(instruction_4);
        assert_eq!(dial.value, 95);
        assert_eq!(dial.zero_frequency_on_rotations, 2, "instruction 4");

        dial.rotate(instruction_5);
        assert_eq!(dial.value, 55);
        assert_eq!(dial.zero_frequency_on_rotations, 3, "instruction 5");

        dial.rotate(instruction_6);
        assert_eq!(dial.value, 0);
        assert_eq!(dial.zero_frequency_on_rotations, 4, "instruction 6");

        dial.rotate(instruction_7);
        assert_eq!(dial.value, 99);
        assert_eq!(dial.zero_frequency_on_rotations, 4, "instruction 7");

        dial.rotate(instruction_8);
        assert_eq!(dial.value, 0);
        assert_eq!(dial.zero_frequency_on_rotations, 5, "instruction 8");

        dial.rotate(instruction_9);
        assert_eq!(dial.value, 14);
        assert_eq!(dial.zero_frequency_on_rotations, 5, "instruction 9");

        dial.rotate(instruction_10);
        assert_eq!(dial.value, 32);
        assert_eq!(dial.zero_frequency_on_rotations, 6, "instruction 10");

        dial.rotate(instruction_11);
        dial.rotate(instruction_12);
        assert_eq!(dial.value, 32);
        assert_eq!(dial.zero_frequency_on_rotations, 8, "instruction 11 and 12");
    }

    #[test]
    fn test_dial_times_pointing_at_zero() {
        // Arrange
        let mut dial = Dial::new();
        let instruction = Instruction::new("R1000".to_string());

        // Act
        dial.rotate(instruction);

        // Assert
        assert_eq!(dial.times_pointing_at_zero(), 10);
    }

    #[test]
    fn test_simple_rotation() {
        // Arrange
        let mut dial = Dial::new();
        let instruction = Instruction::new("L51".to_string());

        // Act
        dial.rotate(instruction);

        // Assert
        assert_eq!(dial.value, 99);
    }
}
