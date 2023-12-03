use log::debug;

pub fn problem1(input: &String) -> i32 {
    // Create a 2D array, this array is going to be an array of arrays.
    let lines = input.lines();

    // let SYMBOLS: (char) = ('*', '#', '+', '$');

    let mut engine_schematic: Vec<Vec<char>> = Vec::new();
    for i in lines {
        let test: Vec<char> = i.chars().collect();
        engine_schematic.push(test)
    }
    print_schematic(&engine_schematic);
    // print_exact_char_schematic(&engine_schematic, 0, 0);
    
    let coords = find_coord_of_numbers(&engine_schematic);
    let parts_coords = find_part_number_coord(&engine_schematic, &coords);
    
    return 0;
}

// pub fn problem2(input: &String) -> i32 {
//     return 0;
// }

// Vec<(u32, u32)>
fn find_coord_of_numbers(engine_schematic: &Vec<Vec<char>>) -> Vec<(usize, usize)>{
    let mut coord: Vec<(usize, usize)> = Vec::new();
    for (y, outer_vec) in engine_schematic.iter().enumerate() {
        for (x, element) in outer_vec.iter().enumerate() {
            if element.is_ascii_digit() {
                coord.push( (x, y) );
                println!("Found {element} at {x},{y}")
            }
        }
    }
    return coord;
}

fn find_part_number_coord(engine_schematic: &Vec<Vec<char>>, number_coords: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    // Check all number coords for all spots around it for symbols.
    let mut coord: Vec<(usize, usize)> = Vec::new();


    for (x, y) in number_coords {
        // println!("Coordinates ({x},{y}), In Bounds {}, and symbols near?");
        if symbol_near(&engine_schematic, *x, *y) {
            coord.push( (*x, *y) );
            println!("Found part {} at {x},{y}", engine_schematic[*y][*x])
        }
    }
    return coord;
}

fn print_schematic(schematic: &Vec<Vec<char>>) {
    for x in schematic {
        println!("{:?}", x)
    }
}

fn print_exact_char_schematic(schematic: &Vec<Vec<char>>, x: usize, y: usize) {
    println!("{}", schematic[x][y]);
}

fn in_bounds(schematic: &Vec<Vec<char>>, x: isize, y: isize) -> bool {
    // 4.saturating_sub(rhs) TODO Add this instead of using isize to check if sub would wrap around.
     if y < 0 || x < 0 {
        return false;
     }
     if y as usize > schematic.len() - 1 ||  x as usize > schematic[y as usize].len() {
        return false
     }
     return true;
}

fn symbol_near(schematic: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let mut SYMBOLS: Vec<char> = Vec::new(); // TODO change this to a tuple
    SYMBOLS.push('*');
    SYMBOLS.push('#');
    SYMBOLS.push('+');
    SYMBOLS.push('$');
    //Upper Left
    if in_bounds(&schematic, x as isize - 1 , y as isize - 1 ) && SYMBOLS.contains(&schematic[y - 1 ][x - 1]){
        return true;
    }
    //Upper
    if in_bounds(&schematic, x as isize, y as isize - 1) && SYMBOLS.contains(&schematic[ y - 1][x]){
        return true;
    }
    //Upper Right
    if in_bounds(&schematic, x as isize + 1, y as isize - 1) && SYMBOLS.contains(&schematic[ y - 1][x + 1]){
        return true;
    }
    // Right
    if in_bounds(&schematic, x as isize + 1, y as isize) && SYMBOLS.contains(&schematic[y][x + 1]){
        return true;
    }
    // Down Right
    if in_bounds(&schematic, x as isize + 1, y as isize + 1) && SYMBOLS.contains(&schematic[y + 1][x + 1]){
        return true;
    }
    // Down
    if in_bounds(&schematic, x as isize, y as isize + 1) && SYMBOLS.contains(&schematic[y][x + 1]){
        return true;
    }
    // Down Left
    if in_bounds(&schematic, x as isize - 1, y as isize + 1) && SYMBOLS.contains(&schematic[y + 1][x - 1]){
        return true;
    }
    // Left
    if in_bounds(&schematic, x as isize - 1, y as isize) && SYMBOLS.contains(&schematic[y][x - 1]){
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
