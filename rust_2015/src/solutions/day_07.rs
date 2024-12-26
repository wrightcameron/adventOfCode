use std::collections::HashMap;

struct Instruction<'a> {
    command: &'a str,
    args: [&'a str; 2],
    destination: &'a str,
}

/// Problem 1,
pub fn problem1(input: &str) -> i32 {
    let mut signals: HashMap<&str, u16> = HashMap::new();
    for line in input.lines() {
        let (left, right) = line.split_once("->").unwrap();
        let variable = right.trim();
        let statement = left.trim().split_whitespace().collect::<Vec<&str>>();
        // Exp: 123 -> x
        if statement.len() == 1 {
            let value = return_digit_or_variable(&signals, statement[0]);
            signals.insert(variable, value);
        } 
        // Exp: NOT x -> h
        else if statement.len() == 2 {
            let value = ! return_digit_or_variable(&signals, statement[1]);
            signals.insert(variable, value);
        } 
        // Exp: x AND y -> d
        else if statement.len() == 3 {
            let left_value = return_digit_or_variable(&signals, statement[0]);
            let operation = statement[1];
            let right_value = return_digit_or_variable(&signals, statement[2]);
            match operation {
                "AND" => {
                    let res = left_value & right_value;
                    signals.insert(variable, res);
                },
                "OR" => {
                    let res = left_value | right_value;
                    signals.insert(variable, res);
                },
                "LSHIFT" => {
                    let res = left_value << right_value;
                    signals.insert(variable, res);
                },
                "RSHIFT" => {
                    let res = left_value >> right_value;
                    signals.insert(variable, res);
                },
                _ => panic!("Unrecognized symbol")
            }
        } else {
            panic!("Left side statement has 0 or greater than 3 elements");
        }
    }
    // dbg!(signals);
    if signals.contains_key("a") {
        *signals.get("a").unwrap() as i32
    } else {
        0
    }
}

fn return_digit_or_variable(signals: &HashMap<&str, u16>, value: &str) -> u16 {
    if value.chars().all(| c | c.is_digit(10)){
        value.parse::<u16>().unwrap()
    } else {
        if signals.contains_key(value) {
            *signals.get(value).unwrap()
        } else {
            0
        }
    }
}

/// Problem 2,
pub fn problem2(input: &str) -> i32 {
    todo!()
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
        // let input = "123 -> x\n\
        //                 456 -> y\n\
        //                 x AND y -> d\n\
        //                 x OR y -> e\n\
        //                 x LSHIFT 2 -> f\n\
        //                 y RSHIFT 2 -> g\n\
        //                 NOT x -> h\n\
        //                 NOT y -> i";
        // let expected = 0;
        // assert_eq!(problem1(input), expected);
        //Actual
        let input = fs::read_to_string("data/day_07.txt").expect("Data file doesn't exist!");
        let expected = get_solution("day07".to_string(), 1);
        assert_eq!(problem1(&input) as i64, expected);
    }

    #[test]
    fn test_problem2() {
        // Sample
        let input = "^v";
        let expected = 3;
        assert_eq!(problem2(input), expected);
        //Actual
        let input = fs::read_to_string("data/day_07.txt").expect("Data file doesn't exist!");
        let expected = get_solution("day07".to_string(), 2);
        assert_eq!(problem2(&input) as i64, expected);
    }

}
