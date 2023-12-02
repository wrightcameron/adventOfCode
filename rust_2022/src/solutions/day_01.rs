pub fn problem1(input: &String) -> i32 {
    let mut sum = 0;
    let mut max_sum = 0;
    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for line in input.lines() {
        if line.chars().count() == 0 {
            println!("Sum {}", sum);
            if sum > max_sum {
                max_sum = sum;
            }
            sum = 0;  // Sum should be restarted 
        } else {
           sum += line.parse::<i32>().unwrap(); 
        }
    }
    return max_sum
}


pub fn problem2(input: &String) -> i32 {
    let mut sum = 0;
    // Replace this maxSum with a Array containing 3 spots, starting each with 0
    let mut max_sums_array: [i32; 3] = [0,0,0];
    // let max_sum: i32 = 0;
    
    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for line in input.lines() {
        if line.chars().count() == 0 {
            println!("Finished Elf");
            add_num_to_array(sum,&mut max_sums_array);
            sum = 0;  // Sum should be restarted
        } else {
           sum += line.parse::<i32>().unwrap(); 
        }
    }
    return sum_array(&max_sums_array);
}

fn add_num_to_array(n: i32, arr: &mut [i32]) {
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

fn sum_array(arr: &[i32]) -> i32 {
    let mut max_sum = 0;
    for i in 0..arr.len() {
        max_sum += arr[i];
    }
    return max_sum;
}