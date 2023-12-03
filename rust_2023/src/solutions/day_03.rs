use log::debug;

pub fn problem1(input: &String) -> i32 {
    // Create a 2D array, this array is going to be an array of arrays.
    let lines = input.lines();

    let mut engine_schematic: Vec<Vec<char>> = Vec::new();
    for i in lines {
        let test: Vec<char> = i.chars().collect();
        engine_schematic.push(test)
    }
    print_schematic(&engine_schematic);
    // print_exact_char_schematic(&engine_schematic, 0, 0);

    let coords = find_coord_of_numbers(&engine_schematic);
    let parts_coords = find_part_number_coord(&engine_schematic, &coords);
   
    return build_whole_numbers(&engine_schematic, &parts_coords).iter().sum();
}

// I messed up part 1 now that I see part 2.  In part 1 I hunted for numbers first then hunted for the symbols.
// it would have been better to find coordinates of all symbols and then get all numbers closest to the symbol.
// As I am now forced to do with gears.
pub fn problem2(input: &String) -> i32 {
    // Find all gears
    let lines = input.lines();
    let mut engine_schematic: Vec<Vec<char>> = Vec::new();
    for i in lines {
        let test: Vec<char> = i.chars().collect();
        engine_schematic.push(test)
    };
    return get_gear_ratios(&engine_schematic).iter().sum();
}

fn get_gear_ratios(schematic: &Vec<Vec<char>>) -> Vec<i32> {
    let mut ratios: Vec<i32> = Vec::new();
    let gear = '*';
    //Find all gears
    for (y, outer_vec) in schematic.iter().enumerate() {
        for (x, element) in outer_vec.iter().enumerate() {
            if *element == gear {
                println!("Found gear!");
                ratios.push( find_all_number_around_cord(&schematic, x, y));
            }
        }
    }
    return ratios;
}

fn find_all_number_around_cord(schematic: &Vec<Vec<char>>, x: usize, y: usize) -> i32 {
    let mut numbers: Vec<i32> = Vec::new();
    //Upper Left
    if in_bounds(&schematic, x.saturating_sub(1), y.saturating_sub(1)) && schematic[y.saturating_sub(1)][x.saturating_sub(1)].is_ascii_digit() {
        let (num_x, num_y) = find_number_starting_coord(&schematic, x.saturating_sub(1), y.saturating_sub(1) );
        let number = build_whole_number(&schematic, num_x, num_y);
        if !numbers.contains(&number) {
            numbers.push(number);
        }
    }
    //Upper
    if in_bounds(&schematic, x, y.saturating_sub(1)) && schematic[y.saturating_sub(1)][x].is_ascii_digit() {
        let (num_x, num_y) = find_number_starting_coord(&schematic, x, y.saturating_sub(1) );
        let number = build_whole_number(&schematic, num_x, num_y);
        if !numbers.contains(&number) {
            numbers.push(number);
        }
    }
    //Upper Right
    if in_bounds(&schematic, x + 1, y.saturating_sub(1)) && schematic[y.saturating_sub(1)][x + 1].is_ascii_digit() {
        let (num_x, num_y) = find_number_starting_coord(&schematic, x + 1, y.saturating_sub(1) );
        let number = build_whole_number(&schematic, num_x, num_y);
        if !numbers.contains(&number) {
            numbers.push(number);
        }
    }
    // Right
    if in_bounds(&schematic, x + 1, y) && schematic[y][x + 1].is_ascii_digit() {
        let (num_x, num_y) = find_number_starting_coord(&schematic, x + 1, y );
        let number = build_whole_number(&schematic, num_x, num_y);
        if !numbers.contains(&number) {
            numbers.push(number);
        }
    }
    // Down Right
    if in_bounds(&schematic, x + 1, y + 1) && schematic[y + 1][x + 1].is_ascii_digit() {
        let (num_x, num_y) = find_number_starting_coord(&schematic, x + 1, y + 1 );
        let number = build_whole_number(&schematic, num_x, num_y);
        if !numbers.contains(&number) {
            numbers.push(number);
        }
    }
    // Down
    if in_bounds(&schematic, x, y + 1) && schematic[y + 1][x].is_ascii_digit() {
        let (num_x, num_y) = find_number_starting_coord(&schematic,  x, y + 1);
        let number = build_whole_number(&schematic, num_x, num_y);
        if !numbers.contains(&number) {
            numbers.push(number);
        }
    }
    // Down Left
    if in_bounds(&schematic, x.saturating_sub(1), y + 1) && schematic[y + 1][x.saturating_sub(1)].is_ascii_digit() {
        let (num_x, num_y) = find_number_starting_coord(&schematic, x.saturating_sub(1), y + 1);
        let number = build_whole_number(&schematic, num_x, num_y);
        if !numbers.contains(&number) {
            numbers.push(number);
        }
    }
    // Left
    if in_bounds(&schematic, x.saturating_sub(1), y) && schematic[y][x.saturating_sub(1)].is_ascii_digit() {
        let (num_x, num_y) = find_number_starting_coord(&schematic, x.saturating_sub(1), y );
        let number = build_whole_number(&schematic, num_x, num_y);
        if !numbers.contains(&number) {
            numbers.push(number);
        }
    }
    //Multiply the numbers
    if numbers.len() != 2 {
        return 0;
    }
    let mut gear_ratio = 1;
    for i in numbers {
        println!("Number: {i}");
        gear_ratio *= i;
    }
    println!("Ratio: {gear_ratio}");
    return gear_ratio;

}

