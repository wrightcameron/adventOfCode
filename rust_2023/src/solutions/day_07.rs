
struct Hand {
    cards: Vec<char>,
    bid: i32
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        // TODO This is the compare for Hand
        let (self_hand, self_stregth) = card_strength(&self.cards);
        let (other_hand, other_stregth) = card_strength(&other.cards);
        if self_hand > other_hand {
            // Our hand is stronger
        } else if self_hand < other_hand {
            // Our hand is weaker
        } else if self_stregth > other_stregth {
            // Our hand is stronger
        } else {
            
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // TODO This is the compare for Hand
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
         // TODO This is the compare for Hand
        self.cards == other.cards
    }
}
pub fn problem1(input: &String) -> i64 {
    let (before, after) = input.split_once("\n").unwrap();
    
    return 0;
}

// pub fn problem2(input: &String) -> i64 {
//     return 0;
// }

fn card_strength(cards: &Vec<char>) -> (i32, i32) {
    let unique_cards: Vec<char> = Vec::new();
    let strenth: (i32, i32);
    for i in cards {
        if !unique_cards.contains(i) {
            unique_cards.push(i)
        }
    }
    let first_card_strength = find_strength_of_card(cards[0]);
    
    if unique_cards.len() == 5 {
        // High Card
        strenth = (0, first_card_strength);
    } else if unique_cards.len() == 4 {
        //One pair
        strenth = (1, first_card_strength);
    } else if unique_cards.len() == 3 {
        // Two Pair or Three of a kind
        let card_count: i32 = 0;
        let track_card: char = '1';
        cards.sort().map(| x | {
            if track_card == x{
                card_count += 1;
            }else{
                track_card = x;
                card_count = 0;
            }
        });
        if card_count == 3 {
            // Three of a Kind
            strenth = (2, first_card_strength);
        }else{
             // Two Pair
            strenth = (3, first_card_strength);
        }
    } else if  unique_cards.len() == 2 {
        //Full House or Four of a kind
        let card_count: i32 = 0;
        let track_card: char = '1';
        cards.sort().map(| x | {
            if track_card == x{
                card_count += 1;
            }else{
                track_card = x;
                card_count = 0;
            }
        });
        if card_count == 3 {
            // Full House
            strenth = (4, first_card_strength);
        }else{
             // Four of a kind
            strenth = (5, first_card_strength);
        }
    } else if unique_cards.len() == 1 {
        // Five of a kind
        strenth = (6, first_card_strength);
    }
    return strenth;
}

fn find_strength_of_card(card: char) -> i32 {
    match card {
        'A' => 15,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
         _  => card.to_digit(10).unwrap()
    }
}


