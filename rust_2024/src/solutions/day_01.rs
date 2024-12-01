use log::debug;

/// Solution to Advent of Code Problem 1
pub fn problem1(input: &String) -> i32 {
    let mut left_numbers: Vec<i32> = vec![];
    let mut right_numbers: Vec<i32> = vec![];
    // Add all numbers to a right and left vec
    for line in input.lines(){
        let (first, second) = line.split_once("   ").unwrap();
        let first = first.parse::<i32>().unwrap();
        left_numbers.push(first);
        let second = second.parse::<i32>().unwrap();
        right_numbers.push(second);
    }
    // sort both
    left_numbers.sort();
    right_numbers.sort();
    // Get difference between two
    let mut diff_vec = vec![];
    for i in 0..left_numbers.len() {
        let mut diff = (left_numbers[i] - right_numbers[i]);
        diff = diff.abs();
        diff_vec.push(diff);
    }
    return diff_vec.into_iter().sum();
}

/// Solution to Advent of Code Problem 2
pub fn problem2(input: &String) -> i32 {
    let mut left_numbers: Vec<i32> = vec![];
    let mut right_numbers: Vec<i32> = vec![];
    // Add all numbers to a right and left vec
    for line in input.lines(){
        let (first, second) = line.split_once("   ").unwrap();
        let first = first.parse::<i32>().unwrap();
        left_numbers.push(first);
        let second = second.parse::<i32>().unwrap();
        right_numbers.push(second);
    }
    // sort both
    left_numbers.sort();
    right_numbers.sort();
    // Get similarity Score
    let mut similarity_score_vec: Vec<i32> = vec![];
    for i in 0..left_numbers.len() {
        let left = left_numbers[i];
        let occurances: i32 = right_numbers.iter().filter( | i | *i == &left).collect::<Vec<&i32>>().len() as i32;
        let simularity_score = left_numbers[i] * occurances;
        similarity_score_vec.push(simularity_score);
    }
    return similarity_score_vec.into_iter().sum();
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
            fs::read_to_string("data/sample/day_01.txt").expect("Data file doesn't exist!");
        let expected = 11;
        assert_eq!(problem1(&input), expected);
        //Actual
        let input = fs::read_to_string("data/day_01.txt").expect("Data file doesn't exist!");
        let expected = get_solution("day01".to_string(), 1);
        assert_eq!(problem1(&input) as i64, expected);
    }

    #[test]
    fn test_problem2() {
        // Sample
        let input =
            fs::read_to_string("data/sample/day_01.txt").expect("Data file doesn't exist!");
        let expected = 31;
        assert_eq!(problem2(&input), expected);
        //Actual
        let input = fs::read_to_string("data/day_01.txt").expect("Data file doesn't exist!");
        let expected = get_solution("day01".to_string(), 2);
        assert_eq!(problem2(&input) as i64, expected);
    }

}
