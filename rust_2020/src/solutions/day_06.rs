use std::collections::HashSet;
use log::debug;

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
    use std::fs;

    //Arrange
    //Act
    //Assert

    #[test]
    fn test_problem1() {
        let input = fs::read_to_string("data/sample/day_06.txt").expect("Data file doesn't exist!");
        let expected = 11;
        assert_eq!(problem1(&input), expected);
    }

    #[test]
    fn test_problem2() {
        let input = fs::read_to_string("data/sample/day_06.txt").expect("Data file doesn't exist!");
        let expected = 6;
        assert_eq!(problem2(&input), expected);
    }
}