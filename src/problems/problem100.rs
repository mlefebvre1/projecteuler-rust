use crate::utils::timeit;

fn p() -> String {
    /*
    Arranged probability
    Problem 100

    If a box contains twenty-one coloured discs, composed of fifteen blue discs and six red discs, and two discs were
    taken at random, it can be seen that the probability of taking two blue discs, P(BB) = (15/21)×(14/20) = 1/2.
    The next such arrangement, for which there is exactly 50% chance of taking two blue discs at random, is a box
    containing eighty-five blue discs and thirty-five red discs.
    By finding the first arrangement to contain over 10**12 = 1,000,000,000,000 discs in total, determine the number
    of blue discs that the box would contain.

    The ratio blue disc on total disc ( x/y ) seems to converge to 1/sqrt(2) as y gets bigger. So x will always be
    y/sqrt(2) + 1 for working y value. The other observation is that the next total discs seems to converge to a ratio
    y(n)/y(n-1) ~5.82.. Every new solution found, simply multiply the total by y(n)/y(n-1) to converge faster.
    */
    const TARGET: u128 = 10_u128.pow(12);
    let mut nb_disk_total = 120_u128; // 85 + 35

    let mut ratio: f64 = 120.0 / 21.0; // initial ratio
    let mut nb_disk_total_previous = nb_disk_total;
    nb_disk_total = (nb_disk_total as f64 * ratio) as u128;
    let mut nb_blue_disk;
    loop {
        nb_blue_disk = (nb_disk_total as f64 / f64::sqrt(2.0) + 1.0).floor() as u128;
        if is_chance_is_50_50(nb_blue_disk, nb_disk_total) {
            if nb_disk_total > TARGET {
                break;
            }
            ratio = nb_disk_total as f64 / nb_disk_total_previous as f64; // re-adjust the multiplier
            nb_disk_total_previous = nb_disk_total;
            nb_disk_total = (nb_disk_total as f64 * ratio) as u128;
        }
        nb_disk_total += 1;
    }
    nb_blue_disk.to_string()
}

fn is_chance_is_50_50(x: u128, y: u128) -> bool {
    /*
    (nb_blue/nb_total) * (nb_blue-1)/(nb_total-1) = 0.5
    2 * nb_blue * (nb_blue -1) = nb_total * (nb_total-1)
    let x be nb_blue and y be nb_total
     */
    2 * x * (x - 1) == y * (y - 1)
}

timeit::timeit!(Problem100, solve, p);
