#![warn(unused_mut)]
pub fn binary_diagnostic(input: &String) -> i32 {
    println!("'{}'", input);
    let mut vec = input.split("\n").map(|x| i32::from_str_radix(x, 2).unwrap() ).collect::<Vec<i32>>();
    let mut gamma_rate = calc_gamma_rate(&vec);
    let number_bits = 5;  // TODO Need to have the bit length calculated, sample is 5 the actual is 12
    // // Find bit size of largest number, or just assume for now that its alwasy 5
    // for n in (0..number_bits).rev() {
    //     let mut common_bit = 0;
    //     for i in &vec {
    //         println!("{} => {:05b}", i, i);
    //         // print_type_of(&i);
    //         let bit = i & (1 << n);  // We don't need to shift, just check if the number is not 0
    //         if bit > 0 {
    //             common_bit += 1;
    //         } else {
    //             common_bit -= 1;
    //         }
    //         // println!("Most significant bit is {}", bit);
    //     }
    //     // println!("common_bit is {}", common_bit);
    //     if common_bit > 0 {
    //         gamma_rate = gamma_rate | (1 << n);
    //     } else {
    //         gamma_rate = gamma_rate | (0 << n);
    //     }
    //     // println!("gamma_rate is {:05b}", gamma_rate);
    // }
    let base: i32 = 2;
    let epsilon_rate = !gamma_rate & (base.pow(number_bits) - 1);
    println!("gamma_rate is {}, epsilon_rate is {}", gamma_rate, epsilon_rate);
    return gamma_rate * epsilon_rate;
}

fn calc_gamma_rate(input: &Vec<i32>) -> i32 {
    let mut gamma_rate = 0;
    let number_bits = 5;  // TODO Need to have the bit length calculated, sample is 5 the actual is 12
    // Find bit size of largest number, or just assume for now that its alwasy 5
    for n in (0..number_bits).rev() {
        let mut common_bit = 0;
        for i in input {
            // println!("{} => {:05b}", i, i);
            // print_type_of(&i);
            let bit = i & (1 << n);  // We don't need to shift, just check if the number is not 0
            if bit > 0 {
                common_bit += 1;
            } else {
                common_bit -= 1;
            }
            // println!("Most significant bit is {}", bit);
        }
        // println!("common_bit is {}", common_bit);
        if common_bit > 0 {
            gamma_rate = gamma_rate | (1 << n);
        } else {
            gamma_rate = gamma_rate | (0 << n);
        }
        // println!("gamma_rate is {:05b}", gamma_rate);
    }
    return gamma_rate;
}

fn calc_oxygen_generator_rating(input: &Vec<i32>) -> i32 {
    if input.len() == 1 {
        return input[0];
    }else if input.len() == 0 {
        return 0;
    }
    return 0;
}

fn calc_co2_scrubber_rating(input: &Vec<i32>) -> i32 {
    return 0;
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}