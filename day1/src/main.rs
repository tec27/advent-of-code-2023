use std::{
    io::{self, BufRead},
    sync::OnceLock,
};

use regex::Regex;

fn main() {
    let mut values = Vec::new();

    println!("Input values! (empty line to stop)");

    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }
        values.push(calibration_value(&line));
    }

    println!("Result: {}", values.iter().sum::<u32>())
}

static REGEX_CELL: OnceLock<Regex> = OnceLock::new();

fn calibration_value(s: &str) -> u32 {
    let re = REGEX_CELL.get_or_init(|| {
        Regex::new(r"([0-9]|one|two|three|four|five|six|seven|eight|nine)").unwrap()
    });

    let mut digits = Vec::new();
    let mut i = 0;
    while i < s.len() {
        let Some(m) = re.find_at(s, i) else {
            break;
        };

        digits.push(match m.as_str() {
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9,
            s => {
                if let Ok(n) = s.parse::<u32>() {
                    n
                } else {
                    panic!("Invalid digit: {}", s)
                }
            }
        });

        i = m.start() + 1;
    }

    digits.first().unwrap() * 10 + digits.last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calibration_values() {
        assert_eq!(calibration_value("1abc2"), 12);
        assert_eq!(calibration_value("pqr3stu8vwx"), 38);
        assert_eq!(calibration_value("a1b2c3d4e5f"), 15);
        assert_eq!(calibration_value("treb7uchet"), 77);
    }

    #[test]
    fn calibration_values_part2() {
        assert_eq!(calibration_value("two1nine"), 29);
        assert_eq!(calibration_value("eightwothree"), 83);
        assert_eq!(calibration_value("abcone2threexyz"), 13);
        assert_eq!(calibration_value("xtwone3four"), 24);
        assert_eq!(calibration_value("4nineeightseven2"), 42);
        assert_eq!(calibration_value("zoneight234"), 14);
        assert_eq!(calibration_value("7pqrstsixteen"), 76);
        assert_eq!(calibration_value("oneight"), 18);
    }
}
