use log::debug;

/// Solution to Advent of Code Problem 1
///
/// Combine the first digit and last digit of each line as strings
/// then add up all lines.
///
/// * `input` - input Text
pub fn problem1(input: &String) -> i32 {
    //Split all lines and convert all lines to strings, collect them in a vector
    let lines = input
        .lines()
        .map(|line| line.chars().filter(|c| c.is_ascii_digit()).collect())
        .collect();
    return add_and_sum_lines(&lines);
}

/// Solution to Advent of Code Problem 2
///
/// Change all the numbers as words to actual digits. Then
/// combine the first digit and last digit of each line as strings
/// then add up all lines.
///
/// * `input` - input Text
pub fn problem2(input: &String) -> i32 {
    // Could replace with one1one, but this is completly broken
    let numbers_to_digits: [(&str, &str); 9] = [
        ("one", "1ne"),
        ("two", "2wo"),
        ("three", "3three"),
        ("four", "4our"),
        ("five", "5ive"),
        ("six", "6ix"),
        ("seven", "7even"),
        ("eight", "8ight"),
        ("nine", "9ine"),
    ];

    let input_vector = input
        .lines()
        .map(|line| {
            let mut new_line = String::from("");
            for c in line.chars() {
                new_line.push(c);
                // Compare the string with all the numbers
                for (key, value) in &numbers_to_digits {
                    new_line = new_line.replace(&*key, &value);
                }
            }
            return new_line;
        })
        .collect();

    return add_and_sum_lines(&input_vector);
}

/// Sum up all lines with the first and last digit.
///
/// For a collection of strings, find the first and last digit, concatenate them.
/// Then add up all the lines.
///
/// * `input` - Vector of strings.
fn add_and_sum_lines(input: &Vec<String>) -> i32 {
    let mut final_sum = 0;

    // There is a trick, if there are more than one digit on the line, we need to get only the first and the last.
    for line in input {
        let mut subset = String::new();
        //Get the first number in a line
        for i in line.chars() {
            // Find the ints in this and put them into a sub group.
            if i.is_digit(10) {
                subset.push(i);
                break;
            }
        }
        for i in line.chars().rev() {
            // Find the ints in this and put them into a sub group.
            if i.is_digit(10) {
                subset.push(i);
                break;
            }
        }
        let number = subset.parse::<i32>().unwrap();
        debug!("Here is the line {line}, here is the number {number}");
        final_sum += number;
    }
    return final_sum;
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
        let input =
            fs::read_to_string("data/sample/day_01_sample.txt").expect("Data file doesn't exist!");
        let expected = 142;
        assert_eq!(problem1(&input), expected);
    }

    #[test]
    fn test_problem2() {
        let input =
            fs::read_to_string("data/sample/day_01_sample2.txt").expect("Data file doesn't exist!");
        let expected = 281;
        assert_eq!(problem2(&input), expected);
    }

    #[test]
    fn test_add_and_sum_lines() {
        let input = fs::read_to_string("data/sample/day_01_sample.txt")
            .expect("Data file doesn't exist!")
            .lines()
            .map(|x| x.to_string())
            .collect();
        let expected = 142;
        assert_eq!(add_and_sum_lines(&input), expected);
    }
}
