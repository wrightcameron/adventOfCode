use std::fs::File;
use std::io::{BufRead, BufReader};

fn findElf() {
    let filename = "./input/problem1.txt";
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut sum = 0;
    let mut maxSum = 0;
    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        if line.chars().count() == 0 {
            println!("Sum {}", sum);
            if sum > maxSum {
                maxSum = sum;
            }
            sum = 0;  // Sum should be restarted 
        } else {
           sum += line.parse::<i32>().unwrap(); 
        }
    }

   println!("Largest Sum Calories {}", maxSum); 

}

fn main() {
    findElf();
}
