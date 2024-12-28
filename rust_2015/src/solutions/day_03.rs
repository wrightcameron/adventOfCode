
type Coord = (i32, i32);

/// Problem 1,
pub fn problem1(input: &str) -> i32 {
    input.lines().map(| line | {
        let mut coordinates: Coord = (0,0);
        // let mut unique_houses = 0;
        let mut house_coordinates: Vec<Coord> = vec![coordinates];
        for i in line.chars() {
            match i {
                '^' => coordinates.1 += 1,
                '>' => coordinates.0 += 1,
                'v' => coordinates.1 -= 1,
                '<' => coordinates.0 -= 1,
                _ => panic!("Not recognized symbol"),
            };
            // dbg!(coordinates);
            if ! house_coordinates.contains(&coordinates) {
                house_coordinates.push(coordinates);
            }
        }
        house_coordinates.len() as i32
    }).sum()
}

/// Problem 2,
pub fn problem2(input: &str) -> i32 {
    input.lines().map(| line | {
        let mut coordinates: Coord = (0,0);
        // let mut unique_houses = 0;
        let mut house_coordinates: Vec<Coord> = vec![coordinates];
        let santa_directions = line.chars()
            .enumerate()
            .filter(|&(i, _)| i % 2 == 0 )
            .map(|(_, e)| e)
            .collect::<Vec<char>>();
        let robot_santa_directions = line.chars()
        .enumerate()
        .filter(|&(i, _)| i % 2 != 0 )
        .map(|(_, e)| e)
        .collect::<Vec<char>>();
        for i in santa_directions {
            match i {
                '^' => coordinates.1 += 1,
                '>' => coordinates.0 += 1,
                'v' => coordinates.1 -= 1,
                '<' => coordinates.0 -= 1,
                _ => panic!("Not recognized symbol"),
            };
            // dbg!(coordinates);
            if ! house_coordinates.contains(&coordinates) {
                house_coordinates.push(coordinates);
            }
        }
        coordinates = (0,0);
        for i in robot_santa_directions {
            match i {
                '^' => coordinates.1 += 1,
                '>' => coordinates.0 += 1,
                'v' => coordinates.1 -= 1,
                '<' => coordinates.0 -= 1,
                _ => panic!("Not recognized symbol"),
            };
            // dbg!(coordinates);
            if ! house_coordinates.contains(&coordinates) {
                house_coordinates.push(coordinates);
            }
        }
        house_coordinates.len() as i32
    }).sum()
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
        let input = ">";
        let expected = 2;
        assert_eq!(problem1(input), expected);
        let input = "^>v<";
        let expected = 4;
        assert_eq!(problem1(input), expected);
        let input = "^v^v^v^v^v";
        let expected = 2;
        assert_eq!(problem1(input), expected);
        //Actual
        let input = fs::read_to_string("data/day_03.txt").expect("Data file doesn't exist!");
        let expected = get_solution("day03".to_string(), 1);
        assert_eq!(problem1(&input) as i64, expected);
    }

    #[test]
    fn test_problem2() {
        // Sample
        let input = "^v";
        let expected = 3;
        assert_eq!(problem2(input), expected);
        let input = "^>v<";
        let expected = 3;
        assert_eq!(problem2(input), expected);
        let input = "^v^v^v^v^v";
        let expected = 11;
        assert_eq!(problem2(input), expected);
        //Actual
        let input = fs::read_to_string("data/day_03.txt").expect("Data file doesn't exist!");
        let expected = get_solution("day03".to_string(), 2);
        assert_eq!(problem2(&input) as i64, expected);
    }

}
