pub fn dive(input: &String) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;
    for i in input.lines() {
        let vec = i.split(" ").map(|x| x.to_string()).collect::<Vec<String>>();
        let command = &vec[0];
        let value = vec[1].parse::<i32>().unwrap();
        if command == "forward"{
            horizontal += value;
        } else if command == "down" {
            depth += value;
        } else if command == "up" {
            depth -= value;
        } else {
            println!("Unknown State");
        }
    }
    return horizontal * depth;
}

pub fn advanced_dive(input: &String) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;
    for i in input.lines() {
        let vec = i.split(" ").map(|x| x.to_string()).collect::<Vec<String>>();
        let command = &vec[0];
        let value = vec[1].parse::<i32>().unwrap();
        if command == "forward"{
            horizontal += value;
            depth += aim * value;
        } else if command == "down" {
            aim += value;
        } else if command == "up" {
            aim -= value;
        } else {
            println!("Unknown State");
        }
    }
    return horizontal * depth;
}