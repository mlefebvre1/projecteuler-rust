use arrayvec::ArrayVec;

#[derive(PartialEq, PartialOrd)]
enum PokerHandRank {
    HighestCard = 0,
    Pair = 1,
    TwoPair = 2,
    ThreeOfAKind = 3,
    Straight = 4,
    Flush = 5,
    FullHouse = 6,
    FourOfAKind = 7,
    StraightFlush = 8,
    RoyalFlush = 9,
}

struct PokerHandResult {
    rank: PokerHandRank,
    highest_card: u8,
    value1: u8,
    value2: u8,
}

#[derive(Debug, PartialEq)]
enum CardColor {
    Clubs,
    Diamonds,
    Heart,
    Spades,
}

#[derive(Debug, PartialEq)]
pub struct Card {
    pub value: u8,
    color: CardColor,
}

impl Card {
    pub fn from_string(card: &str) -> Card {
        // Card should be writing as : <value><color>
        let value = match &card[0..1] {
            "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => {
                Some(card[0..1].parse::<u8>().unwrap())
            }
            "T" => Some(10u8),
            "J" => Some(11u8),
            "Q" => Some(12u8),
            "K" => Some(13u8),
            "A" => Some(14u8),
            _ => None,
        }
        .unwrap();
        let color = match &card[1..2] {
            "C" => Some(CardColor::Clubs),
            "D" => Some(CardColor::Diamonds),
            "H" => Some(CardColor::Heart),
            "S" => Some(CardColor::Spades),
            _ => None,
        }
        .unwrap();
        Card { value, color }
    }
}

type PokerHand = [Card; 5];

fn get_hand_result(hand: &PokerHand) -> PokerHandResult {
    let highest = hand.iter().max_by_key(|&card| card.value).unwrap().value;

    if is_royal_flush(hand).is_some() {
        return PokerHandResult {
            rank: PokerHandRank::RoyalFlush,
            highest_card: highest,
            value1: 0,
            value2: 0,
        };
    }
    if let Some(value) = is_straight_flush(hand) {
        return PokerHandResult {
            rank: PokerHandRank::StraightFlush,
            highest_card: highest,
            value1: value,
            value2: 0,
        };
    }
    if let Some(value) = is_four_of_a_kind(hand) {
        return PokerHandResult {
            rank: PokerHandRank::FourOfAKind,
            highest_card: highest,
            value1: value,
            value2: 0,
        };
    }
    if let Some(value) = is_full_house(hand) {
        return PokerHandResult {
            rank: PokerHandRank::FullHouse,
            highest_card: highest,
            value1: value[0],
            value2: value[1],
        };
    }
    if is_flush(hand).is_some() {
        return PokerHandResult {
            rank: PokerHandRank::Flush,
            highest_card: highest,
            value1: 0,
            value2: 0,
        };
    }
    if let Some(value) = is_straight(hand) {
        return PokerHandResult {
            rank: PokerHandRank::Straight,
            highest_card: highest,
            value1: value,
            value2: 0,
        };
    }
    if let Some(value) = is_three_of_a_kind(hand) {
        return PokerHandResult {
            rank: PokerHandRank::ThreeOfAKind,
            highest_card: highest,
            value1: value,
            value2: 0,
        };
    }
    if let Some(value) = is_a_two_pair(hand) {
        return PokerHandResult {
            rank: PokerHandRank::TwoPair,
            highest_card: highest,
            value1: value[0],
            value2: value[1],
        };
    }
    if let Some(value) = is_a_pair(hand) {
        return PokerHandResult {
            rank: PokerHandRank::Pair,
            highest_card: highest,
            value1: value,
            value2: 0,
        };
    }
    PokerHandResult {
        rank: PokerHandRank::HighestCard,
        highest_card: highest,
        value1: 0,
        value2: 0,
    }
}

pub fn is_player1_winner(hand_player1: &PokerHand, hand_player2: &PokerHand) -> bool {
    let hand1_result = get_hand_result(hand_player1);
    let hand2_result = get_hand_result(hand_player2);
    if hand1_result.rank > hand2_result.rank {
        return true;
    } else if hand1_result.rank == hand2_result.rank {
        if hand1_result.value1 == hand2_result.value1 && hand1_result.value2 == hand2_result.value2
        {
            if hand1_result.highest_card > hand2_result.highest_card {
                return true;
            }
            return false;
        } else if hand1_result.value1 > hand2_result.value1
            || hand1_result.value2 > hand2_result.value2
        {
            return true;
        }
        return false;
    }
    false
}

