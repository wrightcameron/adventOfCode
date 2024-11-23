use log::debug;

/// Solution to Advent of Code Problem 1
///
/// For a module, take its mass, divide by three, round down, and subtract 2
///
/// * `input` - input Text
pub fn problem1(input: &String) -> i32 {
    input.lines().map(| line | fuel_required(line.parse().unwrap() )).sum()
}

/// Solution to Advent of Code Problem 2
///
/// todo
///
/// * `input` - input Text
pub fn problem2(input: &String) -> i32 {
    todo!()
}

fn fuel_required(mass: i32) -> i32 {
    mass / 3 - 2
}

fn recursive_fuel_required(mass: i32) -> i32 {
    let mut fuel = mass / 3 - 2;
    if fuel < 0 {
        fuel = 0;
    }
    println!("{fuel}");
    if fuel > 0 {
        recursive_fuel_required(fuel) + fuel
    } else {
        mass
    }
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
        todo!();
}
    #[test]
    fn test_problem2() {
        todo!();
    }

    #[test]
    fn test_fuel_required() {
        let expected = 2;
        let actual = fuel_required(12);
        assert_eq!(expected, actual);
        let expected = 2;
        let actual = fuel_required(14);
        assert_eq!(expected, actual);
        let expected = 654;
        let actual = fuel_required(1969);
        assert_eq!(expected, actual);
        let expected = 33583;
        let actual = fuel_required(100756);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_recursive_fuel_required() {
        let expected = 2;
        let actual = recursive_fuel_required(14);
        assert_eq!(expected, actual);
        let expected = 966;
        let actual = recursive_fuel_required(1969);
        assert_eq!(expected, actual);
        let expected = 50346;
        let actual = recursive_fuel_required(100756);
        assert_eq!(expected, actual);
    }

}