use std::io::{self, BufRead};

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

fn calibration_value(s: &str) -> u32 {
    let digits = s.chars().filter(|c| c.is_digit(10)).collect::<Vec<_>>();
    digits[0].to_digit(10).unwrap() * 10 + digits[digits.len() - 1].to_digit(10).unwrap()
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
}
