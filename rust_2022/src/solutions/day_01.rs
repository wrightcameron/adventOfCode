pub fn findElf(input: &String) -> i32 {
    let mut sum = 0;
    let mut maxSum = 0;
    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for line in input.lines() {
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
    return maxSum
}


pub fn findCaloricElf(input: &String) -> i32 {
    let mut sum = 0;
    // Replace this maxSum with a Array containing 3 spots, starting each with 0
    let mut maxSumsArray: [i32; 3] = [0,0,0];
    let mut maxSum = 0;
    
    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for line in input.lines() {
        if line.chars().count() == 0 {
            println!("Finished Elf");
            addNumToArray(sum,&mut maxSumsArray);
            sum = 0;  // Sum should be restarted
        } else {
           sum += line.parse::<i32>().unwrap(); 
        }
    }
    return sumArray(&maxSumsArray);
}

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