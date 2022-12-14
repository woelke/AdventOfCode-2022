use itertools::iproduct;
use itertools::Itertools;
use itertools::MinMaxResult::MinMax;
use simple_matrix::Matrix;

use crate::data_loader::DataLoader;

pub trait MatrixLoader {
    fn to_matrix<T: std::default::Default + std::str::FromStr>(&self) -> Result<Matrix<T>, &str>;
}

impl MatrixLoader for DataLoader {
    fn to_matrix<T: std::default::Default + std::str::FromStr>(&self) -> Result<Matrix<T>, &str> {
        let col_len: usize;
        match self.lines.iter().map(|line| line.len()).minmax() {
            MinMax(min, max) if min == max => col_len = max,
            _ => return Err("rows not of equal length"),
        }

        let mut mat: Matrix<T> = Matrix::<T>::new(self.lines.len(), col_len); // (Reihe(row), Spalte

        for (row, line) in self.lines.iter().enumerate() {
            for (col, c) in line.chars().enumerate() {
                let val = c
                    .to_string()
                    .parse::<T>()
                    .map_err(|_| "failed to parse value")?;
                mat.set(row, col, val);
            }
        }

        Ok(mat)
    }
}

pub fn get_flatten_matrix<'a, T>(mat: &'a Matrix<T>) -> Vec<(usize, usize, &'a T)> {
    let mut res = vec![];

    for (row, col) in iproduct!(0..mat.rows(), 0..mat.cols()) {
        res.push((row, col, mat.get(row, col).unwrap()));
    }

    res
}

pub fn print_matrix<T: std::fmt::Display>(mat: &Matrix<T>) {
    for row in 0..mat.rows() {
        for col in 0..mat.cols() {
            print!("{} ", mat.get(row, col).unwrap());
        }
        println!();
    }
}
