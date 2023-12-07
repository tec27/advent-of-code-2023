use std::{
    collections::HashSet,
    io::{self, BufRead},
};

fn main() {
    let mut values = Vec::new();

    println!("Input games! (empty line to stop)");

    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }

        values.push(card_value(&line));
    }

    println!("Result, part 1: {}", values.iter().sum::<u32>())
}

fn card_value(s: &str) -> u32 {
    let parts = s
        .split(":")
        .skip(1)
        .next()
        .unwrap()
        .split("|")
        .collect::<Vec<_>>();
    let winners = parts[0]
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<HashSet<_>>();
    let ours = parts[1]
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let num_winners = ours.iter().filter(|&n| winners.contains(n)).count() as u32;
    if num_winners == 0 {
        0
    } else {
        2_u32.pow(num_winners - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_values() {
        assert_eq!(
            card_value("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"),
            8
        );
        assert_eq!(
            card_value("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19"),
            2
        );
        assert_eq!(
            card_value("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1"),
            2
        );
        assert_eq!(
            card_value("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 831"),
            1
        );
        assert_eq!(
            card_value("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36"),
            0
        );
        assert_eq!(
            card_value("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"),
            0
        );
    }
}