pub fn poker_hand_from_string(hand: &[&str]) -> PokerHand {
    let poker_hand: ArrayVec<_, 5> = hand.iter().map(|card| Card::from_string(card)).collect();

    poker_hand.into_inner().unwrap_or_else(|_| panic!()) as PokerHand
}

fn poker_hand_value_occurences(hand: &PokerHand) -> [usize; 15] {
    let mut occurrences = [0usize; 15];
    for card in hand {
        occurrences[card.value as usize] += 1;
    }
    occurrences
}

fn is_straight(hand: &PokerHand) -> Option<u8> {
    // Assumes cards are sorted by value
    let first_card = &hand[0];
    let mut next_card_value = first_card.value + 1;
    for card in &hand[1..] {
        if card.value != next_card_value {
            return None;
        }
        next_card_value += 1;
    }
    Some(first_card.value)
}

fn is_flush(hand: &PokerHand) -> Option<u8> {
    let first_color = &hand[0].color;
    for card in &hand[1..] {
        if card.color != *first_color {
            return None;
        }
    }
    Some(hand.iter().max_by_key(|card| card.value).unwrap().value as u8)
}

fn is_royal_flush(hand: &PokerHand) -> Option<u8> {
    if let Some(first_card) = is_straight(hand) {
        if is_flush(hand).is_some() && first_card == 10 {
            return Some(10);
        }
    }
    None
}

fn is_straight_flush(hand: &PokerHand) -> Option<u8> {
    if let Some(first_card) = is_straight(hand) {
        if is_flush(hand).is_some() {
            return Some(first_card);
        }
    }
    None
}

fn is_four_of_a_kind(hand: &PokerHand) -> Option<u8> {
    let occurrences = poker_hand_value_occurences(hand);
    for (value, number) in occurrences.iter().enumerate() {
        if *number == 4 {
            return Some(value as u8);
        }
    }
    None
}

fn is_three_of_a_kind(hand: &PokerHand) -> Option<u8> {
    let occurrences = poker_hand_value_occurences(hand);
    for (value, number) in occurrences.iter().enumerate() {
        if *number == 3 {
            return Some(value as u8);
        }
    }
    None
}

fn is_a_pair(hand: &PokerHand) -> Option<u8> {
    let occurrences = poker_hand_value_occurences(hand);
    for (value, number) in occurrences.iter().enumerate() {
        if *number == 2 {
            return Some(value as u8);
        }
    }
    None
}

fn is_a_two_pair(hand: &PokerHand) -> Option<Vec<u8>> {
    let occurrences = poker_hand_value_occurences(hand);
    let mut pairs = Vec::new();
    for (value, number) in occurrences.iter().enumerate() {
        if *number == 2 {
            pairs.push(value as u8);
        }
    }
    if pairs.len() == 2 {
        return Some(pairs);
    }
    None
}

fn is_full_house(hand: &PokerHand) -> Option<[u8; 2]> {
    if let (Some(three_value), Some(pair_value)) = (is_three_of_a_kind(hand), is_a_pair(hand)) {
        return Some([three_value, pair_value]);
    }
    None
}

// Unittest tests below
#[test]
fn test_poker_hand_from_string() {
    let hand = ["5H", "5C", "6S", "7S", "KD"];
    let expected = [
        Card {
            value: 5,
            color: CardColor::Heart,
        },
        Card {
            value: 5,
            color: CardColor::Clubs,
        },
        Card {
            value: 6,
            color: CardColor::Spades,
        },
        Card {
            value: 7,
            color: CardColor::Spades,
        },
        Card {
            value: 13,
            color: CardColor::Diamonds,
        },
    ];
    assert_eq!(poker_hand_from_string(&hand), expected as PokerHand);
}

#[test]
fn test_poker_hand_value_occurences() {
    let hand = [
        Card {
            value: 5,
            color: CardColor::Heart,
        },
        Card {
            value: 5,
            color: CardColor::Clubs,
        },
        Card {
            value: 6,
            color: CardColor::Spades,
        },
        Card {
            value: 7,
            color: CardColor::Spades,
        },
        Card {
            value: 13,
            color: CardColor::Diamonds,
        },
    ];
    let expected = [0, 0, 0, 0, 0, 2, 1, 1, 0, 0, 0, 0, 0, 1, 0];
    assert_eq!(poker_hand_value_occurences(&hand), expected);
}