fn find_number_starting_coord(schematic: &Vec<Vec<char>>, x: usize, y: usize) -> (usize, usize) {
    if x == 0{
        return (x, y);
    }
    if ! in_bounds(&schematic, x - 1, y) || ! schematic[y][x - 1].is_ascii_digit() {
        return (x, y);
    }
    return find_number_starting_coord(&schematic, x - 1, y);
}

// Vec<(u32, u32)>
fn find_coord_of_numbers(engine_schematic: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut coord: Vec<(usize, usize)> = Vec::new();
    for (y, outer_vec) in engine_schematic.iter().enumerate() {
        for (x, element) in outer_vec.iter().enumerate() {
            if element.is_ascii_digit() && (x == 0 || !is_coord_digit(&engine_schematic, x.saturating_sub(1), y) ){
                coord.push((x, y));
                debug!("Found {element} at {x},{y}");
            }
        }
    }
    return coord;
}

fn find_part_number_coord(
    engine_schematic: &Vec<Vec<char>>,
    number_coords: &Vec<(usize, usize)>,
) -> Vec<(usize, usize)> {
    // Check all number coords for all spots around it for symbols.
    let mut coord: Vec<(usize, usize)> = Vec::new();

    for (x, y) in number_coords {
        // println!("Coordinates ({x},{y}), In Bounds {}, and symbols near?");
        if recursive_find_symbol(&engine_schematic, *x, *y) {
            coord.push((*x, *y));
            debug!("Found part {} at {x},{y}", engine_schematic[*y][*x])
        }
    }
    return coord;
}

fn recursive_find_symbol(schematic: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    // If out of bounds or number no longer digit return false
    if ! in_bounds(&schematic, x, y) || ! is_coord_digit(&schematic, x, y){
        return false;
    }
    if symbol_near(&schematic, x, y) {
        return true;
    }
    return recursive_find_symbol(&schematic, x + 1, y);
}

fn build_whole_number(schematic: &Vec<Vec<char>>, x: usize, y: usize) -> i32 {
    let number = recursive_build_number(&schematic, x, y).iter().cloned().collect::<String>().parse().unwrap();
    return number;
}

