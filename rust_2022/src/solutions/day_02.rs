pub fn problem1(input: &String) -> i32 {
    let mut total_score = 0;
    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for line in input.lines() {
        if line.chars().count() == 0 {
            continue;
        }

        let char_vec: Vec<char> = line.chars().collect();
        let opp = char_vec[0].to_string();
        let you = char_vec[2].to_string();

        //Result
        let mut res = roshambo(&opp, &you);
        if res == -2 || res == 2 {
            res = res * -1;
        }

        let mut score;

        if res > 0 {
            score = 6;
        } else if res == 0 {
            score = 3;
        } else {
            score = 0;
        }

        if you.eq("X") {
            score += 1;
        } else if you.eq("Y") {
            score += 2;
        } else if you.eq("Z") {
            score += 3;
        }else{
            score += 0;
        }

        total_score += score;

    }
    return total_score;
}

pub fn problem2(input: &String) -> i32 {
    let mut total_score = 0;
    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for line in input.lines() {
        if line.chars().count() == 0 {
            continue;
        }

        let char_vec: Vec<char> = line.chars().collect();
        let opp = char_vec[0].to_string();
        let you = char_vec[2].to_string();

        let our_hand = calc_hand(&opp, &you);

        //Result
        let mut res = roshambo(&opp, &our_hand);
        if res == -2 || res == 2 {
            res = res * -1;
        }

        let mut score;

        if res > 0 {
            score = 6;
        } else if res == 0 {
            score = 3;
        } else {
            score = 0;
        }

        if our_hand.eq("X") {
            score += 1;
        } else if our_hand.eq("Y") {
            score += 2;
        } else if our_hand.eq("Z") {
            score += 3;
        }else{
            score += 0;
        }

        total_score += score;

    }
    return total_score;

}

fn roshambo(op: &str, me: &str) -> i32{
    let mut op_num = 0;
    let mut my_num = 0;

    // Convert move into num
    if op.eq("A") {
        op_num = -1;
    }
    else if op.eq("B") {
        op_num = -2;
    }
    else if op.eq("C") {
        op_num = -3;
    }

    // Convert move into num
    if me.eq("X") {
        my_num = 1;
    }
    else if me.eq("Y") {
        my_num = 2;
    }
    else if me.eq("Z") {
        my_num = 3;
    }

    return my_num + op_num;

}


fn calc_hand(op_hand: &str, outcome: &str) -> String{
    let mut desired_hand = String::new();
    //Opponent plays Rock
    if op_hand.eq("A") {
        // I should lose - Scissors
        if outcome.eq("X") {
            desired_hand = String::from("Z");
        }
        // I should draw - Rock
        else if outcome.eq("Y") {
            desired_hand = String::from("X");
        }
        // I should win - Paper
        else if outcome.eq("Z") {
            desired_hand = String::from("Y");
        }
    }
    //Opponent plays Paper
    else if op_hand.eq("B") {
        // I should lose - Rock
        if outcome.eq("X") {
            desired_hand = String::from("X");
        }
        // I should draw - Paper
        else if outcome.eq("Y") {
            desired_hand = String::from("Y");
        }
        // I should win - Scissors
        else if outcome.eq("Z") {
            desired_hand = String::from("Z");
        }
    }
    //Opponent plays Scissors
    else if op_hand.eq("C") {
        // I should lose - Paper
        if outcome.eq("X") {
            desired_hand = String::from("Y");
        }
        // I should draw - Scissors
        else if outcome.eq("Y") {
            desired_hand = String::from("Z");
        }
        // I should win - Rock
        else if outcome.eq("Z") {
            desired_hand = String::from("X");
        }
    }
    return desired_hand;
}