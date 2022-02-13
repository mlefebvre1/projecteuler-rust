use crate::ntheory::primes::is_prime;
use crate::utils::timeit;

fn p() -> String {
    /*
    Spiral primes
    Problem 58

    Starting with 1 and spiralling anticlockwise in the following way, a square spiral with side length 7 is formed.

    37 36 35 34 33 32 31
    38 17 16 15 14 13 30
    39 18  5  4  3 12 29
    40 19  6  1  2 11 28
    41 20  7  8  9 10 27
    42 21 22 23 24 25 26
    43 44 45 46 47 48 49

    It is interesting to note that the odd squares lie along the bottom right diagonal, but what is more interesting is
    that 8 out of the 13 numbers lying along both diagonals are prime; that is, a ratio of 8/13 ≈ 62%.

    If one complete new layer is wrapped around the spiral above, a square spiral with side length 9 will be formed.
    If this process is continued, what is the side length of the square spiral for which the ratio of primes along both
    diagonals first falls below 10%?

    101 100 99  98  97  96  95  94  93  92  91
    102 65  64  63  62  61  60  59  58  57  90
    103 66  37  36  35  34  33  32  31  56  89
    104 67  38  17  16  15  14  13  30  55  88
    105 68  39  18  5   4   3   12  29  54  87
    106 69  40  19  6   1   2   11  28  53  86
    107 70  41  20  7   8   9   10  27  52  85
    108 71  42  21  22  23  24  25  26  51  84
    109 72  43  44  45  46  47  48  49  50  83
    110 73  74  75  76  77  78  79  80  81  82
    111 112 113 114 115 116 117 118 119 120 121

    Diag down-right : 1,9,25,49,81,121 -> +8,+16,+24,+32,+40  or 3*3 5*5 7*7 9*9 11*11
    Diag up-right : 1,3,13,31,57,91    -> +2,+10,+18,+26,+34
    Diag up-left : 1,5,17,37,65,101    -> +4,+12,+20,+28,+36
    Diag down-left : 1,7,21,43,73,111  -> +6,+14,+22,+30,+38
    */
    let mut current_values = Spiral {
        down_right: 1,
        up_right: 1,
        up_left: 1,
        down_left: 1,
    };
    let mut increments = Spiral {
        down_right: 8,
        up_right: 2,
        up_left: 4,
        down_left: 6,
    };
    let mut nb_primes = 0;
    let mut nb_candidates = 1;
    let mut side_len = 0;
    loop {
        current_values.incr_by(&increments);
        increments.incr_by(&Spiral {
            down_right: 8,
            up_right: 8,
            up_left: 8,
            down_left: 8,
        });
        nb_primes += current_values.get_nb_primes();
        nb_candidates += 4;
        side_len += 1;
        if (100 * nb_primes / nb_candidates) < 10 {
            // nb_primes / nb_candidates is less than 10%
            break;
        }
    }
    (2 * side_len + 1).to_string()
}

struct Spiral {
    down_right: usize,
    up_right: usize,
    up_left: usize,
    down_left: usize,
}

impl Spiral {
    fn incr_by(&mut self, increments: &Self) {
        self.down_right += increments.down_right;
        self.up_right += increments.up_right;
        self.up_left += increments.up_left;
        self.down_left += increments.down_left;
    }
    fn get_nb_primes(&self) -> usize {
        let mut nb_primes = 0;
        if is_prime(self.down_right) {
            nb_primes += 1;
        }
        if is_prime(self.up_right) {
            nb_primes += 1;
        }
        if is_prime(self.up_left) {
            nb_primes += 1;
        }
        if is_prime(self.down_left) {
            nb_primes += 1;
        }
        nb_primes
    }
}

timeit::timeit!(Problem58, solve, p);
