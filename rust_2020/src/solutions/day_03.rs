use log::debug;

/// Find number of trees hit going down mountain.
///
/// Description.
pub fn problem1(input: &String) -> i32 {
    let map: Vec<Vec<char>> = input
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect();
    debug!("Size of map, x: {}, y: {}", map[0].len(), map.len());
    return find_tree_encounter(&map, 3, 1);
}

/// Find number of trees hit going down mountain, at different slopes.
pub fn problem2(input: &String) -> i64 {
    let map: Vec<Vec<char>> = input
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect();
    debug!("Size of map, x: {}, y: {}", map[0].len(), map.len());
    let mut total_tree_hits: Vec<i64> = Vec::new();
    // Right 1, Down 1
    total_tree_hits.push(find_tree_encounter(&map, 1, 1) as i64);
    // Right 3, Down 1
    total_tree_hits.push(find_tree_encounter(&map, 3, 1) as i64);
    // Right 5, Down 1
    total_tree_hits.push(find_tree_encounter(&map, 5, 1) as i64);
    // Right 7, Down 1
    total_tree_hits.push(find_tree_encounter(&map, 7, 1) as i64);
    // Right 1, Down 2
    total_tree_hits.push(find_tree_encounter(&map, 1, 2) as i64);
    return total_tree_hits.iter().product();
}

/// Find number of trees hit going down mountain.
///
/// Loop through map starting for 0,0, and checking each new coord based on slope
/// provided.  Loop through entire map till reach the bottom.
fn find_tree_encounter(map: &Vec<Vec<char>>, x_asc: usize, y_asc: usize) -> i32 {
    let mut x = 0;
    let mut y = 0;
    let mut hit_tree_count = 0;
    while y < map.len() {
        if hit_tree(&map, (x, y)) {
            hit_tree_count += 1;
        }
        (x, y) = new_coords(&map, (x, y), (x_asc, y_asc));
    }
    debug!("Hit trees {hit_tree_count}");
    return hit_tree_count;
}

/// Based on current position and slope, find new coords.
fn new_coords(
    map: &Vec<Vec<char>>,
    (x, y): (usize, usize),
    (x_asc, y_asc): (usize, usize),
) -> (usize, usize) {
    // Find new x
    let new_x = if x + x_asc > map[y].len() - 1 {
        x + x_asc - map[y].len()
    } else {
        x + x_asc
    };
    let new_y = if y + y_asc > map.len() {
        map.len()
    } else {
        y + y_asc
    };
    debug!("New coords {new_x}, {new_y}");
    return (new_x, new_y);
}

/// Are current coords on tree.
///
/// Check if coords passed in are ontop of tree with given map.
fn hit_tree(map: &Vec<Vec<char>>, (x, y): (usize, usize)) -> bool {
    map[y][x] == '#'
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
        let expected = 7;
        assert_eq!(problem1(&input), expected);
        //Actual
        let input = fs::read_to_string("data/day_03.txt").expect("Data file doesn't exist!");
        let expected = get_solution("day03".to_string(), 1);
        assert_eq!(problem1(&input) as i64, expected);
    }

    #[test]
    fn test_problem2() {
        // Sample
        let input = fs::read_to_string("data/sample/day_03.txt").expect("Data file doesn't exist!");
        let expected = 336;
        assert_eq!(problem2(&input), expected);
        // Actual
        let input = fs::read_to_string("data/day_03.txt").expect("Data file doesn't exist!");
        let expected = get_solution("day03".to_string(), 2);
        assert_eq!(problem2(&input) as i64, expected);
    }
}
