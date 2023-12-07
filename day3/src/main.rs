use std::{
    collections::HashSet,
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

    let part_numbers = find_part_numbers(&schematic);
    println!("Result: {}", part_numbers.iter().sum::<u32>());
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

fn find_part_numbers(schematic: &[String]) -> Vec<u32> {
    let mut numbers = Vec::new();
    let mut symbols = HashSet::new();

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

        'outer: for sx in min_x..=max_x {
            for sy in min_y..=max_y {
                if sy == y && sx >= x && sx < x + len {
                    // Skip positions that are within the number itself
                    continue;
                }

                if symbols.contains(&Location { x: sx, y: sy }) {
                    part_numbers.push(number);
                    break 'outer;
                }
            }
        }
    }

    part_numbers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_part_numbers() {
        assert_eq!(
            find_part_numbers(
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
                .collect::<Vec<_>>()
            ),
            [467, 35, 633, 617, 592, 755, 664, 598]
        )
    }
}
