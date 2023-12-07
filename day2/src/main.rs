const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

fn main() {
    let mut values = Vec::new();

    println!("Input games! (empty line to stop)");

    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }
        let (id, max_red, max_green, max_blue) = parse(&line);

        if max_red <= MAX_RED && max_green <= MAX_GREEN && max_blue <= MAX_BLUE {
            values.push(id);
        }
    }

    println!("Result: {}", values.iter().sum::<u32>())
}

/// Returns (game_id, max_red, max_green, max_blue).
fn parse(s: &str) -> (u32, u32, u32, u32) {
    let parts = s.split(':').collect::<Vec<_>>();
    assert!(parts.len() == 2);

    let game_id = parts[0]
        .split(' ')
        .skip(1)
        .next()
        .unwrap()
        .parse::<u32>()
        .unwrap();

    let mut max_red = 0;
    let mut max_green = 0;
    let mut max_blue = 0;

    for round in parts[1].split(';') {
        for draw in round.split(',') {
            let parts = draw.trim().split(' ').collect::<Vec<_>>();
            let count = parts[0].parse::<u32>().unwrap();
            let color = parts[1];

            match color {
                "red" => max_red = max_red.max(count),
                "green" => max_green = max_green.max(count),
                "blue" => max_blue = max_blue.max(count),
                _ => panic!("unknown color"),
            };
        }
    }

    (game_id, max_red, max_green, max_blue)
}

mod tests {
    use super::*;

    #[test]
    fn parses() {
        assert_eq!(
            parse("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            (1, 4, 2, 6)
        );

        assert_eq!(
            parse("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"),
            (2, 1, 3, 4)
        );

        assert_eq!(
            parse("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"),
            (3, 20, 13, 6)
        );
    }
}
