use log::debug;

struct Password {
    fewest: i32,
    most: i32,
    character: char,
    password: String,
}

struct PasswordV2 {
    start: i32,
    end: i32,
    character: char,
    password: String,
}

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
                password: password.to_string()
            }
        })
        .collect();
    return passwords.iter().map(| x | {
        is_valid(x)
    }).filter(| x | *x == true).count() as i32
}

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
            password: password.to_string()
        }
    })
    .collect();
    passwords.iter().map(| x | {
        let start:usize  = x.start as usize - 1;
        let end: usize = x.end as usize - 1;
        if x.password.chars().nth(start).unwrap() == x.character && x.password.chars().nth(end).unwrap() != x.character {
            return true;
        } else if x.password.chars().nth(start).unwrap() != x.character && x.password.chars().nth(end).unwrap() == x.character {
            return true
        }
        return false;
    }).filter(| x | *x == true).count() as i32
}

fn is_valid(password: &Password) -> bool {
    let occurances = password.password.chars().filter(| x | *x == password.character).count();
    //TODO Change to single line if statement, and figure out how to compare i32 into with usize without casting
    if occurances >= password.fewest as usize && occurances <= password.most as usize {
        return true;
    }
    return false;
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
        let input = fs::read_to_string("data/sample/day_02.txt").expect("Data file doesn't exist!");
        let expected = 2;
        assert_eq!(problem1(&input), expected);
    }

    #[test]
    fn test_problem2() {
        let input = fs::read_to_string("data/sample/day_02.txt").expect("Data file doesn't exist!");
        let expected = 1;
        assert_eq!(problem2(&input), expected);
    }
}
