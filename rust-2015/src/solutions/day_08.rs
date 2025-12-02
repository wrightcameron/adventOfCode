use regex::Regex;

/// Problem 1,
pub fn problem1(input: &str) -> i32 {
    input.lines().map(| line | {
        let line = line.trim();
        let escaped_string = strip_escape_tokens(line);
        let literal_count = line.len() as i32;
        let memory_count = escaped_string.len() as i32;
        literal_count - memory_count
    }).sum()
}

fn strip_escape_tokens(raw_string: &str) -> String {
    let re = Regex::new(r#"\\x[0-9a-f]{2}|\\"|\\\\"#).unwrap();
    let mut string = re.replace_all(&raw_string, "x").to_string();
    string.pop();      // remove last
    if string.len() > 0 {
        string.remove(0);  // remove first
    }
    string
}

/// Problem 2,
pub fn problem2(input: &str) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use crate::test_utils::get_solution;

    #[test]
    fn test_string_literal_char_count() {
        let input = r#""""#;
        let expected = 2;
        assert_eq!(input.len() as i32, expected);
        let input = r#""abc""#;
        let expected = 5;
        assert_eq!(input.len() as i32, expected);
        let input = r#""aaa\"aaa""#;
        let expected = 10;
        assert_eq!(input.len() as i32, expected);
        let input = r#""\x27""#;
        let expected = 6;
        assert_eq!(input.len() as i32, expected);
        let input = r#""aa\\\\bb""#;
        let expected = 10;
        assert_eq!(input.len() as i32, expected);
        let input = r#""lhyjky\\m\"pvnm\\xmynpxnlhndmahjl""#;
        let expected = 35;
        assert_eq!(input.len() as i32, expected);
    }

    #[test]
    fn test_strip_escape_tokens() {
        let input = strip_escape_tokens(r#""""#);
        let expected = 0;
        assert_eq!(input.len() as i32, expected);
        let input = strip_escape_tokens(r#""abc""#);
        let expected = 3;
        assert_eq!(input.len() as i32, expected);
        let input = strip_escape_tokens(r#""aaa\"aaa""#);
        let expected = 7;
        assert_eq!(input.len() as i32, expected);
        let input = strip_escape_tokens(r#""\x27""#);
        let expected: i32 = 1;
        assert_eq!(input.len() as i32, expected);
        let input = strip_escape_tokens(r#""aa\\\\bb""#);
        let expected = 6;
        println!("{input}");
        assert_eq!(input.len() as i32, expected);
    }

    #[test]
    fn test_problem1() {
        // Sample
        let input = fs::read_to_string("data/sample/day_08.txt").expect("Data file doesn't exist!");
        let expected = 12;
        assert_eq!(problem1(&input), expected);
        //Actual
        let input = fs::read_to_string("data/day_08.txt").expect("Data file doesn't exist!");
        let expected = get_solution("day08".to_string(), 1);
        assert_eq!(problem1(&input) as i64, expected);
    }

    #[test]
    fn test_problem2() {
        // Sample
        let input = "^v";
        let expected = 3;
        assert_eq!(problem2(input), expected);
        //Actual
        let input = fs::read_to_string("data/day_06.txt").expect("Data file doesn't exist!");
        let expected = get_solution("day06".to_string(), 2);
        assert_eq!(problem2(&input) as i64, expected);
    }

}
