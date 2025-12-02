use log::debug;

const SYMBOLS: [char; 11] = ['*', '#', '+', '$', '-', '&', '%', '@', '/', '=', '$'];

/// Find all symbols and then sum up all numbers around symbols
pub fn problem1(input: &String) -> i32 {
    // Create a 2D array, this array is going to be an array of arrays.
    let engine_schematic: Vec<Vec<char>> = input
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect();

    // print_exact_char_schematic(&engine_schematic, 0, 0);

    let coords = find_coord_of_symbols(&engine_schematic);
    return coords.iter().map(| (x, y) | find_sum_around_cord(&engine_schematic, *x, *y)).sum();
}

// Find all gear symbols and then multiple 2 numbers around gear and then sum up all those numbers.
pub fn problem2(input: &String) -> i32 {
    // Find all gears
    let engine_schematic: Vec<Vec<char>> = input
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect();
    return get_gear_ratios(&engine_schematic).iter().sum();
}

///Find all the coordinates of the symbols in the engine schematic
fn find_coord_of_symbols(schematic: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut coord: Vec<(usize, usize)> = Vec::new();
    for (y, outer_vec) in schematic.iter().enumerate() {
        for (x, element) in outer_vec.iter().enumerate() {
            if SYMBOLS.contains(element) {
                coord.push((x, y));
                debug!("Found {element} at {x},{y}");
            }
        }
    }
    return coord;
}

/// Get all the gear ratios in the schematic
/// 
/// A gear ratio is a * symbol that has two numbers around it,
/// both numbers have to be present and then are multiplied.
fn get_gear_ratios(schematic: &Vec<Vec<char>>) -> Vec<i32> {
    let mut ratios: Vec<i32> = Vec::new();
    let gear = '*';
    //Find all gears
    for (y, outer_vec) in schematic.iter().enumerate() {
        for (x, element) in outer_vec.iter().enumerate() {
            if *element == gear {
                debug!("Found gear!");
                ratios.push(find_all_number_around_cord(&schematic, x, y));
            }
        }
    }
    return ratios;
}

/// Get all number coords around given coord
fn get_all_number_cords_around_cord(schematic: &Vec<Vec<char>>, x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut number_coords: Vec<(usize, usize)> = Vec::new();
    //Upper Left
    if in_bounds(&schematic, x.saturating_sub(1), y.saturating_sub(1))
        && schematic[y.saturating_sub(1)][x.saturating_sub(1)].is_ascii_digit()
    {
        let (num_x, num_y) =
            find_number_starting_coord(&schematic, x.saturating_sub(1), y.saturating_sub(1));
        if !number_coords
            .iter()
            .any(|(tmp_x, tmp_y)| num_x == *tmp_x && num_y == *tmp_y)
        {
            number_coords.push((num_x, num_y));
        }
    }
    //Upper
    if in_bounds(&schematic, x, y.saturating_sub(1))
        && schematic[y.saturating_sub(1)][x].is_ascii_digit()
    {
        let (num_x, num_y) = find_number_starting_coord(&schematic, x, y.saturating_sub(1));
        if !number_coords
            .iter()
            .any(|(tmp_x, tmp_y)| num_x == *tmp_x && num_y == *tmp_y)
        {
            number_coords.push((num_x, num_y));
        }
    }
    //Upper Right
    if in_bounds(&schematic, x + 1, y.saturating_sub(1))
        && schematic[y.saturating_sub(1)][x + 1].is_ascii_digit()
    {
        let (num_x, num_y) = find_number_starting_coord(&schematic, x + 1, y.saturating_sub(1));
        if !number_coords
            .iter()
            .any(|(tmp_x, tmp_y)| num_x == *tmp_x && num_y == *tmp_y)
        {
            number_coords.push((num_x, num_y));
        }
    }
    // Right
    if in_bounds(&schematic, x + 1, y) && schematic[y][x + 1].is_ascii_digit() {
        let (num_x, num_y) = find_number_starting_coord(&schematic, x + 1, y);
        if !number_coords
            .iter()
            .any(|(tmp_x, tmp_y)| num_x == *tmp_x && num_y == *tmp_y)
        {
            number_coords.push((num_x, num_y));
        }
    }
    // Down Right
    if in_bounds(&schematic, x + 1, y + 1) && schematic[y + 1][x + 1].is_ascii_digit() {
        let (num_x, num_y) = find_number_starting_coord(&schematic, x + 1, y + 1);
        if !number_coords
            .iter()
            .any(|(tmp_x, tmp_y)| num_x == *tmp_x && num_y == *tmp_y)
        {
            number_coords.push((num_x, num_y));
        }
    }
    // Down
    if in_bounds(&schematic, x, y + 1) && schematic[y + 1][x].is_ascii_digit() {
        let (num_x, num_y) = find_number_starting_coord(&schematic, x, y + 1);
        if !number_coords
            .iter()
            .any(|(tmp_x, tmp_y)| num_x == *tmp_x && num_y == *tmp_y)
        {
            number_coords.push((num_x, num_y));
        }
    }
    // Down Left
    if in_bounds(&schematic, x.saturating_sub(1), y + 1)
        && schematic[y + 1][x.saturating_sub(1)].is_ascii_digit()
    {
        let (num_x, num_y) = find_number_starting_coord(&schematic, x.saturating_sub(1), y + 1);
        if !number_coords
            .iter()
            .any(|(tmp_x, tmp_y)| num_x == *tmp_x && num_y == *tmp_y)
        {
            number_coords.push((num_x, num_y));
        }
    }
    // Left
    if in_bounds(&schematic, x.saturating_sub(1), y)
        && schematic[y][x.saturating_sub(1)].is_ascii_digit()
    {
        let (num_x, num_y) = find_number_starting_coord(&schematic, x.saturating_sub(1), y);
        if !number_coords
            .iter()
            .any(|(tmp_x, tmp_y)| num_x == *tmp_x && num_y == *tmp_y)
        {
            number_coords.push((num_x, num_y));
        }
    }
    return number_coords;
}

