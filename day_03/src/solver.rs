use aoc_helpers::data_loader::DataLoader;
use std::collections::HashSet;

fn get_groups(loader: &DataLoader) -> Vec<Vec<&String>> {
    let mut res: Vec<Vec<&String>> = Vec::new();
    let mut tmp: Vec<&String> = Vec::new();

    for (i, line) in loader.iter().enumerate() {
        tmp.push(line);

        if i % 3 == 2 {
            res.push(tmp.clone());
            tmp.clear();
        }
    }

    res
}

fn get_prio(item: char) -> Option<i32> {
    match item {
        'a'..='z' => Some((item as i32) - ('a' as i32) + 1),
        'A'..='Z' => Some((item as i32) - ('A' as i32) + 27),
        other => None,
    }
}

fn find_duplicates(l: &str, r: &str) -> Vec<char> {
    let lh: HashSet<char> = HashSet::from_iter(l.chars());
    let rh: HashSet<char> = HashSet::from_iter(r.chars());

    let mut res: Vec<char> = vec![];
    for c in lh.intersection(&rh) {
        res.push(c.clone())
    }
    res
}

fn find_duplicate_in_group(group: &Vec<&String>) -> char {
    let mut acc = group[0].clone();

    for line in group.iter() {
        acc = String::from_iter(find_duplicates(acc.as_str(), line.as_str()));
    }

    if acc.len() != 1 {
        for line in group.iter() {
            println!("{line}");
        }

        panic!("one result expected, but got <{acc}>");
    }

    acc.chars().next().unwrap()
}

pub fn solve_a(loader: &DataLoader) -> String {
    loader
        .iter()
        .map(|line| line.split_at(line.len() / 2))
        .map(|(a, b)| find_duplicates(a, b))
        .flatten()
        .map(|c| get_prio(c).unwrap())
        .sum::<i32>()
        .to_string()
}

pub fn solve_b(raw_input: &DataLoader) -> String {
    get_groups(raw_input)
        .iter()
        .map(|group| find_duplicate_in_group(group))
        .map(|c| get_prio(c).unwrap())
        .sum::<i32>()
        .to_string()
}

#[cfg(test)]
mod test_solver {
    use super::*;

    #[test]
    fn test_prio() {
        assert_eq!(get_prio('p').unwrap(), 16);
        assert_eq!(get_prio('L').unwrap(), 38);
        assert_eq!(get_prio('P').unwrap(), 42);
        assert_eq!(get_prio('v').unwrap(), 22);
    }

    #[test]
    fn test_duplicates() {
        assert_eq!(find_duplicates("vJrwpWtwJgWr", "hcsFMMfFFhFp"), vec!['p']);
        assert_eq!(find_duplicates("PmmdzqPrV", "vPwwTWBwg"), vec!['P']);
    }

    #[test]
    fn test_get_groups() {
        let loader = DataLoader::from_file("data/test_input.txt");
        let groups = get_groups(&loader);

        assert_eq!(groups.len(), 2);
        assert_eq!(groups[0].len(), 3);
        assert_eq!(groups[1].len(), 3);
    }

    #[test]
    fn test_find_duplicate_in_group() {
        assert_eq!(
            find_duplicate_in_group(&vec![
                &"vJrwpWtwJgWrhcsFMMfFFhFp".to_string(),
                &"jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string(),
                &"PmmdzqPrVvPwwTWBwg".to_string()
            ]),
            'r'
        );
    }
}
