/// Solution to Advent of Code Problem 1
///
/// * `input` - input Text
pub fn problem1(input: &str) -> i32 {
    let mut dial = 50;
    let dial_index = input
        .lines()
        .map(|line| {
            let (direction, distance) = line.split_at(1);
            let distance = distance.parse::<i32>().unwrap();
            if direction == "L" {
                dial = (dial - distance) % 100;
            } else if direction == "R" {
                dial = (dial + distance) % 100;
            } else {
                panic!("Didn't parse an L or R!")
            }
            dial
        })
        .filter(|x| *x == 0)
        .count();
    dial_index.try_into().unwrap()
}

/// Solution to Advent of Code Problem 2
///
/// * `input` - input Text
pub fn problem2(input: &str) -> i32 {
    let mut dial = 50;
    let full_rotations = input
        .lines()
        .map(|line| {
            let (direction, distance) = line.split_at(1);
            let mut distance = distance.parse::<i32>().unwrap();
            distance = match direction {
                "L" => -distance,
                "R" => distance,
                _ => panic!("Didn't parse an L or R!"),
            };
            let rotations = if ((distance + dial) / 100) >= 1 {
                (distance + dial) / 100
            } else if (distance + dial) < 0 && distance.abs() >= dial {
                (distance / 100).abs() + 1
            } else {
                0
            };
            dbg!(dial);
            dial = (dial + distance).rem_euclid(100);
            assert!(dial >= 0, "Dial was less than 0, it shouldn't be.");
            dbg!(distance, dial, rotations);
            rotations
        })
        .sum();
    full_rotations
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
        let input = fs::read_to_string("data/sample/day_01.txt").expect("Data file doesn't exist!");
        let expected = 3;
        assert_eq!(problem1(&input), expected);
        // Actual
        let input = fs::read_to_string("data/day_01.txt").expect("Data file doesn't exist!");
        let expected = get_solution("day01".to_string(), 1);
        assert_eq!(problem1(&input) as i64, expected);
    }

    #[test]
    fn test_problem2() {
        // Sample
        let input = fs::read_to_string("data/sample/day_01.txt").expect("Data file doesn't exist!");
        let expected = 6;
        assert_eq!(problem2(&input), expected);
        // Actual
        // 5464 is too low
        // 6122 is too high
        let input = fs::read_to_string("data/day_01.txt").expect("Data file doesn't exist!");
        let expected = get_solution("day01".to_string(), 2);
        assert_eq!(problem2(&input) as i64, expected);
    }
}
