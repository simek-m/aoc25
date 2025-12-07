use std::convert::From;
use std::error::Error;
use std::fmt::Display;
use std::ops::RangeInclusive;
use std::str::FromStr;

const INPUT_FILE: &str = include_str!("../../data/day02.txt");
#[derive(Debug)]
struct ParseError(String);

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "not a correct ID range format {}", self.0)
    }
}

impl Error for ParseError {}

#[derive(Debug)]
struct IdRange(RangeInclusive<i64>);

fn is_invalid_id(s: &str) -> bool {
    let l = s.len();
    assert!(l > 0);

    for size in 1..=(l / 2) {
        if s[..size] == s[size..] {
            return true;
        }
    }
    false
}

impl IdRange {
    fn invalid_sum(&self) -> i64 {
        self.0
            .clone()
            .filter(|id| is_invalid_id(&id.to_string()))
            .sum()
    }
}

impl From<RangeInclusive<i64>> for IdRange {
    fn from(item: RangeInclusive<i64>) -> Self {
        IdRange(item)
    }
}

impl FromStr for IdRange {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((start_str, end_str)) = s.split_once('-')
            && let Ok(start) = start_str.parse::<i64>()
            && let Ok(end) = end_str.parse::<i64>()
        {
            Ok(RangeInclusive::new(start, end).into())
        } else {
            Err(ParseError(s.to_string()))
        }
    }
}

fn parse_input(file: &str) -> Result<Vec<IdRange>, ParseError> {
    file.split(',').map(str::parse::<IdRange>).collect()
}

fn total_count_invalid_ids(input: Vec<IdRange>) -> i64 {
    input.into_iter().map(|r| r.invalid_sum()).sum()
}

fn main() {
    let input = parse_input(INPUT_FILE).unwrap();
    let solution = total_count_invalid_ids(input);
    println!("Day 02: {solution}");
}
