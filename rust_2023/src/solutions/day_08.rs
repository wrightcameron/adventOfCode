use std::collections::HashMap;

pub fn problem1(input: &String) -> i64 {
    let (tape, after) = input.split_once("\n\n").unwrap();
    let lines = after.lines();
    let mut cfg = HashMap::new();
    for line in lines{
        let (before, after) = line.split_once("=").unwrap();
        let key = before.trim();
        // let after2 = after2.replace(&['(', ')'][..], "");
        
        let (left, right) = after.split_once(",").unwrap();
        cfg.insert(key, (left.trim().replace("(", ""), right.trim().replace(")", "")) );
    }
    // println!("{:?}", cfg["AAA"]);
    let mut count = 0;
    let mut current_key = "AAA";
    // Get to the end of the tape
   
    while current_key != "ZZZ" {
        let (new_count, new_current_key) = iterate_through_tape(tape.chars().collect(), &cfg, count, current_key);
        count = new_count;
        current_key = new_current_key;
        println!("Current state {}, the count {}", current_key, count);
    }
    // If we got to the end of the tape, but not at ZZZ
    println!("Current state {}, the count {}", current_key, count);
    return count;
}

fn iterate_through_tape<'a>(tape: Vec<char>, map: &'a HashMap<&'a str, (String, String) >, count: i64, current_key: &'a str) -> (i64, &'a str) {
    let mut count = count;
    let mut current_key = current_key;
    for t in tape {
        count += 1;
        if t == 'L' {
            current_key = &map[current_key].0;
        } else {
            current_key = &map[current_key].1;
        }
        if current_key == "ZZZ" {
            break
        }
    }
    return (count, current_key);
}

pub fn problem2(input: &String) -> i64 {
    // TODO Final solution will require Least Common Multiple.
    return 0;
}