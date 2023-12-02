use std::collections::HashMap;

pub fn problem1(input: &String) -> i32 {
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

    // let mut total_sack: Vec<char> = Vec::new();
    let mut sum = 0;
    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for line in input.lines(){
        println!("{line}");
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

    return sum;
}

pub fn problem2(_input: &Vec<i32>) -> i32 {
    return 0;
}

fn find_duplicates(rucksack: &Vec<char>) -> Vec<char>{
    //Find size of rucksack
    let rucksack_size = rucksack.len();
    let mut sack: Vec<char> = Vec::new();
    //Split rucksack into right and left compartments
    let right = &rucksack[0..rucksack_size/2];
    let left = &rucksack[rucksack_size/2..rucksack_size];
    for x in right.iter() {
        if sack.contains(x) {
            continue;
        }
        //Compare contents of right with left, try to remove duplicates first
        for y in left.iter() {
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