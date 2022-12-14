fn preprocess_input(raw_input: &Vec<String>) -> Vec<Vec<i32>> {
    let mut input: Vec<Vec<i32>> = Vec::new();
    let mut tmp: Vec<i32> = Vec::new();

    for line in raw_input.iter() {
        match line.as_str() {
            "" => {
                input.push(tmp.clone());
                tmp.clear();
            }
            _ => tmp.push(line.parse::<i32>().unwrap()),
        }
    }
    input.push(tmp.clone());

    input
}

pub fn solve_a(raw_input: &Vec<String>) -> String {
    let elfs = preprocess_input(&raw_input);

    elfs.iter()
        .map(|elf| elf.iter().sum::<i32>())
        .max()
        .unwrap()
        .to_string()
}

pub fn solve_b(raw_input: &Vec<String>) -> String {
    let elfs = preprocess_input(&raw_input);

    elfs.iter()
        .map(|elf| elf.iter().sum::<i32>())
        .fold(vec![0, 0, 0], |mut acc, val| {
            acc.push(val);
            acc.sort();
            acc.remove(0);
            acc
        })
        .iter()
        .sum::<i32>()
        .to_string()
}
