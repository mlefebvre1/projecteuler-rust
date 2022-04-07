use rand::{thread_rng, Rng};

pub const NB_SQUARES: usize = 40;
const NB_FACES: usize = 4;
const CC: [usize; 3] = [2, 17, 33]; // Community chest
const CH: [usize; 3] = [7, 22, 36]; // Chance
const G2J: usize = 30;
const JAIL: usize = 10;
const GO: usize = 0;
const C1: usize = 11;
const E3: usize = 24;
const H2: usize = 39;
const R1: usize = 5;
const NB_CH_OR_CC_CARDS: usize = 16;

pub struct Game {
    current_square: usize,
    current_nb_doubles: usize,
    nb_rolls: usize,
    pub nb_visit_per_square: [usize; NB_SQUARES],
}

impl Game {
    pub fn new(nb_rolls: usize) -> Self {
        Self {
            current_square: 0,
            current_nb_doubles: 0,
            nb_rolls,
            nb_visit_per_square: [0; NB_SQUARES],
        }
    }

    pub fn play(&mut self) {
        for _roll in 0..self.nb_rolls {
            let dice_sum = self.roll_dices();
            self.current_square = self.get_next_square(dice_sum);
            self.nb_visit_per_square[self.current_square] += 1;
        }
    }

    fn next_r(&self, current_square: usize) -> usize {
        match current_square {
            x if x < 5 => 5,
            x if x < 15 => 15,
            x if x < 25 => 25,
            x if x < 35 => 35,
            _ => 5,
        }
    }

    fn next_u(&self, current_square: usize) -> usize {
        match current_square {
            x if x < 12 => 12,
            x if x < 28 => 28,
            _ => 12,
        }
    }

    fn roll_dices(&mut self) -> usize {
        let mut rng = thread_rng();
        let d1: usize = rng.gen_range(1..=NB_FACES);
        let d2: usize = rng.gen_range(1..=NB_FACES);
        if d1 == d2 {
            self.current_nb_doubles += 1;
        } else {
            self.current_nb_doubles = 0;
        }
        d1 + d2
    }

    fn get_next_square(&self, dice_sum: usize) -> usize {
        if self.current_nb_doubles == 3 {
            // GO to JAIL if you roll 3 times a double
            return JAIL;
        }
        let mut next_square = (self.current_square + dice_sum) % NB_SQUARES;
        if CH.contains(&next_square) {
            let mut rng = thread_rng();
            let draw = rng.gen_range(1..=NB_CH_OR_CC_CARDS);
            next_square = match draw {
                1 => GO,
                2 => JAIL,
                3 => C1,
                4 => E3,
                5 => H2,
                6 => R1,
                7 => self.next_r(next_square),
                8 => self.next_r(next_square),
                9 => self.next_u(next_square),
                10 => (next_square - 3) % NB_SQUARES,
                _ => next_square,
            };
        } else if CC.contains(&next_square) {
            let mut rng = thread_rng();
            let draw = rng.gen_range(1..NB_CH_OR_CC_CARDS);
            next_square = match draw {
                1 => GO,
                2 => JAIL,
                _ => next_square,
            };
        } else if next_square == G2J {
            next_square = JAIL;
        }
        next_square
    }
}
