use log::debug;

pub fn problem1(input: &String) -> i32 {
    // TODO Figure how to do this functionaly, with two iterators of the same vec.
    let expense = input.lines().map(| x | x.parse().unwrap()).collect::<Vec<i32>>();
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

pub fn problem2(input: &String) -> i32 {
    let expense = input.lines().map(| x | x.parse().unwrap()).collect::<Vec<i32>>();
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
    use std::fs;

    //Arrange
    //Act
    //Assert

    #[test]
    fn test_problem1() {
        let input =
            fs::read_to_string("data/sample/day_01.txt").expect("Data file doesn't exist!");
        let expected = 514579;
        assert_eq!(problem1(&input), expected);
    }

    #[test]
    fn test_problem2() {
        let input =
            fs::read_to_string("data/sample/day_01.txt").expect("Data file doesn't exist!");
        let expected = 241861950;
        assert_eq!(problem2(&input), expected);
    }

}
