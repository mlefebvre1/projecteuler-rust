use anyhow::Result;

fn p() -> Result<String> {
    /*
    Disc game prize fund
    Problem 121

    A bag contains one red disc and one blue disc. In a game of chance a player takes a disc at random and its colour is
    noted. After each turn the disc is returned to the bag, an extra red disc is added, and another disc is taken at
    random.

    The player pays £1 to play and wins if they have taken more blue discs than red discs at the end of the game.

    If the game is played for four turns, the probability of a player winning is exactly 11/120, and so the maximum
    prize fund the banker should allocate for winning in this game would be £10 before they would expect to incur a
    loss. Note that any payout will be a whole number of pounds and also includes the original £1 paid to play the game,
    so in the example given the player actually wins £9.

    Find the maximum prize fund that should be allocated to a single game in which fifteen turns are played.

    Solution : Use the binary representation to determine the probability of occuring if we've got a 0 (blue),
               the fraction is 1/(turn+1) if its a 1 the fraction is (turn)/(turn+1). See the table below.

        0 0 0 0   -->   p = 1/2 * 2/3 * 3/4 * 4/5
        0 0 0 1   -->   p = 1/2 * 2/3 * 3/4 * 1/5
        0 0 1 0   -->   p = 1/2 * 2/3 * 1/4 * 4/5
        0 0 1 1   -->   p = 1/2 * 2/3 * 1/4 * 1/5
        0 1 0 0   -->   p = 1/2 * 1/3 * 3/4 * 4/5
        0 1 0 1   -->   p = 1/2 * 1/3 * 3/4 * 1/5
        0 1 1 0   -->   p = 1/2 * 1/3 * 1/4 * 4/5
        0 1 1 1   -->   p = 1/2 * 1/3 * 1/4 * 1/5 <-- blue win
        1 0 0 0   -->   p = 1/2 * 2/3 * 3/4 * 4/5
        1 0 0 1   -->   p = 1/2 * 2/3 * 3/4 * 1/5
        1 0 1 0   -->   p = 1/2 * 2/3 * 1/4 * 4/5
        1 0 1 1   -->   p = 1/2 * 2/3 * 1/4 * 1/5 <-- blue win
        1 1 0 0   -->   p = 1/2 * 1/3 * 3/4 * 4/5
        1 1 0 1   -->   p = 1/2 * 1/3 * 3/4 * 1/5 <-- blue win
        1 1 1 0   -->   p = 1/2 * 1/3 * 1/4 * 4/5 <-- blue win
        1 1 1 1   -->   p = 1/2 * 1/3 * 1/4 * 1/5 <-- blue win
    */
    let mut blue_win_probability: f64 = 0.0;
    for event in 0..2_u64.pow(15) {
        let event_as_bin = format!("{event:015b}");
        let mut outcome_probability: f64 = 1.0;
        let (mut nb_red, mut nb_blue) = (0, 0);
        for (turn, pick) in event_as_bin.chars().enumerate() {
            if pick == '0' {
                nb_blue += 1;
                outcome_probability *= 1.0 / (turn + 2) as f64;
            } else {
                nb_red += 1;
                outcome_probability *= (turn + 1) as f64 / (turn + 2) as f64;
            }
        }
        if nb_blue > nb_red {
            blue_win_probability += outcome_probability;
        }
    }
    let ans = (1.0 / blue_win_probability).floor();

    Ok(ans.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(p().unwrap(), "2269");
    }
}
