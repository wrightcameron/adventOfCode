use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn find_duplicates(rucksack: &Vec<char>) -> Vec<char>{
    //Find size of rucksack
    let rucksack_size = rucksack.len();
    let mut sack: Vec<char> = Vec::new();
    //Split rucksack into right and left compartments
    let right = &rucksack[0..rucksack_size/2];
    let left = &rucksack[rucksack_size/2..rucksack_size];
    for (i, x) in right.iter().enumerate() {
        if sack.contains(x) {
            continue;
        }
        //Compare contents of right with left, try to remove duplicates first
        for (j, y) in left.iter().enumerate() {
            if x == y && !sack.contains(x) {
                //If a duplicate is found add it to list of chars to pass back
                let copied_x = x.clone();
                sack.push(copied_x);
                break;
            }
        }
    }
    for x in sack.iter(){
        print!("{},",x);
    }
    println!("");
    return sack;
}

fn problem1(input: &str) {
    let alphabet_nums = HashMap::from([
        ('a', 1),
        ('b', 2),
        ('c', 3),
        ('d', 4),
        ('e', 5),
        ('f', 6),
        ('g', 7),
        ('h', 8),
        ('i', 9),
        ('j', 10),
        ('k', 11),
        ('l', 12),
        ('m', 13),
        ('n', 14),
        ('o', 15),
        ('p', 16),
        ('q', 17),
        ('r', 18),
        ('s', 19),
        ('t', 20),
        ('u', 21),
        ('v', 22),
        ('w', 23),
        ('x', 24),
        ('y', 25),
        ('z', 26),
        ('A', 27),
        ('B', 28),
        ('C', 29),
        ('D', 30),
        ('E', 31),
        ('F', 32),
        ('G', 33),
        ('H', 34),
        ('I', 35),
        ('J', 36),
        ('K', 37),
        ('L', 38),
        ('M', 39),
        ('N', 40),
        ('O', 41),
        ('P', 42),
        ('Q', 43),
        ('R', 44),
        ('S', 45),
        ('T', 46),
        ('U', 47),
        ('V', 48),
        ('W', 49),
        ('X', 50),
        ('Y', 51),
        ('Z', 52),
    ]);

    // Open the file in read-only mode (ignoring errors).
    let file = File::open(input).unwrap();
    let reader = BufReader::new(file);

    
    let mut total_sack: Vec<char> = Vec::new();
    let mut sum = 0;
    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        println!("Line: {}, line - {}", _index, line);
        if line.chars().count() == 0 {
            continue;
        }
        let char_vec: Vec<char> = line.chars().collect();
        let sack: Vec<char> = find_duplicates(&char_vec);
        for x in sack.iter() {
                // total_sack.push(*x);
                let value = alphabet_nums[x];
                sum += value;
        }


    }



    

    // for x in total_sack.iter() {
    //     let value = alphabet_nums[x];
    //     sum += value;
    // }

    println!("> {}", sum);

}

fn problem2(input: &str) {

}

fn main() {
    let problem;
    let input;

    let args: Vec<String> = env::args().collect();

    if args.len() >= 3 {
        problem = args[1].parse::<i32>().unwrap();
        input = &args[2];
    } else {
        println!("{} <problem:1|2> <inputPath>", &args[0]);
        std::process::exit(0)
    }

    if problem == 1 {
        problem1(&input);
    } else {
        problem2(&input);
    }
}

