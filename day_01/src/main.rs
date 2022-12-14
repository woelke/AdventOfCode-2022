#![allow(unused_variables, dead_code)]

use aoc_helpers::data_loader::get_input;

mod solver;
use crate::solver::{solve_a, solve_b};

fn main() {
    let a_input = get_input("data/a_puzzle_input.txt").unwrap();
    println!("a: {}", solve_a(&a_input));
    println!("b: {}", solve_b(&a_input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_helpers::data_loader::get_result;

    #[test]
    fn a_test() {
        let input = get_input("data/a_test_input.txt").unwrap();
        let result = get_result("data/a_test_result.txt").unwrap();

        assert_eq!(solve_a(&input), result);
    }

    #[test]
    fn b_test() {
        let input = get_input("data/a_test_input.txt").unwrap();
        let result = get_result("data/b_test_result.txt").unwrap();

        assert_eq!(solve_b(&input), result);
    }
}
