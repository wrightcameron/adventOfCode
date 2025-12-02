use std::collections::HashSet;
use log::debug;

/// Count the number of questions from all customs surveys.
pub fn problem1(input: &String) -> i32 {
    let counts = input.split("\n\n").map(| x | {
        let mut seen = HashSet::new();
        // Take all people in group and combine them into one line, easier to find duplicates.
        let mut custom = x.replace("\n", "").chars().collect::<Vec<char>>();
        // Find unique chars in vector https://davidchen.io/blog/remove-duplicates-while-preserving-order-in-rust/
        custom.retain(|question| seen.insert(*question));
        return custom.len() as i32;
    }).collect::<Vec<i32>>();
    return counts.iter().sum();
}

/// Count the number of questions from all customs surveys that all users of group answered.
pub fn problem2(input: &String) -> i32 {
    let counts = input.split("\n\n").map(| x | {
        let group = x.lines().collect::<Vec<&str>>();
        let mut group_iter = group.iter();
        let mut common = group_iter.next().unwrap().chars().collect::<Vec<char>>();
        for person in group {
            let user_questions = person.chars().collect::<Vec<char>>();
                common.retain(| f | user_questions.contains(f))
        }
        debug!("Common questions: {:?}", common);
        if common.len() == 0 {
            return 0;
        } else {
            return common.len() as i32;
        }

    }).collect::<Vec<i32>>();
    return counts.iter().sum();
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
        let input = fs::read_to_string("data/sample/day_06.txt").expect("Data file doesn't exist!");
        let expected = 11;
        assert_eq!(problem1(&input), expected);
        //Actual
        let input = fs::read_to_string("data/day_06.txt").expect("Data file doesn't exist!");
        let expected = get_solution("day06".to_string(), 1);
        assert_eq!(problem1(&input) as i64, expected);
    }

    #[test]
    fn test_problem2() {
        // Sample
        let input = fs::read_to_string("data/sample/day_06.txt").expect("Data file doesn't exist!");
        let expected = 6;
        assert_eq!(problem2(&input), expected);
        // Actual
        let input = fs::read_to_string("data/day_06.txt").expect("Data file doesn't exist!");
        let expected = get_solution("day06".to_string(), 2);
        assert_eq!(problem1(&input) as i64, expected);
    }
}
