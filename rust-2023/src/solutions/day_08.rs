use std::collections::HashMap;

pub fn problem1(input: &String) -> i64 {
    let (tape, after) = input.split_once("\n\n").unwrap();
    let lines = after.lines();
    let mut cfg = HashMap::new();
    for line in lines{
        let (before, after) = line.split_once("=").unwrap();
        let key = before.trim();
        // let after2 = after2.replace(&['(', ')'][..], "");
        
        let (left, right) = after.split_once(",").unwrap();
        cfg.insert(key, (left.trim().replace("(", ""), right.trim().replace(")", "")) );
    }
    // println!("{:?}", cfg["AAA"]);
    let mut count = 0;
    let mut current_key = "AAA";
    // Get to the end of the tape
   
    while current_key != "ZZZ" {
        let (new_count, new_current_key) = iterate_through_tape(tape.chars().collect(), &cfg, count, current_key);
        count = new_count;
        current_key = new_current_key;
        println!("Current state {}, the count {}", current_key, count);
    }
    // If we got to the end of the tape, but not at ZZZ
    println!("Current state {}, the count {}", current_key, count);
    return count;
}

fn iterate_through_tape<'a>(tape: Vec<char>, map: &'a HashMap<&'a str, (String, String) >, count: i64, current_key: &'a str) -> (i64, &'a str) {
    let mut count = count;
    let mut current_key = current_key;
    for t in tape {
        count += 1;
        if t == 'L' {
            current_key = &map[current_key].0;
        } else {
            current_key = &map[current_key].1;
        }
        if current_key == "ZZZ" {
            break
        }
    }
    return (count, current_key);
}

pub fn problem2(input: &String) -> i64 {
    // TODO Final solution will require Least Common Multiple, like use num::Integer has the ability to do this
    return 0;
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
            fs::read_to_string("data/sample/day_08.txt").expect("Data file doesn't exist!");
        let expected = 6;
        assert_eq!(problem1(&input), expected);
        // Actual
        let input = fs::read_to_string("data/day_08.txt").expect("Data file doesn't exist!");
        let expected = get_solution("day04".to_string(), 1);
        assert_eq!(problem1(&input) as i64, expected);
    }

    // #[test]
    // fn test_problem2() {
    //     // Sample
    //     let input =
    //         fs::read_to_string("data/sample/day_04.txt").expect("Data file doesn't exist!");
    //     let expected = 30;
    //     assert_eq!(problem2(&input), expected);
    //     // Actual
    //     let input = fs::read_to_string("data/day_04.txt").expect("Data file doesn't exist!");
    //     let expected = get_solution("day04".to_string(), 2);
    //     assert_eq!(problem2(&input) as i64, expected);
    // }
}
