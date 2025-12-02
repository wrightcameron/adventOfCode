use log::debug;

pub fn problem1(input: &String) -> i32 {
    let next_values = input.lines().map(| x | {
        let sequence = x.split(" ").map(| x | x.parse().unwrap()).collect::<Vec<i32>>();
        next_num_in_seq(&sequence)
    }).collect::<Vec<i32>>();
    return next_values.iter().sum();
}

// pub fn problem2(input: &String) -> i32 {
//     return 0;
// }

fn next_num_in_seq(sequence: &Vec<i32>) -> i32 {
    debug!("Sequence {:?}",sequence);
    // Generate the differences sequence
    let mut differences: Vec<i32> = Vec::new();
    for i in 0..(sequence.len() - 1) {
        differences.push( sequence[i + 1] - sequence[i] );
    }
    // Check if sequence hit the bottom
    let mut is_bottom = true;
    for i in differences.iter() {
        if *i != 0 {
            is_bottom = false;
            break;
        }
    }
    if is_bottom {
         return *sequence.last().unwrap();
    }
    let next_sequence = sequence.last().unwrap() + next_num_in_seq(&differences);
    debug!("Next Sequence {next_sequence}");
    return next_sequence;
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
        let input = fs::read_to_string("data/sample/day_09.txt").expect("Data file doesn't exist!");
        let expected = 114;
        assert_eq!(problem1(&input), expected);
        // Actual
        let input = fs::read_to_string("data/day_09.txt").expect("Data file doesn't exist!");
        let expected = get_solution("day09".to_string(), 1);
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