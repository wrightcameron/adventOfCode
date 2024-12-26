use log::debug;

/// Problem 1,
pub fn problem1(input: &str) -> i32 {
    let asc = input.chars().filter(| c | *c == '(').count() as i32;
    let desc = input.chars().filter(| c | *c == ')').count() as i32;
    asc - desc
}

/// Problem 2,
pub fn problem2(input: &str) -> i32 {
    let mut curr_floor = 0;
    let mut basement_position = 0;
    for (i, el) in input.chars().enumerate() {
        if curr_floor == 0 && el == ')' {
            basement_position = i as i32 + 1;
            break;
        }
        if el == '(' {
            curr_floor += 1;
        } else if el == ')' {
            curr_floor -= 1;
        } else {
            panic!("Unsupported char: {el}");
        }
    }
    basement_position
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
        // Floor 0
        let input = "(())";
        let expected = 0;
        assert_eq!(problem1(input), expected);
        let input = "()()";
        assert_eq!(problem1(input), expected);
        // Floor 3
        let input = "(((";
        let expected = 3;
        assert_eq!(problem1(input), expected);
        let input = "(()(()(";
        assert_eq!(problem1(input), expected);
        // Floor 3
        let input = "))(((((";
        let expected = 3;
        assert_eq!(problem1(input), expected);
        // Floor -1
        let input = "())";
        let expected = -1;
        assert_eq!(problem1(input), expected);
        let input = "))(";
        assert_eq!(problem1(input), expected);
        // Floor -3
        let input = ")))";
        let expected = -3;
        assert_eq!(problem1(input), expected);
        let input = ")())())";
        assert_eq!(problem1(input), expected);
        //Actual
        let input = fs::read_to_string("data/day_01.txt").expect("Data file doesn't exist!");
        let expected = get_solution("day01".to_string(), 1);
        assert_eq!(problem1(&input) as i64, expected);
    }

    #[test]
    fn test_problem2() {
        // Sample
        
        // Position 1
        let input = ")";
        let expected = 1;
        assert_eq!(problem2(input), expected);
        // Position 5
        let input = "()())";
        let expected = 5;
        assert_eq!(problem2(input), expected);
        //Actual
        let input = fs::read_to_string("data/day_01.txt").expect("Data file doesn't exist!");
        let expected = get_solution("day01".to_string(), 2);
        assert_eq!(problem2(&input) as i64, expected);
    }

}
