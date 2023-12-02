use log::debug;
use std::collections::HashMap;

/// Solution to Advent of Code Problem 1
///
/// Combine the first digit and last digit of each line as strings
/// then add up all lines.
///
/// * `input` - input Text
pub fn problem1(input: &String) -> i32 {
    // TODO Add regex that removes non numbers
    // TODO Parse all the values, find the ints, ignore the strings
    //Split all lines and convert all lines to strings, collect them in a vector
    let lines = input
        .split("\n")
        .map(|depth| depth.parse::<String>().expect("Error parsing line"))
        .collect::<Vec<String>>();
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
    let mut numbers_to_digits = HashMap::new();
    numbers_to_digits.insert("one".to_string(), "1ne".to_string());
    numbers_to_digits.insert("two".to_string(), "2wo".to_string());
    numbers_to_digits.insert("three".to_string(), "3hree".to_string());
    numbers_to_digits.insert("four".to_string(), "4our".to_string());
    numbers_to_digits.insert("five".to_string(), "5ive".to_string());
    numbers_to_digits.insert("six".to_string(), "6ix".to_string());
    numbers_to_digits.insert("seven".to_string(), "7even".to_string());
    numbers_to_digits.insert("eight".to_string(), "8ight".to_string());
    numbers_to_digits.insert("nine".to_string(), "9ine".to_string());

    let mut input_vector = input
        .split("\n")
        .map(|depth| depth.parse::<String>().expect("Error parsing line"))
        .collect::<Vec<String>>();
    // input_vector.pop();  // TODO For some reason on Linux, the last of the input is a new line.

    // let mut let_rename_this_later  = Vec::new();

    // iitermut
    for line in input_vector.iter_mut() {
        // Turn the string into a char for loop, and build the
        let mut new_line = String::from("");

        for c in line.chars() {
            new_line.push(c);
            // Compare the string with all the numbers
            for (key, value) in &numbers_to_digits {
                new_line = new_line.replace(&*key, &value);
            }
        }
        *line = new_line;
    }
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
