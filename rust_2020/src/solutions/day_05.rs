use log::debug;

pub fn problem1(input: &String) -> i32 {
    return input.lines().map(| x | x.chars().collect::<Vec<char>>() ).map(| x | {
        let seat_row = find_seat_row(&x, 0, 0, 127);
        let seat = find_seat(&x, 0, 0, 7);
        let seat_id = seat_row * 8 + seat;
        debug!("Seat ID: {seat_id}");
        seat_id
    }).max().unwrap();
}

// pub fn problem2(input: &String) -> i32 {
//     return 0;
// }

fn find_seat_row(boarding_pass: &Vec<char>, index: usize, lower_bound: i32, upper_bound: i32) -> i32 {
    debug!("Range: {lower_bound} -> {upper_bound}");
    if index == 6 {
        if boarding_pass[index] == 'F' {
            return lower_bound;
        } else {
            return upper_bound;
        }
    }
    let delta = ((upper_bound - lower_bound) as f64 / 2.0).ceil() as i32;
    if boarding_pass[index] == 'F' {
        return find_seat_row(boarding_pass, index + 1, lower_bound, upper_bound - delta);
    }else{
        return find_seat_row(boarding_pass, index + 1, lower_bound + delta, upper_bound);
    }
}

fn find_seat(boarding_pass: &Vec<char>, index: usize, lower_bound: i32, upper_bound: i32) -> i32 {
    debug!("Seat Range: {lower_bound} -> {upper_bound}");
    if index == 2 {
        if boarding_pass[7 + index] == 'R' {
            return lower_bound;
        } else {
            return upper_bound;
        }
    }
    let delta = ((upper_bound - lower_bound) as f64 / 2.0).floor() as i32;
    if boarding_pass[7 + index] == 'R' {
        return find_seat(boarding_pass, index + 1, lower_bound, upper_bound - delta);
    }else{
        return find_seat(boarding_pass, index + 1, lower_bound + delta, upper_bound);
    }
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
        let input = fs::read_to_string("data/sample/day_05.txt").expect("Data file doesn't exist!");
        let expected = 820;
        assert_eq!(problem1(&input), expected);
    }

    // #[test]
    // fn test_problem2() {
    //     let input = fs::read_to_string("data/sample/day_04.txt").expect("Data file doesn't exist!");
    //     let expected = 336;
    //     assert_eq!(problem2(&input), expected);
    // }
}