pub fn check_depth_increase(input: &String) -> i32 {
    let mut count = 0;
    let mut previous_depth = i32::MAX;
    for i in input.lines() {
        let depth = i.parse::<i32>().unwrap(); 
        if depth > previous_depth {
            count += 1;
        }
        previous_depth = depth;
    }
    return count;
}


pub fn check_depth_sliding_window(input: &String) -> i32 {
    let mut count = 0;
    let mut previous_sum = i32::MAX;
    let vec = input.split("\n").map(| depth | depth.parse::<i32>().expect("Should only have integers on each line") ).collect::<Vec<i32>>();
    for (i, el) in vec.iter().enumerate() {
        if i == 0 || i == vec.len() - 1 {
            continue;
        }
        let sum = vec[i - 1] + el + vec[i + 1];
        if sum > previous_sum {
            count += 1
        }
        previous_sum  = sum;
    }
    return count;
}