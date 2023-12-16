use std::cmp::Ordering;

fn main() {
    println!("part1");
    let input = include_str!("./input1.txt");
    let result = part1(input);

    // 250255509 -> too low
    println!("{result}");
}

// Five of a kind, where all five cards have the same label: AAAAA
// Four of a kind, where four cards have the same label and one card has a different label: AA8AA
// Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
// Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
// Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
// One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
// High card, where all cards' labels are distinct: 23456

#[derive(Debug, PartialEq, Eq, Clone)]
enum HandType {
    HighCard = 0,
    OnePair = 1,
    TwoPair = 2,
    ThreeOfAKind = 3,
    FullHouse = 4,
    FourOfAKind = 5,
    FiveOfAKind = 6,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum CardType {
    Two = 0,
    Three = 1,
    Four = 2,
    Five = 3,
    Six = 4,
    Seven = 5,
    Eight = 6,
    Nine = 7,
    Ten = 8,
    Jack = 9,
    Queen = 10,
    King = 11,
    Ace = 12,
    NumCardTypes = 13,
}

fn get_card_type(ch: char) -> CardType {
    match ch {
        'A' => CardType::Ace,
        'K' => CardType::King,
        'Q' => CardType::Queen,
        'J' => CardType::Jack,
        'T' => CardType::Ten,
        '9' => CardType::Nine,
        '8' => CardType::Eight,
        '7' => CardType::Seven,
        '6' => CardType::Six,
        '5' => CardType::Five,
        '4' => CardType::Four,
        '3' => CardType::Three,
        '2' => CardType::Two,
        _ => CardType::NumCardTypes
    }
}

fn get_hand_type(hand: &Vec<char>) -> HandType {
    let mut num_each_card: Vec<u64> = vec![0; CardType::NumCardTypes as usize];
    for card in hand.iter() {
        num_each_card[get_card_type(*card) as usize] += 1;
    }
    let mut num_matches: Vec<u64> = vec![0; 6]; // Can have a 5 of kind
    for num_cards in num_each_card.iter() {
        num_matches[*num_cards as usize] += 1;
    }
    if num_matches[5] == 1 {
        return HandType::FiveOfAKind;
    }
    if num_matches[4] == 1 {
        return HandType::FourOfAKind;
    }
    if num_matches[3] == 1 {
        if num_matches[2] == 1 {
            return HandType::FullHouse;
        }
        return HandType::ThreeOfAKind;
    }
    if num_matches[2] == 2 {
        return HandType::TwoPair;
    }
    if num_matches[2] == 1 {
        return HandType::OnePair;
    }
    return HandType::HighCard;
}

fn get_hands_and_bid(input: &str) -> Vec<(Vec<char>, u64)> {
    return input.lines()
        .map(String::from)
        .collect::<Vec<String>>()
        .iter()
        .map(|s| (s.split(' ').filter(|s| !s.is_empty()).map(|s| s.trim().to_string()) .collect::<Vec<String>>()[0] .chars() .collect(), 
                s.split(' ').map(|s| s.to_string()) .collect::<Vec<String>>()[1] .parse::<u64>().unwrap()))
        .collect();
}

fn sort_hands(a: &(Vec<char>, u64), b: &(Vec<char>, u64)) -> Ordering {
    let hand_type_a = get_hand_type(&a.0);
    let hand_type_b = get_hand_type(&b.0);
    if (hand_type_a.clone() as usize) > (hand_type_b.clone() as usize) {
        return Ordering::Greater;
    }
    else if (hand_type_a.clone() as usize) < (hand_type_b.clone() as usize) {
        return Ordering::Less;
    }
    for card_num in 0..5 {
        let card_a = get_card_type(a.0[card_num]);
        let card_b = get_card_type(b.0[card_num]);
        if (card_a.clone() as usize) > (card_b.clone() as usize) {
            return Ordering::Greater;
        }
        else if (card_a.clone() as usize) < (card_b.clone() as usize) {
            return Ordering::Less;
        }
    }
    println!("Equal");
    return Ordering::Equal;
}

fn part1(input: &str) -> u64 {
    let mut hands_and_bids = get_hands_and_bid(input);
    hands_and_bids.sort_by(|a, b| sort_hands(a, b));
    // for v in hands_and_bids.iter() {
    //     println!("{:?}", v.0.iter().collect::<String>()); 
    // }
    return hands_and_bids.iter()
        .enumerate()
        .map(|(pos, p)| ((pos as u64) + 1) * p.1)
        .sum();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::*;
        let input1 = 
"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let result1 = part1(input1);
        assert_eq!(result1, 6440);
    }
}
