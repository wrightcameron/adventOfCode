use std::cmp;

/// Problem 1,
pub fn problem1(input: &str) -> i32 {
    input.lines().map(| line | {
        let res = line.trim().split('x').collect::<Vec<&str>>();
        assert_eq!(res.len(), 3);
        let length = res[0].parse().unwrap();
        let width = res[1].parse().unwrap();
        let height = res[2].parse().unwrap();
        calc_surface_area(length, width, height) + min_area(length, width, height)
    }).sum()
}

fn calc_surface_area(length: i32, width: i32, height: i32) -> i32{
    (2 * length * width) + (2 * width * height) + (2 * height * length)
}

fn min_area(length: i32, width: i32, height: i32) -> i32{
    cmp::min( cmp::min( length * width, length * height ), width * height )
}

/// Problem 2,
pub fn problem2(input: &str) -> i32 {
    input.lines().map(| line | {
        let mut res = line.trim().split('x').map(| x | x.parse::<i32>().unwrap() ).collect::<Vec<i32>>();
        assert_eq!(res.len(), 3);
        res.sort();
        let length = res[0];
        let width = res[1];
        let height = res[2];
        feet_of_ribbion(length, width, height)
    }).sum()
}

fn feet_of_ribbion(length: i32, width: i32, height: i32) -> i32 {
    let smallest_perimeter = ( 2 * length) + ( 2 * width);
    let volume = length * width * height;
    smallest_perimeter + volume
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use serde_json;
    use std::fs;

    #[derive(Deserialize, Debug)]
    struct Solution {
        id: String,
        first: i64,
        second: i64,
    }

    fn get_solution(day: String, problem: i8) -> i64 {
        let json_string =
            fs::read_to_string("data/solutions.json").expect("JSON file doesn't exist!");
        let json: Vec<Solution> =
            serde_json::from_str(&json_string).expect("JSON was not well-formatted");
        let solution = json.iter().find(|x| x.id == day).unwrap();
        return if problem == 1 {
            solution.first
        } else {
            solution.second
        };
    }

    //Arrange
    //Act
    //Assert

    #[test]
    fn test_problem1() {
        // Sample
        let input = "2x3x4";
        let expected = 58;
        assert_eq!(problem1(input), expected);
        let input = "1x1x10";
        let expected = 43;
        assert_eq!(problem1(input), expected);
        //Actual
        let input = fs::read_to_string("data/day_02.txt").expect("Data file doesn't exist!");
        let expected = get_solution("day02".to_string(), 1);
        assert_eq!(problem1(&input) as i64, expected);
    }

    #[test]
    fn test_problem2() {
        // Sample
        let input = "2x3x4";
        let expected = 34;
        assert_eq!(problem2(input), expected);
        let input = "1x1x10";
        let expected = 14;
        assert_eq!(problem2(input), expected);
        //Actual
        let input = fs::read_to_string("data/day_02.txt").expect("Data file doesn't exist!");
        let expected = get_solution("day02".to_string(), 2);
        assert_eq!(problem2(&input) as i64, expected);
    }

}
