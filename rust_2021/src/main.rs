mod solutions;

use std::fs;

fn main() {
    run_day_01();
}

fn run_day_01(){
    let input = fs::read_to_string("data/day_01.txt")
        .expect("Data file doesn't exist!");
    let mut output = solutions::day_01::check_depth_increase(&input);
    println!("Part 1: Depth is \n{output}");
    output = solutions::day_01::check_depth_sliding_window(&input);
    println!("Part 1: Depth is \n{output}");
}
