use log::debug;

pub fn problem1(input: &String) -> i32 {
    let map: Vec<Vec<char>> = input.lines().map(| x | {
       x.chars().collect::<Vec<char>>()
    }).collect();
    debug!("Size of map, x: {}, y: {}",map[0].len(), map.len());
    let mut x = 0;
    let mut y = 0;
    let x_asc = 3;
    let y_asc = 1;
    let mut hit_tree_count = 0;
    while y < map.len()  {
        if hit_tree(&map, (x,y) ) {
            hit_tree_count += 1;
        }
        (x, y) = new_coords(&map, (x,y), (x_asc, y_asc));
    }
    return hit_tree_count;
}

pub fn problem2(input: &String) -> i64 {
    let map: Vec<Vec<char>> = input.lines().map(| x | {
        x.chars().collect::<Vec<char>>()
     }).collect();
     debug!("Size of map, x: {}, y: {}",map[0].len(), map.len());
     let mut total_tree_hits: Vec<i64> = Vec::new();
     // Right 1, Down 1
     let mut x = 0;
     let mut y = 0;
     let x_asc = 1;
     let y_asc = 1;
     let mut hit_tree_count = 0;
     while y < map.len()  {
         if hit_tree(&map, (x,y) ) {
             hit_tree_count += 1;
         }
         (x, y) = new_coords(&map, (x,y), (x_asc, y_asc));
     }
     total_tree_hits.push(hit_tree_count);
     debug!("Hit trees {hit_tree_count}");
     // Right 3, Down 1
     x = 0;
     y = 0;
     let x_asc = 3;
     let y_asc = 1;
     hit_tree_count = 0;
     while y < map.len()  {
         if hit_tree(&map, (x,y) ) {
             hit_tree_count += 1;
         }
         (x, y) = new_coords(&map, (x,y), (x_asc, y_asc));
     }
     total_tree_hits.push(hit_tree_count);
     debug!("Hit trees {hit_tree_count}");
     // Right 5, Down 1
     x = 0;
     y = 0;
     let x_asc = 5;
     let y_asc = 1;
     hit_tree_count = 0;
     while y < map.len()  {
         if hit_tree(&map, (x,y) ) {
             hit_tree_count += 1;
         }
         (x, y) = new_coords(&map, (x,y), (x_asc, y_asc));
     }
     total_tree_hits.push(hit_tree_count);
     debug!("Hit trees {hit_tree_count}");
     // Right 7, Down 1
     x = 0;
     y = 0;
     let x_asc = 7;
     let y_asc = 1;
     hit_tree_count = 0;
     while y < map.len()  {
         if hit_tree(&map, (x,y) ) {
             hit_tree_count += 1;
         }
         (x, y) = new_coords(&map, (x,y), (x_asc, y_asc));
     }
     total_tree_hits.push(hit_tree_count);
     debug!("Hit trees {hit_tree_count}");
     // Right 1, Down 2
     x = 0;
     y = 0;
     let x_asc = 1;
     let y_asc = 2;
     hit_tree_count = 0;
     while y < map.len()  {
         if hit_tree(&map, (x,y) ) {
             hit_tree_count += 1;
         }
         (x, y) = new_coords(&map, (x,y), (x_asc, y_asc));
     }
     total_tree_hits.push(hit_tree_count);
     debug!("Hit trees {hit_tree_count}");
     return total_tree_hits.iter().product();
}

fn new_coords(map: &Vec<Vec<char>>, (x, y): (usize, usize), (x_asc, y_asc): (usize, usize) ) -> (usize, usize) {
    // Find new x
    let new_x;
    let new_y;
    if x + x_asc > map[y].len() - 1 {
        new_x = x + x_asc - map[y].len();
    } else {
        new_x = x + x_asc;
    }
    if y + y_asc > map.len() {
        new_y = map.len();
    } else {
        new_y = y + y_asc;
    }
    debug!("New coords {new_x}, {new_y}");
    return (new_x, new_y);
}

fn hit_tree(map: &Vec<Vec<char>>, (x, y): (usize, usize)) -> bool {
    return map[y][x] == '#';
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
        let input = fs::read_to_string("data/sample/day_03.txt").expect("Data file doesn't exist!");
        let expected = 7;
        assert_eq!(problem1(&input), expected);
    }

    #[test]
    fn test_problem2() {
        let input = fs::read_to_string("data/sample/day_03.txt").expect("Data file doesn't exist!");
        let expected = 336;
        assert_eq!(problem2(&input), expected);
    }
}