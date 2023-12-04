use log::debug;

struct Scratch {
    id: i32,
    copies: i32,
    winning: Vec<i32>,
    numbers: Vec<i32>,
}

pub fn problem1(input: &String) -> i32 {
    let total_points: Vec<i32> = input
        .lines()
        .map(|line| {
            let (before, after) = line.split_once(": ").unwrap();
            let _before = before.strip_prefix("Card ").unwrap();
            // let id = before.parse().unwrap();
            let (winning, your_numbers) = after.split_once(" | ").unwrap();
            let winning = winning.trim().replace("  ", " ").split(" ").map(|x | x.parse().unwrap()).collect::<Vec<i32>>();
            let your_numbers = your_numbers.trim().replace("  ", " ").split(" ").map(|x | x.parse().unwrap()).collect::<Vec<i32>>();
            debug!("Winning numbers are {:?}", winning);
            debug!("Your numbers are {:?}", your_numbers);
            let mut points = 0;
            for num in winning {
                if your_numbers.contains(&num) {
                    if points < 1 {
                        points += 1;
                    } else {
                        points = points << 1;
                    }
                    debug!("Match!!! {num}, current points are {points}");
                }
            }
            debug!("Winning points are {points}");
            return points;
        })
        .collect();
    return total_points.iter().sum();
}

pub fn problem2(input: &String) -> i32 {
    let scratchs: Vec<Scratch> = input
        .lines()
        .map(|line| {
            let (before, after) = line.split_once(": ").unwrap();
            let _before = before.strip_prefix("Card ").unwrap();
            let id = before.parse().unwrap();
            let (winning, your_numbers) = after.split_once(" | ").unwrap();
            let winning = winning.trim().replace("  ", " ").split(" ").map(|x | x.parse().unwrap()).collect::<Vec<i32>>();
            let numbers = your_numbers.trim().replace("  ", " ").split(" ").map(|x | x.parse().unwrap()).collect::<Vec<i32>>();
            debug!("Winning numbers are {:?}", winning);
            debug!("Your numbers are {:?}", numbers);
            Scratch {id, copies: 1, winning, numbers} 
        })
        .collect();
    // for (index, scratch) in scratchs.iter().enumerate() {
    //     let points = count_points(&scratch.winning, &scratch.numbers);
    //     for i in 0..points {
    //         let true_index = i + index as i32;
    //         let fut_scratch: &mut Scratch = scratchs.iter_mut().filter(| x | x.id == true_index).next().unwrap();
    //         fut_scratch.copies += 1;
    //         println!("Adding 1 copy to Card {}", scratch.id)
    //     }
    // }
    for i in 0..scratchs.len() {
        let scratch = &scratchs[i];
        let points = count_points(&scratch.winning, &scratch.numbers);
        for j in 0..points + points {
            
        }
        println!("Points are {points}")
    }
    return 0;
}

fn count_points(winning_nums: &Vec<i32>, numbers: &Vec<i32>) -> i32 {
    let mut points = 0;
    for num in winning_nums {
        if numbers.contains(&num) {
            points += 1;
        }
    }
    return points;
}

// fn add_copies(scratchs: &Vec<Scratch>, current_card: i32, points: i32) {
//     for i in 0..points {
//         let true_index = i + current_card;
//         let scratch: &Scratch = scratchs.iter().filter(| x | x.id == true_index).next().unwrap();
//         scratch.copies += 1;
//         println!("Adding 1 copy to Card {}", scratch.id)
//     }
// }
