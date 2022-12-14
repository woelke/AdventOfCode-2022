#![allow(unused_variables, dead_code)]

use aoc_helpers::data_loader::DataLoader;

mod solver;
use crate::solver::{solve_a, solve_b};

fn main() {
    let a_input = DataLoader::from_file("data/puzzle_input.txt");
    println!("a: {}", solve_a(&a_input));
    println!("b: {}", solve_b(&a_input));
}

#[cfg(test)]
mod test_main {
    use super::*;

    #[test]
    fn a_test() {
        assert_eq!(
            solve_a(&DataLoader::from_file("data/test_input.txt")),
            DataLoader::from_file("data/a_test_result.txt").test_result()
        );
    }

    #[test]
    fn b_test() {
        assert_eq!(
            solve_b(&DataLoader::from_file("data/test_input.txt")),
            DataLoader::from_file("data/b_test_result.txt").test_result()
        );
    }
}
