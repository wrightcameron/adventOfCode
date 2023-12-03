pub fn problem1(input: &String) -> i32 {
    let elves = input.split("\n\n").map(| elf |  {
        elf.split("\n").map(| food | food.parse::<i32>().unwrap() ).sum::<i32>()
    }).max().unwrap();
    return elves;
}

pub fn problem2(input: &String) -> i32 {
    let elves = input.split("\n\n").map(| elf |  {
        elf.split("\n").map(| food | food.parse::<i32>().unwrap() ).sum::<i32>()
    });
    // Replace this maxSum with a Array containing 3 spots, starting each with 0
    let mut max_sums: [i32; 3] = [0,0,0];
    // let max_sum: i32 = 0;
    
    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for elf in elves {
        if elf < max_sums[0]{
            continue;
        }else if elf <= max_sums[1] {
            max_sums[0] = elf;
       } else if elf <= max_sums[2] {
        max_sums[0] = max_sums[1];
        max_sums[1] = elf;
        } else {
            max_sums[0] = max_sums[1];
            max_sums[1] = max_sums[2];
           max_sums[2] = elf;
        }
    }
    return max_sums.iter().sum();
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
        let expected = 24000;
        assert_eq!(problem1(&input), expected);
    }

    #[test]
    fn test_problem2() {
        let input =
            fs::read_to_string("data/sample/day_01.txt").expect("Data file doesn't exist!");
        let expected = 45000;
        assert_eq!(problem2(&input), expected);
    }
}