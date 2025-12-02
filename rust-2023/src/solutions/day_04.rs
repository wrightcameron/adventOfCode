use log::debug;

struct Scratch {
    id: i32,
    copies: i32,
    winning: Vec<i32>,
    numbers: Vec<i32>,
}

pub fn problem1(input: &String) -> i32 {
    let total_points: Vec<i32> = input
        .lines()
        .map(|line| {
            let (_before, after) = line.split_once(": ").unwrap();
            let (winning, your_numbers) = after.split_once(" | ").unwrap();
            let winning = winning
                .trim()
                .replace("  ", " ")
                .split(" ")
                .map(|x| x.parse().unwrap())
                .collect::<Vec<i32>>();
            let your_numbers = your_numbers
                .trim()
                .replace("  ", " ")
                .split(" ")
                .map(|x| x.parse().unwrap())
                .collect::<Vec<i32>>();
            debug!("Winning numbers are {:?}", winning);
            debug!("Your numbers are {:?}", your_numbers);
            let mut points = 0;
            for num in winning {
                if your_numbers.contains(&num) {
                    if points < 1 {
                        points += 1;
                    } else {
                        points = points << 1;
                    }
                    debug!("Match!!! {num}, current points are {points}");
                }
            }
            debug!("Winning points are {points}");
            return points;
        })
        .collect();
    return total_points.iter().sum();
}

pub fn problem2(input: &String) -> i32 {
    let mut scratchs: Vec<Scratch> = input
        .lines()
        .map(|line| {
            let (before, after) = line.split_once(": ").unwrap();
            let before = before.strip_prefix("Card ").unwrap();
            let id = before.trim().parse().unwrap();
            let (winning, your_numbers) = after.split_once(" | ").unwrap();
            let winning = winning
                .trim()
                .replace("  ", " ")
                .split(" ")
                .map(|x| x.parse().unwrap())
                .collect::<Vec<i32>>();
            let numbers = your_numbers
                .trim()
                .replace("  ", " ")
                .split(" ")
                .map(|x| x.parse().unwrap())
                .collect::<Vec<i32>>();
            debug!("Winning numbers are {:?}", winning);
            debug!("Your numbers are {:?}", numbers);
            Scratch {
                id,
                copies: 1,
                winning,
                numbers,
            }
        })
        .collect();
    for i in 0..scratchs.len() {
        let scratch = &scratchs[i];
        let points = count_points(&scratch.winning, &scratch.numbers);
        let copies = scratch.copies;
        for j in 0..points {
            let scratch2 = &mut scratchs[j as usize + i + 1];
            scratch2.copies += 1 * copies;
        }
        debug!(
            "Card {}, copies {}, points {}",
            scratchs[i].id, scratchs[i].copies, points
        )
    }
    return scratchs.iter().map(|x| x.copies).sum();
}

fn count_points(winning_nums: &Vec<i32>, numbers: &Vec<i32>) -> i32 {
    let mut points = 0;
    for num in winning_nums {
        if numbers.contains(&num) {
            points += 1;
        }
    }
    return points;
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
        let input = fs::read_to_string("data/sample/day_04.txt").expect("Data file doesn't exist!");
        let expected = 13;
        assert_eq!(problem1(&input), expected);
        // Actual
        let input = fs::read_to_string("data/day_04.txt").expect("Data file doesn't exist!");
        let expected = get_solution("day04".to_string(), 1);
        assert_eq!(problem1(&input) as i64, expected);
    }

    #[test]
    fn test_problem2() {
        // Sample
        let input = fs::read_to_string("data/sample/day_04.txt").expect("Data file doesn't exist!");
        let expected = 30;
        assert_eq!(problem2(&input), expected);
        // Actual
        let input = fs::read_to_string("data/day_04.txt").expect("Data file doesn't exist!");
        let expected = get_solution("day04".to_string(), 2);
        assert_eq!(problem2(&input) as i64, expected);
    }
}
