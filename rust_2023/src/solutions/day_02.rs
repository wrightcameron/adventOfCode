use log::debug;

pub fn problem1(input: &String) -> u16 {
    println!("Starting Day 2");
    let stuff = input.lines();
    let mut games = 0;
    let red_dice = 12;
    let green_dice = 13;
    let blue_dice = 14;
    
    for i in stuff {
        let (game_id, red, green, blue) = parse_line(i);
        println!("Game id {game_id}, red: {red}, green {green}, blue {blue}");
        if red <= red_dice && green <= green_dice && blue <= blue_dice {
            games += game_id;
        }
    }
    return games;
}


//TODO parse Line, return tuple size four Game ID, R, G, B
fn parse_line(line: &str) -> (u16, u16, u16, u16) {
    debug!("The full line is '{line}'");
    // Parse the game id
    let mut split = line.split(": ");
    let game = split.next().unwrap();
    let rest = split.next().unwrap();
    // Parse the game id out of game, the parse pulls ints out of strings.
    let game_id: u16 = game.replace("Game ", "").parse().unwrap();
    let dice_games = rest.split("; ").collect::<Vec<&str>>();
    
    let mut total_red = 0;
    let mut total_green = 0;
    let mut total_blue = 0;
    for dice_game in dice_games {
        let (red, green, blue) = parse_dice_game(dice_game);
        total_red += red;
        total_green += green;
        total_blue += blue;
    }
    debug!("The game is '{game_id}', while the rest of the line is '{rest}'");
    return (game_id, total_red, total_green, total_blue);
}

fn parse_dice_game(dice_game: &str) ->  (u16, u16, u16) {
    let colors:  Vec<(String, u16)> = dice_game.split(", ").map(| dice | {
        let (amount, color) = dice.split_once(' ').unwrap();
        let amount: u16 = amount.parse().unwrap();
        (color.to_string(), amount)
    }).collect();

    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

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