use std::fs::File;
use std::io::{BufRead, BufReader};

fn addNumToArray(n: i32, arr: &mut [i32]) {
    println!("Number to add: {}, Array {:?}", n, arr); 
    //TODO Get this refactored into loop
    if n < arr[0]{
        return;
    }else if n <= arr[1] {
        arr[0] = n;
   } else if n <= arr[2] {
       arr[0] = arr[1];
       arr[1] = n;
    } else {
       arr[0] = arr[1];
       arr[1] = arr[2];
       arr[2] = n;
    }
}

fn sumArray(arr: &[i32]) -> i32 {
    let mut maxSum = 0;
    for i in 0..arr.len() {
        maxSum += arr[i];
    }
    return maxSum;
}

fn findElf(){
    let filename = "./input/problem1.txt";
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut sum = 0;
    // Replace this maxSum with a Array containing 3 spots, starting each with 0
    let mut maxSumsArray: [i32; 3] = [0,0,0];
    let mut maxSum = 0;
    
    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        if line.chars().count() == 0 {
            println!("Finished Elf");
            addNumToArray(sum,&mut maxSumsArray);
            sum = 0;  // Sum should be restarted
        } else {
           sum += line.parse::<i32>().unwrap(); 
        }
    }

    println!("Largest Sum Calories {}", sumArray(&maxSumsArray));
}

fn main() {
    findElf();
}
