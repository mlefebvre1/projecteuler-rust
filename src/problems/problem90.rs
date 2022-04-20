use crate::utils::timeit;

fn p() -> String {
    /*
    Cube digit pairs
    Problem 90

    Each of the six faces on a cube has a different digit (0 to 9) written on it; the same is done to a second cube. By
    placing the two cubes side-by-side in different positions we can form a variety of 2-digit numbers.
    For example, the square number 64 could be formed:
    In fact, by carefully choosing the digits on both cubes it is possible to display all of the square numbers below
    one-hundred: 01, 04, 09, 16, 25, 36, 49, 64, and 81.
    For example, one way this can be achieved is by placing {0, 5, 6, 7, 8, 9} on one cube and {1, 2, 3, 4, 8, 9} on the
    other cube.
    However, for this problem we shall allow the 6 or 9 to be turned upside-down so that an arrangement like
    {0, 5, 6, 7, 8, 9} and {1, 2, 3, 4, 6, 7} allows for all nine square numbers to be displayed; otherwise it would be
    impossible to obtain 09.
    In determining a distinct arrangement we are interested in the digits on each cube, not the order.
    {1, 2, 3, 4, 5, 6} is equivalent to {3, 6, 4, 1, 2, 5}
    {1, 2, 3, 4, 5, 6} is distinct from {1, 2, 3, 4, 5, 9}
    But because we are allowing 6 and 9 to be reversed, the two distinct sets in the last example both represent the
    extended set {1, 2, 3, 4, 5, 6, 9} for the purpose of forming 2-digit numbers.
    How many distinct arrangements of the two cubes allow for all of the square numbers to be displayed?
    */
    let mut arrangements = Vec::new();
    let dice_candidates = generate_dices();
    for dice1 in dice_candidates.iter() {
        for dice2 in dice_candidates.iter() {
            if squares_validation(dice1, dice2)
                && !arrangements.contains(&(dice1, dice2))
                && !arrangements.contains(&(dice2, dice1))
            {
                arrangements.push((dice1, dice2));
            }
        }
    }
    arrangements.len().to_string()
}

type Dice = [usize; 6]; // 6 faces
type Square = (usize, usize); // 2 faces

const POSSIBLE_SQUARES: [Square; 14] = [
    (0, 1),
    (0, 4),
    (0, 6),
    (0, 9),
    (1, 6),
    (1, 9),
    (2, 5),
    (3, 6),
    (3, 9),
    (4, 6),
    (4, 9),
    (6, 4),
    (8, 1),
    (9, 4),
];

fn generate_dices() -> Vec<Dice> {
    let mut dices = Vec::new();
    for f1 in 0..10 {
        for f2 in (f1 + 1)..10 {
            for f3 in (f2 + 1)..10 {
                for f4 in (f3 + 1)..10 {
                    for f5 in (f4 + 1)..10 {
                        for f6 in (f5 + 1)..10 {
                            let dice = [f1, f2, f3, f4, f5, f6];
                            dices.push(dice);
                        }
                    }
                }
            }
        }
    }
    dices
}

fn make_two_dices_pairs(dice1: &Dice, dice2: &Dice) -> Vec<Square> {
    let mut two_dices_pairs = Vec::new();
    for face1 in dice1.iter() {
        for face2 in dice2.iter() {
            two_dices_pairs.push((*face1, *face2));
            two_dices_pairs.push((*face2, *face1));
        }
    }
    two_dices_pairs
}

fn presence_index_from_square(square: &Square) -> usize {
    match square {
        (0, 1) => 0,
        (0, 4) => 1,
        (0, 6) | (0, 9) => 2,
        (1, 6) | (1, 9) => 3,
        (2, 5) => 4,
        (3, 6) | (3, 9) => 5,
        (4, 9) | (4, 6) => 6,
        (6, 4) | (9, 4) => 7,
        (8, 1) => 8,
        _ => usize::MAX,
    }
}

fn squares_validation(dice1: &Dice, dice2: &Dice) -> bool {
    let mut pair_presence = [false; 9];

    let pairs = make_two_dices_pairs(dice1, dice2);
    for square in POSSIBLE_SQUARES.iter() {
        if pairs.contains(square) {
            pair_presence[presence_index_from_square(square)] = true;
        }
    }
    pair_presence.iter().find(|&presence| !presence) == None
}

timeit::timeit!(Problem90, solve, p);