#[test]
fn test_is_straight() {
    let hand = [
        Card {
            value: 5,
            color: CardColor::Heart,
        },
        Card {
            value: 6,
            color: CardColor::Clubs,
        },
        Card {
            value: 7,
            color: CardColor::Spades,
        },
        Card {
            value: 8,
            color: CardColor::Spades,
        },
        Card {
            value: 9,
            color: CardColor::Diamonds,
        },
    ];
    assert_eq!(is_straight(&hand), Some(5));

    let hand = [
        Card {
            value: 9,
            color: CardColor::Heart,
        },
        Card {
            value: 11,
            color: CardColor::Clubs,
        },
        Card {
            value: 12,
            color: CardColor::Spades,
        },
        Card {
            value: 13,
            color: CardColor::Spades,
        },
        Card {
            value: 14,
            color: CardColor::Diamonds,
        },
    ];
    assert_eq!(is_straight(&hand), None);
}

#[test]
fn test_is_flush() {
    let hand = [
        Card {
            value: 2,
            color: CardColor::Heart,
        },
        Card {
            value: 4,
            color: CardColor::Heart,
        },
        Card {
            value: 5,
            color: CardColor::Heart,
        },
        Card {
            value: 6,
            color: CardColor::Heart,
        },
        Card {
            value: 7,
            color: CardColor::Heart,
        },
    ];
    assert_eq!(is_flush(&hand), Some(7));

    let hand = [
        Card {
            value: 2,
            color: CardColor::Heart,
        },
        Card {
            value: 4,
            color: CardColor::Heart,
        },
        Card {
            value: 5,
            color: CardColor::Heart,
        },
        Card {
            value: 6,
            color: CardColor::Heart,
        },
        Card {
            value: 7,
            color: CardColor::Spades,
        },
    ];
    assert_eq!(is_flush(&hand), None);
}

#[test]
fn test_is_royal_flush() {
    let hand = [
        Card {
            value: 10,
            color: CardColor::Heart,
        },
        Card {
            value: 11,
            color: CardColor::Heart,
        },
        Card {
            value: 12,
            color: CardColor::Heart,
        },
        Card {
            value: 13,
            color: CardColor::Heart,
        },
        Card {
            value: 14,
            color: CardColor::Heart,
        },
    ];
    assert_eq!(is_royal_flush(&hand), Some(10));

    let hand = [
        Card {
            value: 9,
            color: CardColor::Heart,
        },
        Card {
            value: 10,
            color: CardColor::Heart,
        },
        Card {
            value: 11,
            color: CardColor::Heart,
        },
        Card {
            value: 12,
            color: CardColor::Heart,
        },
        Card {
            value: 13,
            color: CardColor::Heart,
        },
    ];
    assert_eq!(is_royal_flush(&hand), None);
}

#[test]
fn test_is_straight_flush() {
    let hand = [
        Card {
            value: 5,
            color: CardColor::Clubs,
        },
        Card {
            value: 6,
            color: CardColor::Clubs,
        },
        Card {
            value: 7,
            color: CardColor::Clubs,
        },
        Card {
            value: 8,
            color: CardColor::Clubs,
        },
        Card {
            value: 9,
            color: CardColor::Clubs,
        },
    ];
    assert_eq!(is_straight_flush(&hand), Some(5));

    let hand = [
        Card {
            value: 5,
            color: CardColor::Clubs,
        },
        Card {
            value: 6,
            color: CardColor::Clubs,
        },
        Card {
            value: 7,
            color: CardColor::Clubs,
        },
        Card {
            value: 8,
            color: CardColor::Clubs,
        },
        Card {
            value: 9,
            color: CardColor::Spades,
        },
    ];
    assert_eq!(is_straight_flush(&hand), None);
}

#[test]
fn test_is_four_a_kind() {
    let hand = [
        Card {
            value: 5,
            color: CardColor::Clubs,
        },
        Card {
            value: 5,
            color: CardColor::Spades,
        },
        Card {
            value: 5,
            color: CardColor::Diamonds,
        },
        Card {
            value: 5,
            color: CardColor::Heart,
        },
        Card {
            value: 9,
            color: CardColor::Clubs,
        },
    ];
    assert_eq!(is_four_of_a_kind(&hand), Some(5));

    let hand = [
        Card {
            value: 5,
            color: CardColor::Clubs,
        },
        Card {
            value: 5,
            color: CardColor::Spades,
        },
        Card {
            value: 5,
            color: CardColor::Diamonds,
        },
        Card {
            value: 6,
            color: CardColor::Heart,
        },
        Card {
            value: 9,
            color: CardColor::Clubs,
        },
    ];
    assert_eq!(is_four_of_a_kind(&hand), None);
}

