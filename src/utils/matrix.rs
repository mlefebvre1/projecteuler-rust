use ndarray::Array2;
use std::fs;

pub fn load_matrix2d_from_file<T>(file_name: &str, terminator: char) -> Array2<T>
where
    T: std::default::Default + num::Integer + std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let file = fs::read_to_string(file_name).expect("Problem opening the file");
    let nb_row = file.lines().count();
    let nb_col = file
        .lines()
        .next()
        .unwrap()
        .split_terminator(terminator)
        .count();

    let mut matrix = Array2::<T>::default((nb_row, nb_col));
    for (row, line) in file.lines().enumerate() {
        for (col, value) in line.split_terminator(terminator).enumerate() {
            matrix[[row, col]] = value.parse::<T>().unwrap();
        }
    }
    matrix
}
