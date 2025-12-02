type Coord = (u32, u32);

/// Problem 1,
pub fn problem1(input: &str) -> i32 {
   let mut lights = [[false; 1000]; 1000];
   for line in input.lines() {
        let line_split: Vec<&str> = line.split_whitespace().collect();
        match line_split.len() {
            5 => {
                let (start_x, start_y) = line_split[2].split_once(',').unwrap();
                let start_x : i32 = start_x.parse().unwrap();
                let start_y: i32 = start_y.parse().unwrap();
                
                let (end_x, end_y) = line_split[4].split_once(',').unwrap();
                let end_x : i32 = end_x.parse().unwrap();
                let end_y: i32 = end_y.parse().unwrap();
                let state = match line_split[1] {
                    "on" => true,
                    "off" => false,
                    _ => panic!("Unaccounted state, not on/off")
                };
                set_lights(&mut lights, start_x, start_y, end_x, end_y, state);

            },
            4 => {
                let (start_x, start_y) = line_split[1].split_once(',').unwrap();
                let start_x : i32 = start_x.parse().unwrap();
                let start_y: i32 = start_y.parse().unwrap();
                
                let (end_x, end_y) = line_split[3].split_once(',').unwrap();
                let end_x : i32 = end_x.parse().unwrap();
                let end_y: i32 = end_y.parse().unwrap();
                toggle_lights(&mut lights, start_x, start_y, end_x, end_y);

            },
            _ => panic!("Amount of words should be 4 or 5."),
        }
   }
   count_lights(&lights)
}

fn set_lights(lights: &mut [[bool; 1000]; 1000], start_x: i32, start_y: i32, end_x: i32, end_y: i32, state: bool){
    for row in lights.iter_mut().skip(start_x as usize).take((end_x - start_x + 1) as usize){
        for col in row.iter_mut().skip(start_y as usize).take((end_y - start_y + 1) as usize){
            *col = state;
        }
    }
}

fn toggle_lights(lights: &mut [[bool; 1000]; 1000], start_x: i32, start_y: i32, end_x: i32, end_y: i32){
    for row in lights.iter_mut().skip(start_x as usize).take((end_x - start_x + 1) as usize){
        for col in row.iter_mut().skip(start_y as usize).take((end_y - start_y + 1) as usize){
            *col = ! *col;
        }
    }
}

fn count_lights(lights: &[[bool; 1000]; 1000]) -> i32 {
    // TODO Could use flatten on this 2d array
    let mut lights_on = 0;
    for row in lights.iter() {
        for col in row.iter() {
            if *col == true {
                lights_on += 1;
            }
        }
    }
    lights_on
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
    fn test_problem1() {
        // Sample
        let input = "turn on 0,0 through 999,999";
        let expected = 1000000;
        assert_eq!(problem1(input), expected);
        let input = "toggle 0,0 through 999,0";
        let expected = 1000;
        assert_eq!(problem1(input), expected);
        let input = "turn off 499,499 through 500,500";
        let expected = 0;
        assert_eq!(problem1(input), expected);
        let input = "turn on 0,0 through 999,999\ntoggle 0,0 through 999,0\nturn off 499,499 through 500,500";
        let expected = 998996;
        assert_eq!(problem1(input), expected);
        //Actual
        let input = fs::read_to_string("data/day_06.txt").expect("Data file doesn't exist!");
        let expected = get_solution("day06".to_string(), 1);
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
