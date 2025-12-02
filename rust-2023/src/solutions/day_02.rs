use log::debug;

/// Solution to Advent of Code Day 2 Problem 1
///
/// Parse each line of input, find which rounds in each game
/// that has too many dice to be valid.
///
/// * `input` - input text
pub fn problem1(input: &String) -> u32 {
    debug!("Starting Day 2");
    let lines = input.lines();
    let mut games = 0;

    for line in lines {
        let (game_id, valid) = parse_line(line);
        if valid {
            games += game_id;
        }
    }
    return games;
}

/// Solution to Advent of Code Day 2 Problem 2
///
/// Parse each line of input, find the minimum amount of dice
/// to play that round.
///
/// * `input` - input text
pub fn problem2(input: &String) -> u32 {
    let lines = input.lines();
    let mut total = 0;

    for line in lines {
        // Parse the game id
        let mut split = line.split(": ");
        let game = split.next().unwrap();
        let rest = split.next().unwrap();
        // Parse the game id out of game, the parse pulls ints out of strings.
        let game_id: u16 = game.replace("Game ", "").parse().unwrap();
        let dice_games = rest.split("; ").collect::<Vec<&str>>();

        let mut fewest_red_dice = 1;
        let mut fewest_green_dice = 1;
        let mut fewest_blue_dice = 1;

        // For the entire game find the fewest dice.
        for dice_game in dice_games {
            let (red, green, blue) = parse_round(dice_game);
            if fewest_red_dice < red {
                fewest_red_dice = red;
            }
            if fewest_green_dice < green {
                fewest_green_dice = green;
            }
            if fewest_blue_dice < blue {
                fewest_blue_dice = blue;
            }
        }
        let power = fewest_red_dice * fewest_green_dice * fewest_blue_dice;
        debug!("Game {game_id}: Power {power}");
        total += power;
        // debug!("The game is '{game_id}', while the rest of the line is '{rest}'");
    }
    return total;
}

/// Parse line to gain game id and if entire game is valid
///
/// * `line` - game
fn parse_line(line: &str) -> (u32, bool) {
    debug!("The full line is '{line}'");
    // Parse the game id
    let mut split = line.split(": ");
    let game = split.next().unwrap();
    let rest = split.next().unwrap();
    // Parse the game id out of game, the parse pulls ints out of strings.
    let game_id: u32 = game.replace("Game ", "").parse().unwrap();
    let dice_games = rest.split("; ").collect::<Vec<&str>>();

    let red_dice = 12;
    let green_dice = 13;
    let blue_dice = 14;

    // If a round has too many dice the entire game is invalid
    let mut valid = true;
    for dice_game in dice_games {
        let (red, green, blue) = parse_round(dice_game);
        debug!("Game id {game_id}, red: {red}, green {green}, blue {blue}");
        if red > red_dice || green > green_dice || blue > blue_dice {
            valid = false;
            break;
        }
    }
    debug!("The game is '{game_id}', while the rest of the line is '{rest}'");
    return (game_id, valid);
}

/// Parse round of dice game
///
/// A dice game can contain n rounds, this parses one round and
/// returns the number of red, green, blue dice.
///
/// * `dice_round` - String reprentation of dice round.
fn parse_round(round: &str) -> (u32, u32, u32) {
    let colors: Vec<(String, u32)> = round
        .split(", ")
        .map(|dice| {
            let (amount, color) = dice.split_once(' ').unwrap();
            let amount: u32 = amount.parse().unwrap();
            (color.to_string(), amount)
        })
        .collect();

    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

    // Find ammount of red, green, blue dice.  Add them all up
    // into tuple
    for (color, amount) in colors {
        if color == "red" {
            red += amount;
        } else if color == "green" {
            green += amount;
        } else if color == "blue" {
            blue += amount;
        }
    }
    return (red, green, blue);
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use serde_json;
    use std::fs;

    #[derive(Deserialize, Debug)]
    struct Solution {
        id: String,
        first: i64,
        second: i64,
    }

    fn get_solution(day: String, problem: i8) -> i64 {
        let json_string =
            fs::read_to_string("data/solutions.json").expect("JSON file doesn't exist!");
        let json: Vec<Solution> =
            serde_json::from_str(&json_string).expect("JSON was not well-formatted");
        let solution = json.iter().find(|x| x.id == day).unwrap();
        return if problem == 1 {
            solution.first
        } else {
            solution.second
        };
    }

    //Arrange
    //Act
    //Assert

    #[test]
    fn test_problem1() {
        // Sample
        let input =
            fs::read_to_string("data/sample/day_02.txt").expect("Data file doesn't exist!");
        let expected = 8;
        assert_eq!(problem1(&input), expected);
        //Actual
        let input = fs::read_to_string("data/day_02.txt").expect("Data file doesn't exist!");
        let expected = get_solution("day02".to_string(), 1);
        assert_eq!(problem1(&input) as i64, expected);
    }

    #[test]
    fn test_problem2() {
        // Sample
        let input =
            fs::read_to_string("data/sample/day_02.txt").expect("Data file doesn't exist!");
        let expected = 2286;
        assert_eq!(problem2(&input), expected);
        //Actual
        let input = fs::read_to_string("data/day_02.txt").expect("Data file doesn't exist!");
        let expected = get_solution("day02".to_string(), 2);
        assert_eq!(problem2(&input) as i64, expected);
    }
}
