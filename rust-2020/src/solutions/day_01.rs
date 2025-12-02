use log::debug;

/// Find two entires that sum to 2020.
///
/// In the input each line is a number.  Loop through all lines and find
/// the two lines that equal 2020.
pub fn problem1(input: &String) -> i32 {
    // TODO Figure how to do this functionaly, with two iterators of the same vec.
    let expense = input
        .lines()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i32>>();
    for i in expense.iter() {
        for j in expense.iter() {
            debug!("Sum is {}", i + j);
            if i + j == 2020 {
                return i * j;
            }
        }
    }
    return 0;
}

/// Find three entries that sum to 2020
///
/// In the input each line is a number.  Loop through all lines and find
/// the three lines that equal 2020.
pub fn problem2(input: &String) -> i32 {
    let expense = input
        .lines()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i32>>();
    for x in expense.iter() {
        if *x > 2020 {
            continue;
        }
        for y in expense.iter() {
            if x + y > 2020 {
                continue;
            }
            for z in expense.iter() {
                if x + y + z == 2020 {
                    return x * y * z;
                }
            }
        }
    }
    return 0;
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
        let expected = 514579;
        assert_eq!(problem1(&input), expected);
        //Actual
        let input = fs::read_to_string("data/day_01.txt").expect("Data file doesn't exist!");
        let expected = get_solution("day01".to_string(), 1);
        assert_eq!(problem1(&input) as i64, expected);
    }

    #[test]
    fn test_problem2() {
        // Sample
        let input = fs::read_to_string("data/sample/day_01.txt").expect("Data file doesn't exist!");
        let expected = 241861950;
        assert_eq!(problem2(&input), expected);
        // Actual
        let input = fs::read_to_string("data/day_01.txt").expect("Data file doesn't exist!");
        let expected = get_solution("day01".to_string(), 2);
        assert_eq!(problem2(&input) as i64, expected);
    }
}
