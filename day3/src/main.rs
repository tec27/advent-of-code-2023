use std::{
    collections::{HashMap, HashSet},
    io::{self, BufRead},
};

fn main() {
    let mut schematic = Vec::new();

    println!("Input schematic! (empty line to stop)");

    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }

        schematic.push(line);
    }

    let (part_numbers, gears) = find_part_numbers(&schematic);
    println!("Result, part 1: {}", part_numbers.iter().sum::<u32>());
    println!("Result, part 2: {}", gears.iter().sum::<u32>());
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct Location {
    x: usize,
    y: usize,
}

#[derive(Copy, Clone, Debug)]
struct LocatedNumber {
    number: u32,
    location: Location,
}

/// Returns (part_numbers, gear_ratios).
fn find_part_numbers(schematic: &[String]) -> (Vec<u32>, Vec<u32>) {
    let mut numbers = Vec::new();
    let mut symbols = HashSet::new();
    // Map of location => (count, gear_ratio)
    let mut maybe_gears = HashMap::new();

    for (y, line) in schematic.iter().enumerate() {
        let mut reading_number = false;
        let mut cur_number = 0;
        let mut cur_location = Location { x: 0, y };
        for (x, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                cur_number = cur_number * 10 + c.to_digit(10).unwrap();
                if !reading_number {
                    cur_location.x = x;
                    reading_number = true;
                }
            } else {
                if reading_number {
                    reading_number = false;
                    numbers.push(LocatedNumber {
                        number: cur_number,
                        location: cur_location,
                    });
                    cur_number = 0;
                }

                if c == '.' {
                    continue;
                } else {
                    symbols.insert(Location { x, y });
                    if c == '*' {
                        maybe_gears.insert(Location { x, y }, (0, 1));
                    }
                }
            }
        }

        if reading_number {
            numbers.push(LocatedNumber {
                number: cur_number,
                location: cur_location,
            });
        }
    }

    let max_x = schematic.get(0).unwrap_or(&"".to_owned()).len() - 1;
    let max_y = schematic.len() - 1;

    let mut part_numbers = Vec::new();
    for LocatedNumber {
        number,
        location: Location { x, y },
    } in numbers
    {
        let min_x = if x > 0 { x - 1 } else { x };
        let min_y = if y > 0 { y - 1 } else { y };
        let max_y = if y < max_y { y + 1 } else { y };
        let len = number.to_string().len();
        let max_x = (x + len).min(max_x);
        let mut found = false;

        for sx in min_x..=max_x {
            for sy in min_y..=max_y {
                if sy == y && sx >= x && sx < x + len {
                    // Skip positions that are within the number itself
                    continue;
                }

                if !found && symbols.contains(&Location { x: sx, y: sy }) {
                    part_numbers.push(number);
                    found = true;
                }
                if let Some(gear) = maybe_gears.get_mut(&Location { x: sx, y: sy }) {
                    gear.0 += 1;
                    gear.1 *= number;
                }
            }
        }
    }

    (
        part_numbers,
        maybe_gears
            .values()
            .filter_map(|(count, ratio)| if *count == 2 { Some(*ratio) } else { None })
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_part_numbers() {
        let (mut parts, mut gears) = find_part_numbers(
            &r"
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
            "
            .trim()
            .lines()
            .map(|l| l.trim().to_owned())
            .collect::<Vec<_>>(),
        );

        parts.sort();
        gears.sort();

        let mut expected = vec![467, 35, 633, 617, 592, 755, 664, 598];
        expected.sort();
        assert_eq!(parts, expected);

        let mut expected = vec![467 * 35, 755 * 598];
        expected.sort();
        assert_eq!(gears, expected)
    }
}
