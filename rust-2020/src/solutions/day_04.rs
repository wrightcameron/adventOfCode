use log::debug;

/// Count number of valid passports
pub fn problem1(input: &String) -> i32 {
    let passports = input.split("\n\n").collect::<Vec<&str>>();
    return passports
        .iter()
        .map(|x| valid_passport(&x.to_string()))
        .filter(|x| *x == true)
        .count() as i32;
}

// TODO Need to do, should use regex and capture groups
// pub fn problem2(input: &String) -> i32 {
//     return 0;
// }

/// Check if passport is valid. A valid passport contains all needed sections.
fn valid_passport(passport: &String) -> bool {
    let valid = passport.contains("byr")
        && passport.contains("iyr")
        && passport.contains("eyr")
        && passport.contains("hgt")
        && passport.contains("hcl")
        && passport.contains("ecl")
        && passport.contains("pid");
    if valid {
        debug!("Passport valid:\n{passport}");
    }
    return valid;
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
        let input = fs::read_to_string("data/sample/day_04.txt").expect("Data file doesn't exist!");
        let expected = 2;
        assert_eq!(problem1(&input), expected);
        //Actual
        let input = fs::read_to_string("data/day_04.txt").expect("Data file doesn't exist!");
        let expected = get_solution("day04".to_string(), 1);
        assert_eq!(problem1(&input) as i64, expected);
    }

    // #[test]
    // fn test_problem2() {
    //     // Sample
    //     let input = fs::read_to_string("data/sample/day_04.txt").expect("Data file doesn't exist!");
    //     let expected = 241861950;
    //     assert_eq!(problem2(&input), expected);
    //     // Actual
    //     let input = fs::read_to_string("data/day_04.txt").expect("Data file doesn't exist!");
    //     let expected = get_solution("day04".to_string(), 2);
    //     assert_eq!(problem1(&input), expected);
    // }
}
