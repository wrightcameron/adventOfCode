use log::debug;
use std::cmp::Ordering;

type Card = char;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

#[derive(Debug)]
struct Hand {
    cards: Vec<char>,
    bid: i32,
    hand_type: HandType,
    card_strength: i32,
}

impl Hand {
    pub fn new(cards: Vec<char>, bid: i32) -> Hand {
        let hand_type = get_hand_type(&cards);
        let card_strength = calc_card_strength(&cards);
        Hand {
            cards,
            bid,
            hand_type,
            card_strength,
        }
    }
}

fn calc_card_strength(cards: &Vec<char>) -> i32 {
    debug!("-----");
    let mut strength = 0;
    let rev: i32 = 5;
    for (index, element) in cards.iter().enumerate() {
        let element_power: i32 = match element {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            _ => element.to_digit(10).unwrap() as i32,
        };
        debug!("Card stregth of {element} is {element_power}");
        let base: i32 = 15;
        let pos = rev - index as i32;
        strength += element_power * base.pow(pos as u32);
        debug!("New strength is {strength}");
    }
    return strength;
}

fn get_hand_type(cards: &Vec<char>) -> HandType {
    let mut unique_cards: Vec<char> = Vec::new();
    let mut strenth = HandType::FiveKind;
    for i in cards {
        if !unique_cards.contains(&i) {
            unique_cards.push(*i)
        }
    }
    if unique_cards.len() == 5 {
        // High Card
        strenth = HandType::HighCard;
    } else if unique_cards.len() == 4 {
        //One pair
        strenth = HandType::OnePair;
    } else if unique_cards.len() == 3 {
        // Two Pair or Three of a kind
        let mut card_count = 0;
        // If there is a count of 3 cards it has to be three of a kind
        for i in unique_cards {
            card_count = cards.iter().filter(|x| **x == i).count();
            if card_count == 3 {
                break;
            }
        }
        if card_count == 3 {
            // Three of a Kind
            strenth = HandType::ThreeKind;
        } else {
            // Two Pair
            strenth = HandType::TwoPair;
        }
    } else if unique_cards.len() == 2 {
        //Full House or Four of a kind
        let mut card_count = 0;
        // If there is a count of 3 cards it has to be three of a kind
        for i in unique_cards {
            card_count = cards.iter().filter(|x| **x == i).count();
            if card_count == 3 {
                break;
            }
        }
        if card_count == 3 {
            // Full House
            strenth = HandType::FullHouse;
        } else {
            // Four of a kind
            strenth = HandType::FourKind;
        }
    } else if unique_cards.len() == 1 {
        // Five of a kind
        strenth = HandType::FiveKind;
    }
    return strenth;
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            // If equal, we need to look at the cards.
            // std::cmp::Ordering::Equal => self.cards.cmp(&other.cards),
            // Don't know how to compare vec of chars and return Ordering, so calc card
            // Strength ahead of time, compare that.
            std::cmp::Ordering::Equal => self.card_strength.cmp(&other.card_strength),
            // If not equal, the kind was enough to decide.
            x => x,
        }
    }
}

// Ord requires this implemented
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Hand {}

// PartialOrd requires this implemented
impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards && self.bid == other.bid
    }
}

pub fn problem1(input: &String) -> i32 {
    let mut hands: Vec<Hand> = input
        .lines()
        .map(|x| {
            let (before, after) = x.split_once(" ").unwrap();
            Hand::new(before.chars().collect(), after.parse().unwrap())
        })
        .collect();
    hands.sort();
    for i in hands.iter() {
        debug!(
            "Hand: {:?}, bid {}, type {:?}, card strength {}",
            i.cards, i.bid, i.hand_type, i.card_strength
        );
    }
    // Calculate winnings
    let mut total_winnings = 0;
    for (index, element) in hands.iter().enumerate() {
        total_winnings += (index as i32 + 1) * element.bid;
    }
    return total_winnings;
}

// pub fn problem2(input: &String) -> i64 {
//     return 0;
// }

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use serde_json;
    use std::fs;

    #[derive(Deserialize, Debug)]
    struct Solution {
        id: String,
        first: i64,
        second: i64,
    }

    fn get_solution(day: String, problem: i8) -> i64 {
        let json_string =
            fs::read_to_string("data/solutions.json").expect("JSON file doesn't exist!");
        let json: Vec<Solution> =
            serde_json::from_str(&json_string).expect("JSON was not well-formatted");
        let solution = json.iter().find(|x| x.id == day).unwrap();
        return if problem == 1 {
            solution.first
        } else {
            solution.second
        };
    }

    //Arrange
    //Act
    //Assert

    #[test]
    fn test_problem1() {
        // Sample
        let input = fs::read_to_string("data/sample/day_07.txt").expect("Data file doesn't exist!");
        let expected = 6440;
        assert_eq!(problem1(&input), expected);
        // Actual
        let input = fs::read_to_string("data/day_07.txt").expect("Data file doesn't exist!");
        let expected = get_solution("day07".to_string(), 1);
        assert_eq!(problem1(&input) as i64, expected);
    }

    // #[test]
    // fn test_problem2() {
    //     // Sample
    //     let input =
    //         fs::read_to_string("data/sample/day_07.txt").expect("Data file doesn't exist!");
    //     let expected = 71503;
    //     assert_eq!(problem2(&input), expected);
    //     // Actual
    //     let input = fs::read_to_string("data/day_07.txt").expect("Data file doesn't exist!");
    //     let expected = get_solution("day07".to_string(), 2);
    //     assert_eq!(problem2(&input) as i64, expected);
    // }
}
