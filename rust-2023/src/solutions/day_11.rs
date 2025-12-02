use log::debug;

#[derive(Debug)]
struct Galaxy {
    id: i32,
    x: i32,
    y: i32,
}

pub fn problem1(input: &String) -> i32 {
    let mut galaxy_map: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();
    galaxy_map = galactic_expansion(&galaxy_map);
    // Get list of all galaxies, and their coords
    let mut count = 1;
    let mut galaxies: Vec<Galaxy> = Vec::new();
    //TODO Change this to fold, can't do with just map as some rows will have more than one galaxy
    for (y, row) in galaxy_map.iter().enumerate() {
        for (x, element) in row.iter().enumerate() {
            if *element == '#' {
                galaxies.push(Galaxy {
                    id: count,
                    x: x.try_into().unwrap(),
                    y: y.try_into().unwrap(),
                });
                count += 1;
            }
        }
    }
    // Manhattan distance or Taxicab geometry is used. https://en.wikipedia.org/wiki/Taxicab_geometry
    // Sum is divided in half to reduce duplicate entries, as we should have double of them.
    debug!("Galaxies {:?}", galaxies.len());
    return galaxies
        .iter()
        .map(|a| {
            galaxies
                .iter()
                .map(|b| {
                    debug!("Galaxy A: {:?}", a);
                    debug!("Galaxy B: {:?}", b);
                    let x_delta = a.x - b.x;
                    let y_delta = a.y - b.y;
                    debug!("Delta B: {:?}", x_delta.abs() + y_delta.abs());
                    return x_delta.abs() + y_delta.abs();
                })
                .sum::<i32>()
        })
        .sum::<i32>()
        / 2;
}

fn galactic_expansion(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    print_map(&map);
    println!("-------");
    let mut map = map.clone();
    // Expand empty columns first
    // Record indexes that don't have have a galaxy on the first line
    let mut column_indexes: Vec<usize> = Vec::new();
    for (index, element) in map[0].iter().enumerate() {
        if *element == '.' {
            column_indexes.push(index);
        }
    }
    // Compare with all the other lines
    column_indexes.retain(|x| {
        let mut delete = false;
        for row in map.iter() {
            if row[*x] == '#' {
                delete = true;
                break;
            }
        }
        !delete
    });
    println!("Columns to expand {:?}", column_indexes);
    // Expand columns
    for index in column_indexes.iter().rev() {
        for row in map.iter_mut() {
            row.insert(*index, '.');
        }
    }
    // Expand empty rows first
    let mut row_indexes: Vec<usize> = Vec::new();
    let column_size = map[0].len();
    let mut empty_row = Vec::new();
    for _i in 0..column_size {
        empty_row.push('.');
    }
    for (index, row) in map.iter_mut().enumerate() {
        if !row.contains(&'#') {
            row_indexes.push(index);
        }
    }
    for i in row_indexes.iter().rev() {
        map.insert(*i, empty_row.clone());
    }
    print_map(&map);
    return map;
}

fn print_map(map: &Vec<Vec<char>>) {
    for x in map {
        println!("{:?}", x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::get_solution;
    use std::fs;

    //Arrange
    //Act
    //Assert

    #[test]
    fn test_problem1() {
        // Sample
        let input = fs::read_to_string("data/sample/day_11.txt").expect("Data file doesn't exist!");
        let expected = 374;
        assert_eq!(problem1(&input), expected);
        // Actual
        let input = fs::read_to_string("data/day_11.txt").expect("Data file doesn't exist!");
        let expected = get_solution("day11".to_string(), 1);
        assert_eq!(problem1(&input) as i64, expected);
    }

    // #[test]
    // fn test_problem2() {
    //     // Sample
    //     let input =
    //         fs::read_to_string("data/sample/day_07.txt").expect("Data file doesn't exist!");
    //     let expected = 71503;
    //     assert_eq!(problem2(&input), expected);
    //     // Actual
    //     let input = fs::read_to_string("data/day_07.txt").expect("Data file doesn't exist!");
    //     let expected = get_solution("day07".to_string(), 2);
    //     assert_eq!(problem2(&input) as i64, expected);
    // }
}
