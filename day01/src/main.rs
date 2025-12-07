use std::error::Error;
use std::fmt::Display;
use std::str::{self, FromStr};

/// `AoC` main input.
const INPUT_FILE: &str = include_str!("../../data/day01.txt");

/// Maximal dial number before wrap around.
const MAX_NUMBER: i64 = 99;

/// Threshold for wraparounds (0 with L1 is 99)
const ROTATION_THRESHOLD: i64 = MAX_NUMBER + 1;
/// Starting position for the dial.
const STARTING_POSITION: i64 = 50;

enum Rotation {
    Left(i64),
    Right(i64),
}

#[derive(Debug)]
struct ParseError(String);

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "not a correct rotation repr {}", self.0)
    }
}

impl Error for ParseError {}

impl FromStr for Rotation {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 2 {
            return Err(ParseError(s.to_string()));
        }

        let (direction_str, amount_str) = s.split_at(1);
        let amount = amount_str.parse::<i64>();

        match (direction_str, amount) {
            ("L", Ok(amount)) => Ok(Rotation::Left(amount)),
            ("R", Ok(amount)) => Ok(Rotation::Right(amount)),
            _ => Err(ParseError(s.to_string())),
        }
    }
}

fn parse_input(file: &str) -> Result<Vec<Rotation>, ParseError> {
    file.lines().map(str::parse::<Rotation>).collect()
}

fn door_password(input: &str) -> Result<i64, ParseError> {
    let mut password = 0;
    let mut current_position = STARTING_POSITION;
    for rotation in parse_input(input)? {
        if let Rotation::Left(amount) = rotation {
            let wrapped_amount = amount % ROTATION_THRESHOLD;
            if (current_position - wrapped_amount) < 0 {
                current_position = (current_position - wrapped_amount) + ROTATION_THRESHOLD;
            } else {
                current_position -= wrapped_amount;
            }
        } else if let Rotation::Right(amount) = rotation {
            current_position = (current_position + amount) % ROTATION_THRESHOLD;
        }

        if current_position == 0 {
            password += 1;
        }
    }

    Ok(password)
}

fn main() {
    const EXAMPLE_INPUT: &str = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";
    let test_solution = door_password(EXAMPLE_INPUT).unwrap();
    assert_eq!(test_solution, 3);

    let solution = door_password(INPUT_FILE).unwrap();
    println!("Day 01: {solution}");
}