fn build_whole_numbers(schematic: &Vec<Vec<char>>, coords: &Vec<(usize, usize)>) -> Vec<i32> {
    let mut numbers: Vec<i32> = Vec::new();
    
    for (x, y) in coords {
        let number_string = recursive_build_number(&schematic, *x, *y).iter().cloned().collect::<String>();
        println!("Whole number {number_string}");
        numbers.push( number_string.parse().unwrap() );
    }
    return numbers;
}

fn recursive_build_number(schematic: &Vec<Vec<char>>, x: usize, y: usize) -> Vec<char>{
    // If out of bounds or number no longer digit return false
    if ! in_bounds(&schematic, x, y) || ! is_coord_digit(&schematic, x, y){
        return Vec::new();
    }
    let mut tmp: Vec<char> = Vec::new();
    tmp.push( schematic[y][x] );
    tmp.extend( recursive_build_number(&schematic, x + 1, y) );
    return tmp;
}

fn is_coord_digit(schematic: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let is_digit = schematic[y][x].is_ascii_digit();
    debug!("{x} {y}, is digit {is_digit}");
    return is_digit;
}

fn print_schematic(schematic: &Vec<Vec<char>>) {
    for x in schematic {
        println!("{:?}", x)
    }
}

fn print_exact_char_schematic(schematic: &Vec<Vec<char>>, x: usize, y: usize) {
    println!("{}", schematic[x][y]);
}

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

fn symbol_near(schematic: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let mut SYMBOLS: Vec<char> = Vec::new(); // TODO change this to a tuple
    SYMBOLS.push('*');
    SYMBOLS.push('#');
    SYMBOLS.push('+');
    SYMBOLS.push('$');
    SYMBOLS.push('-');
    SYMBOLS.push('&');
    SYMBOLS.push('%');
    SYMBOLS.push('@');
    SYMBOLS.push('/');
    SYMBOLS.push('=');
    SYMBOLS.push('$');
    println!("{x} {y}");
    //Upper Left
    if in_bounds(&schematic, x.saturating_sub(1), y.saturating_sub(1)) && SYMBOLS.contains(&schematic[y.saturating_sub(1)][x.saturating_sub(1)]) {
        return true;
    }
    //Upper
    if in_bounds(&schematic, x, y.saturating_sub(1)) && SYMBOLS.contains(&schematic[y.saturating_sub(1)][x]) {
        return true;
    }
    //Upper Right
    if in_bounds(&schematic, x + 1, y.saturating_sub(1)) && SYMBOLS.contains(&schematic[y.saturating_sub(1)][x + 1]) {
        return true;
    }
    // Right
    if in_bounds(&schematic, x + 1, y) && SYMBOLS.contains(&schematic[y][x + 1]) {
        return true;
    }
    // Down Right
    if in_bounds(&schematic, x + 1, y + 1) && SYMBOLS.contains(&schematic[y + 1][x + 1]) {
        return true;
    }
    // Down
    if in_bounds(&schematic, x, y + 1) && SYMBOLS.contains(&schematic[y + 1][x]) {
        return true;
    }
    // Down Left
    if in_bounds(&schematic, x.saturating_sub(1), y + 1) && SYMBOLS.contains(&schematic[y + 1][x.saturating_sub(1)]) {
        return true;
    }
    // Left
    if in_bounds(&schematic, x.saturating_sub(1), y) && SYMBOLS.contains(&schematic[y][x.saturating_sub(1)]) {
        return true;
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    //Arrange
    //Act
    //Assert

    #[test]
    fn test_problem1() {
        let input =
            fs::read_to_string("data/sample/day_03_sample.txt").expect("Data file doesn't exist!");
        let expected = 4361;
        assert_eq!(problem1(&input), expected);
    }

    // #[test]
    // fn test_problem2() {
    //     let input =
    //         fs::read_to_string("data/sample/day_03_sample2.txt").expect("Data file doesn't exist!");
    //     let expected = 281;
    //     assert_eq!(problem2(&input), expected);
    // }
}
