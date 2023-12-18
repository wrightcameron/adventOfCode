use log::debug;

pub fn problem1(input: &String) -> i32 {
    let passports = input.split("\n\n").collect::<Vec<&str>>();
    return passports.iter().map(| x | valid_passport(&x.to_string())).filter(| x | *x == true).count() as i32;
}

// TODO Need to do, should use regex and capture groups
// pub fn problem2(input: &String) -> i32 {
//     return 0;
// }

fn valid_passport(passport: &String) -> bool {
    let valid = passport.contains("byr") && passport.contains("iyr") && passport.contains("eyr") && passport.contains("hgt") && passport.contains("hcl") && passport.contains("ecl") && passport.contains("pid");
    if valid {
        debug!("Passport valid:\n{passport}");
    }
    return valid;
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
        let input = fs::read_to_string("data/sample/day_04.txt").expect("Data file doesn't exist!");
        let expected = 2;
        assert_eq!(problem1(&input), expected);
    }

    // #[test]
    // fn test_problem2() {
    //     let input = fs::read_to_string("data/sample/day_04.txt").expect("Data file doesn't exist!");
    //     let expected = 336;
    //     assert_eq!(problem2(&input), expected);
    // }
}