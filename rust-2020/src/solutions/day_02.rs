/// Represents a password
struct Password {
    fewest: i32,
    most: i32,
    character: char,
    password: String,
}

/// Represents a password version 2
struct PasswordV2 {
    start: i32,
    end: i32,
    character: char,
    password: String,
}

/// Find valid password.
///
/// Scan through input, with each line a password contains
/// info on a repeating char and at least or at most it should repeat.
/// Check if all passwords meet there own criteria.
pub fn problem1(input: &String) -> i32 {
    let passwords: Vec<Password> = input
        .lines()
        .map(|x| {
            let (before, password) = x.split_once(": ").unwrap();
            let (before, character) = before.split_once(" ").unwrap();
            let (fewest, most) = before.split_once("-").unwrap();
            Password {
                fewest: fewest.parse().unwrap(),
                most: most.parse().unwrap(),
                character: character.chars().next().unwrap(),
                password: password.to_string(),
            }
        })
        .collect();
    return passwords
        .iter()
        .map(|x| is_valid(x))
        .filter(|x| *x == true)
        .count() as i32;
}

/// Check valid password.
///
/// Scan through input, with each line a password contains
/// info on a repeating char and if the char only shows up in one of two spots.
/// If it shows up in one its valid, none or both invalid.
pub fn problem2(input: &String) -> i32 {
    let passwords: Vec<PasswordV2> = input
        .lines()
        .map(|x| {
            let (before, password) = x.split_once(": ").unwrap();
            let (before, character) = before.split_once(" ").unwrap();
            let (fewest, most) = before.split_once("-").unwrap();
            PasswordV2 {
                start: fewest.parse().unwrap(),
                end: most.parse().unwrap(),
                character: character.chars().next().unwrap(),
                password: password.to_string(),
            }
        })
        .collect();
    passwords
        .iter()
        .map(|x| {
            let start: usize = x.start as usize - 1;
            let end: usize = x.end as usize - 1;
            let start_char = x.password.chars().nth(start).unwrap();
            let end_char = x.password.chars().nth(end).unwrap();
            return (start_char == x.character && end_char != x.character)
                || (start_char != x.character && end_char == x.character);
        })
        .filter(|x| *x == true)
        .count() as i32
}

/// Check if password is valid
///
/// Check if occurances of important char is greater than fewest and less than most.
/// Both set by Password struct
fn is_valid(password: &Password) -> bool {
    let occurances = password
        .password
        .chars()
        .filter(|x| *x == password.character)
        .count();
    //TODO Figure out how to compare i32 into with usize without casting
    return occurances >= password.fewest as usize && occurances <= password.most as usize;
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::get_solution;
    use std::fs;

    //Arrange
    //Act
    //Assert
    #[test]
    fn test_problem1() {
        // Sample
        let input = fs::read_to_string("data/sample/day_02.txt").expect("Data file doesn't exist!");
        let expected = 2;
        assert_eq!(problem1(&input), expected);
        //Actual
        let input = fs::read_to_string("data/day_02.txt").expect("Data file doesn't exist!");
        let expected = get_solution("day02".to_string(), 1);
        assert_eq!(problem1(&input) as i64, expected);
    }

    #[test]
    fn test_problem2() {
        // Sample
        let input = fs::read_to_string("data/sample/day_02.txt").expect("Data file doesn't exist!");
        let expected = 1;
        assert_eq!(problem2(&input), expected);
        // Actual
        let input = fs::read_to_string("data/day_02.txt").expect("Data file doesn't exist!");
        let expected = get_solution("day02".to_string(), 2);
        assert_eq!(problem2(&input) as i64, expected);
    }
}
