use regex::Regex;

/// Problem 1,
pub fn problem1(input: &str) -> i32 {
    input.lines().map(| line | {
        disallowed_strings(line) && three_vowels(line) && duplicate_letter(line)
    })
    .filter(| x | *x == true )
    .collect::<Vec<bool>>()
    .len() as i32
}

fn three_vowels(input: &str) -> bool {
    let re = Regex::new(r"[aeiou]").unwrap();
    let count = re.find_iter(input).count();
    count >= 3
}

fn duplicate_letter(input: &str) -> bool {
    let mut current_char = input.chars().nth(0).unwrap(); 
    for c in input.chars().skip(1) {
        if c == current_char {
            return true
        } else {
            current_char = c;
        }
    }
    return false
}

fn disallowed_strings(input: &str) -> bool {
    let re = Regex::new(r"ab|cd|pq|xy").unwrap();
    let count = re.find_iter(input).count();
    count == 0
}

/// Problem 2,
pub fn problem2(input: &str) -> i32 {
    0
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
        let input = "ugknbfddgicrmopn";
        let expected = 1;
        assert_eq!(problem1(input), expected);
        let input = "aaa";
        let expected = 1;
        assert_eq!(problem1(input), expected);
        let input = "jchzalrnumimnmhp";
        let expected = 0;
        assert_eq!(problem1(input), expected);
        let input = "haegwjzuvuyypxyu";
        let expected = 0;
        assert_eq!(problem1(input), expected);
        let input = "dvszwmarrgswjxmb";
        let expected = 0;
        assert_eq!(problem1(input), expected);
        //Actual
        let input = fs::read_to_string("data/day_05.txt").expect("Data file doesn't exist!");
        let expected = get_solution("day05".to_string(), 1);
        assert_eq!(problem1(&input) as i64, expected);
    }

    #[test]
    fn test_problem2() {
        // Sample
        let input = "^v";
        let expected = 3;
        assert_eq!(problem2(input), expected);
        //Actual
        let input = fs::read_to_string("data/day_05.txt").expect("Data file doesn't exist!");
        let expected = get_solution("day05".to_string(), 2);
        assert_eq!(problem2(&input) as i64, expected);
    }

}
