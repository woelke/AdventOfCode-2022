#[derive(Debug)]
enum Outcome {
    Loss,
    Draw,
    Win,
}

impl Outcome {
    fn value(&self) -> i32 {
        match self {
            Outcome::Loss => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }
}

impl TryFrom<char> for Outcome {
    type Error = &'static str;

    fn try_from(x: char) -> Result<Self, Self::Error> {
        match x {
            'X' => Ok(Outcome::Loss),
            'Y' => Ok(Outcome::Draw),
            'Z' => Ok(Outcome::Win),
            _ => Err("Unkown char {x}"),
        }
    }
}

#[derive(Debug, Clone)]
enum Item {
    Paper,
    Rock,
    Scissors,
}

impl TryFrom<char> for Item {
    type Error = &'static str;

    fn try_from(x: char) -> Result<Self, Self::Error> {
        match x {
            'A' | 'X' => Ok(Item::Rock),
            'B' | 'Y' => Ok(Item::Paper),
            'C' | 'Z' => Ok(Item::Scissors),
            _ => Err("Unkown char {x}"),
        }
    }
}

impl Item {
    fn value(&self) -> i32 {
        match self {
            Item::Paper => 2,
            Item::Rock => 1,
            Item::Scissors => 3,
        }
    }

    fn play(&self, foe: &Item) -> Outcome {
        match (self, foe) {
            (Item::Paper, Item::Rock) => Outcome::Win,
            (Item::Paper, Item::Scissors) => Outcome::Loss,
            (Item::Rock, Item::Paper) => Outcome::Loss,
            (Item::Rock, Item::Scissors) => Outcome::Win,
            (Item::Scissors, Item::Paper) => Outcome::Win,
            (Item::Scissors, Item::Rock) => Outcome::Loss,
            _ => Outcome::Draw,
        }
    }

    fn get_choice_based_on(&self, outcome: &Outcome) -> Item {
        match (self, outcome) {
            (Item::Paper, Outcome::Win) => Item::Scissors,
            (Item::Paper, Outcome::Loss) => Item::Rock,
            (Item::Rock, Outcome::Win) => Item::Paper,
            (Item::Rock, Outcome::Loss) => Item::Scissors,
            (Item::Scissors, Outcome::Win) => Item::Rock,
            (Item::Scissors, Outcome::Loss) => Item::Paper,
            (other, _) => other.clone(),
        }
    }
}

fn a_preprocess_input(raw_input: &Vec<String>) -> Vec<(Item, Item)> {
    let mut res: Vec<(Item, Item)> = Vec::new();

    for line in raw_input.iter() {
        if line.len() != 3 {
            panic!("Unexpectd input: {line}, expectd 3 chars");
        }
        let l = line.as_str().chars().nth(0).unwrap();
        let r = line.as_str().chars().nth(2).unwrap();
        res.push((Item::try_from(l).unwrap(), Item::try_from(r).unwrap()));
    }

    res
}

fn b_preprocess_input(raw_input: &Vec<String>) -> Vec<(Item, Outcome)> {
    let mut res: Vec<(Item, Outcome)> = Vec::new();

    for line in raw_input.iter() {
        if line.len() != 3 {
            panic!("Unexpectd input: {line}, expectd 3 chars");
        }
        let l = line.as_str().chars().nth(0).unwrap();
        let r = line.as_str().chars().nth(2).unwrap();
        res.push((Item::try_from(l).unwrap(), Outcome::try_from(r).unwrap()));
    }

    res
}

pub fn solve_a(raw_input: &Vec<String>) -> String {
    a_preprocess_input(raw_input)
        .iter()
        .fold(0, |acc, (elf, me)| {
            acc + me.play(elf).value() + me.value()
        })
        .to_string()
}

pub fn solve_b(raw_input: &Vec<String>) -> String {
    b_preprocess_input(raw_input)
        .iter()
        .fold(0, |acc, (elf, me)| {
            acc + me.value() + elf.get_choice_based_on(me).value()
        })
        .to_string()
}