#[test]
fn test_is_three_of_a_kind() {
    let hand = [
        Card {
            value: 5,
            color: CardColor::Clubs,
        },
        Card {
            value: 5,
            color: CardColor::Spades,
        },
        Card {
            value: 5,
            color: CardColor::Diamonds,
        },
        Card {
            value: 5,
            color: CardColor::Heart,
        },
        Card {
            value: 9,
            color: CardColor::Clubs,
        },
    ];
    assert_eq!(is_three_of_a_kind(&hand), None);

    let hand = [
        Card {
            value: 6,
            color: CardColor::Clubs,
        },
        Card {
            value: 6,
            color: CardColor::Spades,
        },
        Card {
            value: 6,
            color: CardColor::Diamonds,
        },
        Card {
            value: 3,
            color: CardColor::Heart,
        },
        Card {
            value: 9,
            color: CardColor::Clubs,
        },
    ];
    assert_eq!(is_three_of_a_kind(&hand), Some(6));

    let hand = [
        Card {
            value: 6,
            color: CardColor::Clubs,
        },
        Card {
            value: 6,
            color: CardColor::Spades,
        },
        Card {
            value: 2,
            color: CardColor::Diamonds,
        },
        Card {
            value: 3,
            color: CardColor::Heart,
        },
        Card {
            value: 9,
            color: CardColor::Clubs,
        },
    ];
    assert_eq!(is_three_of_a_kind(&hand), None);
}

#[test]
fn test_is_a_pair() {
    let hand = [
        Card {
            value: 7,
            color: CardColor::Clubs,
        },
        Card {
            value: 7,
            color: CardColor::Spades,
        },
        Card {
            value: 2,
            color: CardColor::Diamonds,
        },
        Card {
            value: 3,
            color: CardColor::Heart,
        },
        Card {
            value: 9,
            color: CardColor::Clubs,
        },
    ];
    assert_eq!(is_a_pair(&hand), Some(7));

    let hand = [
        Card {
            value: 7,
            color: CardColor::Clubs,
        },
        Card {
            value: 7,
            color: CardColor::Spades,
        },
        Card {
            value: 7,
            color: CardColor::Diamonds,
        },
        Card {
            value: 3,
            color: CardColor::Heart,
        },
        Card {
            value: 9,
            color: CardColor::Clubs,
        },
    ];
    assert_eq!(is_a_pair(&hand), None);
}

#[test]
fn test_is_a_two_pair() {
    let hand = [
        Card {
            value: 7,
            color: CardColor::Clubs,
        },
        Card {
            value: 7,
            color: CardColor::Spades,
        },
        Card {
            value: 3,
            color: CardColor::Diamonds,
        },
        Card {
            value: 3,
            color: CardColor::Heart,
        },
        Card {
            value: 9,
            color: CardColor::Clubs,
        },
    ];
    assert_eq!(is_a_two_pair(&hand), Some(vec![3, 7]));

    let hand = [
        Card {
            value: 7,
            color: CardColor::Clubs,
        },
        Card {
            value: 7,
            color: CardColor::Spades,
        },
        Card {
            value: 4,
            color: CardColor::Diamonds,
        },
        Card {
            value: 3,
            color: CardColor::Heart,
        },
        Card {
            value: 9,
            color: CardColor::Clubs,
        },
    ];
    assert_eq!(is_a_two_pair(&hand), None);
}

#[test]
fn test_is_full_house() {
    let hand = [
        Card {
            value: 4,
            color: CardColor::Clubs,
        },
        Card {
            value: 4,
            color: CardColor::Spades,
        },
        Card {
            value: 4,
            color: CardColor::Diamonds,
        },
        Card {
            value: 3,
            color: CardColor::Heart,
        },
        Card {
            value: 3,
            color: CardColor::Clubs,
        },
    ];
    assert_eq!(is_full_house(&hand), Some([4, 3]));

    let hand = [
        Card {
            value: 7,
            color: CardColor::Clubs,
        },
        Card {
            value: 7,
            color: CardColor::Spades,
        },
        Card {
            value: 7,
            color: CardColor::Diamonds,
        },
        Card {
            value: 3,
            color: CardColor::Heart,
        },
        Card {
            value: 9,
            color: CardColor::Clubs,
        },
    ];
    assert_eq!(is_full_house(&hand), None);
}
