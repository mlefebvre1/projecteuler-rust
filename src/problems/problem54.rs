use crate::utils::poker;
use anyhow::Result;
use std::fs;

fn p() -> Result<String> {
    /*
    Poker hands
    Problem 54
    In the card game poker, a hand consists of five cards and are ranked, from lowest to highest, in the following way:

    High Card: Highest value card.
    One Pair: Two cards of the same value.
    Two Pairs: Two different pairs.
    Three of a Kind: Three cards of the same value.
    Straight: All cards are consecutive values.
    Flush: All cards of the same suit.
    Full House: Three of a kind and a pair.
    Four of a Kind: Four cards of the same value.
    Straight Flush: All cards are consecutive values of same suit.
    Royal Flush: Ten, Jack, Queen, King, Ace, in same suit.
    The cards are valued in the order:
    2, 3, 4, 5, 6, 7, 8, 9, 10, Jack, Queen, King, Ace.

    If two players have the same ranked hands then the rank made up of the highest value wins; for example, a pair of eights
    beats a pair of fives (see example 1 below). But if two ranks tie, for example, both players have a pair of queens,
    then highest cards in each hand are compared (see example 4 below); if the highest cards tie then the next highest cards
    are compared, and so on.
    The file, poker.txt, contains one-thousand random hands dealt to two players. Each line of the file contains ten cards (separated by a single space): the first five are Player 1's cards and the last five are Player 2's cards. You can assume that all hands are valid (no invalid characters or repeated cards), each player's hand is in no specific order, and in each hand there is a clear winner.

    How many hands does Player 1 win?
    */
    let data =
        fs::read_to_string("src/problems/data/problem54.txt").expect("Problem opening the file");
    let games = data.lines();

    let nb_win_player1 = games
        .filter(|game| {
            let _game: Vec<&str> = game.split_whitespace().collect();
            let mut hand_player1 = poker::poker_hand_from_string(&_game[0..5]);
            hand_player1.sort_by_key(|k| k.value);
            let mut hand_player2 = poker::poker_hand_from_string(&_game[5..10]);
            hand_player2.sort_by_key(|k| k.value);
            poker::is_player1_winner(&hand_player1, &hand_player2)
        })
        .count();
    Ok(nb_win_player1.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(p().unwrap(), "376");
    }
}
