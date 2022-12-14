use aoc_helpers::data_loader::DataLoader;
use aoc_helpers::slide_iter::SlideIterator;

use itertools::Itertools;

pub fn solve_a(loader: &DataLoader) -> Result<String, &str> {
    let (i, v) = loader
        .data()
        .chars()
        .slide(4)
        .enumerate()
        .find(|(i, cs)| cs.iter().all_unique())
        .ok_or("no match")?;

    Ok((i + 4).to_string())
}

pub fn solve_b(loader: &DataLoader) -> Result<String, &str> {
    let (i, v) = loader
        .data()
        .chars()
        .slide(14)
        .enumerate()
        .find(|(i, cs)| cs.iter().all_unique())
        .ok_or("no match")?;

    Ok((i + 14).to_string())
}

#[cfg(test)]
mod test_solver {
    use super::*;

    fn solve_a_test(input: &str) -> String {
        solve_a(&DataLoader::from_data(&vec![input.to_string()])).unwrap()
    }

    #[test]
    fn test_solve_a() {
        assert_eq!(
            solve_a_test("bvwbjplbgvbhsrlpgdmjqwftvncz"),
            "5".to_string()
        );
        assert_eq!(
            solve_a_test("nppdvjthqldpwncqszvftbrmjlhg"),
            "6".to_string()
        );
        assert_eq!(
            solve_a_test("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),
            "10".to_string()
        );
        assert_eq!(
            solve_a_test("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"),
            "11".to_string()
        );
    }
}
