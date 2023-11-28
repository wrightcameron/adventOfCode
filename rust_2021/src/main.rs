#![allow(dead_code)]

mod solutions;

use std::fs;

fn main() {
    run_day_03();
}

fn run_day_01(){
    let input = fs::read_to_string("data/day_01.txt")
        .expect("Data file doesn't exist!");
    let mut output = solutions::day_01::check_depth_increase(&input);
    println!("Part 1: Depth is \n{output}");
    output = solutions::day_01::check_depth_sliding_window(&input);
    println!("Part 2: Depth is \n{output}");
}

fn run_day_02(){
    let input = fs::read_to_string("data/day_02.txt")
        .expect("Data file doesn't exist!");
    let mut output = solutions::day_02::dive(&input);
    println!("Part 1: Position is \n{output}");
    output = solutions::day_02::advanced_dive(&input);
    println!("Part 2: Position is \n{output}");
}

fn run_day_03(){
    let input = fs::read_to_string("data/day_03_sample.txt")
        .expect("Data file doesn't exist!");
    let output = solutions::day_03::binary_diagnostic(&input);
    println!("Part 1: Position is {output}");
}
