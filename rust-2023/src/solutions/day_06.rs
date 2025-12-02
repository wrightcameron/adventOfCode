use log::debug;

pub fn problem1(input: &String) -> i64 {
    let (before, after) = input.split_once("\n").unwrap();
    // Use the .zip() command
    let time: Vec<i64> = before
        .replace("Time: ", "")
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let distance: Vec<i64> = after
        .replace("Distance: ", "")
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut beats: Vec<i64> = Vec::new();
    for (i, element) in time.iter().enumerate() {
        let beat = find_fastest(*element, distance[i]);
        beats.push(beat);
    }
    return beats.iter().product();
}

pub fn problem2(input: &String) -> i64 {
    let (before, after) = input.split_once("\n").unwrap();

    let binding = before.replace(" ", "");
    let (_before, time) = binding.split_once(":").unwrap();

    let binding = after.replace(" ", "");
    let (_before, distance) = binding.split_once(":").unwrap();

    debug!("'{time}'");
    debug!("'{distance}'");

    let time: i64 = time.parse().unwrap();
    let distance: i64 = distance.parse().unwrap();

    return find_fastest(time, distance);
}

fn find_fastest(time: i64, best_distance: i64) -> i64 {
    // let time_trails: Vec<i64> = Vec::new();
    debug!("Time: {time}, Distance to beat {best_distance}");
    let mut beat_count = 0;
    for i in (0..time).rev() {
        let distance = run_race(best_distance, time, i);
        if distance > best_distance {
            beat_count += 1;
        }
    }
    debug!("Beat Count {beat_count}");
    return beat_count;
}

fn run_race(distance: i64, time: i64, button_press: i64) -> i64 {
    let acceleration: i64;
    if button_press == 0 || button_press >= distance {
        acceleration = 0;
    } else {
        acceleration = button_press;
    }
    debug!(
        "Button Press {}, Distance traveled: {}",
        button_press,
        acceleration * (time - button_press)
    );
    return acceleration * (time - button_press);
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
        let input = fs::read_to_string("data/sample/day_06.txt").expect("Data file doesn't exist!");
        let expected = 288;
        assert_eq!(problem1(&input), expected);
        // Actual
        let input = fs::read_to_string("data/day_06.txt").expect("Data file doesn't exist!");
        let expected = get_solution("day06".to_string(), 1);
        assert_eq!(problem1(&input) as i64, expected);
    }

    #[test]
    fn test_problem2() {
        // Sample
        let input = fs::read_to_string("data/sample/day_06.txt").expect("Data file doesn't exist!");
        let expected = 71503;
        assert_eq!(problem2(&input), expected);
        // Actual
        let input = fs::read_to_string("data/day_06.txt").expect("Data file doesn't exist!");
        let expected = get_solution("day06".to_string(), 2);
        assert_eq!(problem2(&input) as i64, expected);
    }
}
