use std::error::Error;
use std::fmt::Display;
use std::str::FromStr;

const INPUT_FILE: &str = include_str!("../../data/day03.txt");
#[derive(Debug)]
struct ParseError(String);

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "not a correct battery format {}", self.0)
    }
}

impl Error for ParseError {}

#[derive(Debug)]
struct BatteryBank(Vec<u8>);

impl BatteryBank {
    fn joltage(&self) -> i64 {
        let mut joltages = vec![];
        for (idx, jolt1) in self.0.iter().enumerate() {
            let without_idx = &self.0[idx+1..self.0.len()];
            for jolt2 in without_idx {
                let digits = format!("{jolt1}{jolt2}");
                // Panic Safety: must be positive numbers.
                let num = digits.parse().unwrap();
                joltages.push(num);
            }
        }

        joltages.into_iter().max().unwrap_or(0i64)
    }
}

impl FromStr for BatteryBank {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bank = Vec::with_capacity(s.len());
        for char in s.chars() {
            let Ok(parsed) = char.to_string().parse::<u8>() else {
                return Err(ParseError(s.to_string()));
            };
            bank.push(parsed);
        }

        Ok(BatteryBank(bank))
    }
}

fn parse_input(file: &str) -> Result<Vec<BatteryBank>, ParseError> {
    file.lines().map(str::parse::<BatteryBank>).collect()
}

fn total_output_joltage(banks: &[BatteryBank]) -> i64 {
    banks.iter().map(BatteryBank::joltage).sum()
}

fn main() {
    let banks = parse_input(INPUT_FILE).unwrap();
    let solution = total_output_joltage(&banks);
    println!("Day 03: {solution}");
}

mod tests {
    use super::*;

    #[test]
    fn test_battery_bank_joltage() {
        let bank = BatteryBank::from_str("818181911112111").unwrap();
        let joltage = bank.joltage();

        assert_eq!(joltage, 92);
    }
}
