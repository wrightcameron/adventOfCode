use md5;

/// Problem 1,
pub fn problem1(input: &str) -> i32 {
    let mut index = 0;
    loop {
        let msg = format!("{input}{index}");
        let digest = md5::compute(msg.as_bytes());
        let str_digest = format!("{:x}", digest);
        let slice = &str_digest[0..5];
        let all_zeros = slice.chars().all(| c | c == '0');
        if all_zeros {
            break;
        } else {
            index += 1;
        }
    }
    index
}

/// Problem 2,
pub fn problem2(input: &str) -> i32 {
    let mut index = 0;
    loop {
        let msg = format!("{input}{index}");
        let digest = md5::compute(msg.as_bytes());
        let str_digest = format!("{:x}", digest);
        let slice = &str_digest[0..6];
        let all_zeros = slice.chars().all(| c | c == '0');
        if all_zeros {
            break;
        } else {
            index += 1;
        }
    }
    index
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
        let input = "abcdef";
        let expected = 609043;
        assert_eq!(problem1(input), expected);
        let input = "pqrstuv";
        let expected = 1048970;
        assert_eq!(problem1(input), expected);
        //Actual
        let input = fs::read_to_string("data/day_04.txt").expect("Data file doesn't exist!");
        let expected = get_solution("day04".to_string(), 1);
        assert_eq!(problem1(&input) as i64, expected);
    }

    #[test]
    fn test_problem2() {
        //Actual
        let input = fs::read_to_string("data/day_04.txt").expect("Data file doesn't exist!");
        let expected = get_solution("day04".to_string(), 2);
        assert_eq!(problem2(&input) as i64, expected);
    }

}