/// Get gear ratio (multiple of two numbers) around coord
fn find_all_number_around_cord(schematic: &Vec<Vec<char>>, x: usize, y: usize) -> i32 {
    let mut numbers: Vec<i32> = Vec::new();
    let number_coords: Vec<(usize, usize)> = get_all_number_cords_around_cord(schematic, x, y);
    for (x, y) in number_coords {
        let number = build_whole_number(&schematic, x, y);
        numbers.push(number);
    }
    //Multiply the numbers
    if numbers.len() != 2 {
        return 0;
    }
    let mut gear_ratio = 1;
    for i in numbers {
        debug!("Number: {i}");
        gear_ratio *= i;
    }
    debug!("Ratio: {gear_ratio}");
    return gear_ratio;
}

/// Get sum of multiple numbers around coord
fn find_sum_around_cord(schematic: &Vec<Vec<char>>, x: usize, y: usize) -> i32 {
    let mut numbers: Vec<i32> = Vec::new();
    let number_coords: Vec<(usize, usize)> = get_all_number_cords_around_cord(schematic, x, y);
    for (x, y) in number_coords {
        let number = build_whole_number(&schematic, x, y);
        numbers.push(number);
    }
    return numbers.iter().sum();
}

/// Find starting coord of a number, using recursion to shift left till first number is found
fn find_number_starting_coord(schematic: &Vec<Vec<char>>, x: usize, y: usize) -> (usize, usize) {
    if x == 0 {
        return (x, y);
    }
    if !in_bounds(&schematic, x - 1, y) || !schematic[y][x - 1].is_ascii_digit() {
        return (x, y);
    }
    return find_number_starting_coord(&schematic, x - 1, y);
}

/// Combine all chars representing a number into an actual number represented as i32
fn build_whole_number(schematic: &Vec<Vec<char>>, x: usize, y: usize) -> i32 {
    let number = recursive_build_number(&schematic, x, y)
        .iter()
        .cloned()
        .collect::<String>()
        .parse()
        .unwrap();
    return number;
}

/// Find how large a number is, using recursion to shift right till last number is found
fn recursive_build_number(schematic: &Vec<Vec<char>>, x: usize, y: usize) -> Vec<char> {
    // If out of bounds or number no longer digit return false
    if !in_bounds(&schematic, x, y) || !is_coord_digit(&schematic, x, y) {
        return Vec::new();
    }
    let mut tmp: Vec<char> = Vec::new();
    tmp.push(schematic[y][x]);
    tmp.extend(recursive_build_number(&schematic, x + 1, y));
    return tmp;
}

/// Check if coord is digit
fn is_coord_digit(schematic: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let is_digit = schematic[y][x].is_ascii_digit();
    debug!("{x} {y}, is digit {is_digit}");
    return is_digit;
}

/// Check if coords are within bounds of schematic
fn in_bounds(schematic: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    // 4.saturating_sub(rhs) TODO Add this instead of using isize to check if sub would wrap around.
    // if y.saturating_sub(-1) || x.saturating_sub(-1) {
    //     return false;
    // }
    if y > schematic.len() - 1 || x > schematic[y].len() - 1 {
        return false;
    }
    return true;
}

fn print_schematic(schematic: &Vec<Vec<char>>) {
    for x in schematic {
        println!("{:?}", x)
    }
}

fn print_exact_char_schematic(schematic: &Vec<Vec<char>>, x: usize, y: usize) {
    println!("{}", schematic[x][y]);
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
        let input = fs::read_to_string("data/sample/day_03.txt").expect("Data file doesn't exist!");
        let expected = 4361;
        assert_eq!(problem1(&input), expected);
        // Actual
        let input = fs::read_to_string("data/day_03.txt").expect("Data file doesn't exist!");
        let expected = get_solution("day03".to_string(), 1);
        assert_eq!(problem1(&input) as i64, expected);
    }

    #[test]
    fn test_problem2() {
        // Sample
        let input = fs::read_to_string("data/sample/day_03.txt").expect("Data file doesn't exist!");
        let expected = 467835;
        assert_eq!(problem2(&input), expected);
        // Actual
        let input = fs::read_to_string("data/day_03.txt").expect("Data file doesn't exist!");
        let expected = get_solution("day03".to_string(), 2);
        assert_eq!(problem2(&input) as i64, expected);
    }
}
