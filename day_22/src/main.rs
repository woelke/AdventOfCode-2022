#![feature(let_chains)]
#![allow(unused_variables, dead_code)]

use aoc_helpers::data_loader::DataLoader;

mod solver_a;
use crate::solver_a::solve_a;

mod solver_b;
use crate::solver_b::solve_b;

fn main() {
    let a_input = DataLoader::from_file("data/puzzle_input.txt");
    println!("a: {}", solve_a(&a_input).unwrap());
    println!("b: {}", solve_b(&a_input, 50).unwrap());
}

#[cfg(test)]
mod test_main {
    use super::*;

    #[test]
    fn a_test() {
        assert_eq!(
            solve_a(&DataLoader::from_file("data/test_input.txt")).unwrap(),
            DataLoader::from_file("data/a_test_result.txt").test_result()
        );
    }

    #[test]
    fn b_test() {
        assert_eq!(
            solve_b(&DataLoader::from_file("data/test_input.txt"), 4).unwrap(),
            DataLoader::from_file("data/b_test_result.txt").test_result()
        );
    }
}
