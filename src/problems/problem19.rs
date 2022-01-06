use crate::utils::timeit;
use phf::phf_map;

static NB_DAYS_PER_MONTH: phf::Map<&'static str, usize> = phf_map! {
    "january"=> 31,
    "february"=> 28,
    "march"=> 31,
    "april"=> 30,
    "may"=> 31,
    "june"=> 30,
    "july"=> 31,
    "august"=> 31,
    "september"=> 30,
    "october"=> 31,
    "november"=> 30,
    "december"=> 31,
};

fn nb_days_in_month(month: &str, year: usize) -> usize {
    if month == "february" {
        if year % 4 == 0 && year % 400 != 0 {
            return *NB_DAYS_PER_MONTH.get(month).unwrap() + 1;
        }
    }
    return *NB_DAYS_PER_MONTH.get(month).unwrap();
}

fn p() -> usize {
    /*
    Counting Sundays
    Problem 19
    You are given the following information, but you may prefer to do some research for yourself.
        1 Jan 1900 was a Monday.
        Thirty days has September,
        April, June and November.
        All the rest have thirty-one,
        Saving February alone,
        Which has twenty-eight, rain or shine.
        And on leap years, twenty-nine.
        A leap year occurs on any year evenly divisible by 4, but not on a century unless it is divisible by 400.
    How many Sundays fell on the first of the month during the twentieth century (1 Jan 1901 to 31 Dec 2000)?
    */
    const MONTHS: [&str; 12] = [
        "january",
        "february",
        "march",
        "april",
        "may",
        "june",
        "july",
        "august",
        "september",
        "october",
        "november",
        "december",
    ];

    let mut day = 0;
    let mut nb_sundays = 0;

    for year in 1901..2000 {
        // for each year of the twentieth century
        for month in MONTHS {
            let nb_days = nb_days_in_month(month, year);
            while day < nb_days {
                day += 7;
            }
            day -= nb_days; // this gives the first day of the next month
            if day == 6 {
                nb_sundays += 1;
            }
        }
    }
    return nb_sundays;
}

timeit::timeit!(Problem19, solve, p);