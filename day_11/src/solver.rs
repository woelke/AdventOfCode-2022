use aoc_helpers::data_loader::DataLoader;
use aoc_helpers::range::RangeIterator;

use std::fmt;
use std::rc::Rc;

use itertools::Itertools;
use std::collections::VecDeque;

#[derive(Clone)]
struct Monkey {
    id: i128,
    items: VecDeque<i128>,
    op: Rc<dyn Fn(i128) -> i128>,
    next_monkey: Rc<dyn Fn(i128) -> usize>,
    div_by: i128,
    throw_count: usize,
}

impl fmt::Display for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Monkey {}:", self.id).unwrap();
        write!(f, "  Starting items: {:?}:", self.items)
    }
}

struct MonkeyTryFromHelper {}

impl MonkeyTryFromHelper {
    fn parse_id(line: &String) -> Result<i128, &'static str> {
        let err = Err("failed to parse id");
        if line.starts_with("Monkey ") {
            if let Some((l, r)) = &line[..line.len() - 1].split_once(' ') {
                let id = r.parse::<i128>().map_err(|_| "id not an int")?;
                Ok(id)
            } else {
                err
            }
        } else {
            err
        }
    }

    fn parse_items(line: &String) -> Result<VecDeque<i128>, &'static str> {
        let trimmed_line = line.trim();
        if trimmed_line.starts_with("Starting items") {
            trimmed_line
                .split(' ')
                .skip(2)
                .map(|x| x.replace(",", ""))
                .map(|num| {
                    num.parse::<i128>()
                        .map_err(|_| "failed to parse starting item number")
                })
                .collect::<Result<VecDeque<i128>, &str>>()
        } else {
            Err("expected Starting items")
        }
    }

    fn parse_op(line: &String) -> Result<Rc<dyn Fn(i128) -> i128>, &'static str> {
        let trimmed_line = line.trim();

        if trimmed_line.starts_with("Operation: new =") {
            let values = trimmed_line.split(' ').skip(4).collect::<Vec<&str>>();
            if values.len() != 2 {
                Err("expected for Operation: new 2 values")
            } else {
                let op = values[0];
                let str_val = values[1];
                if str_val == "old" {
                    match op {
                        "*" => Ok(Rc::new(move |x: i128| x * x)),
                        "+" => Ok(Rc::new(move |x: i128| x + x)),
                        _ => Err("invalid operator"),
                    }
                } else {
                    let val = str_val
                        .parse::<i128>()
                        .map_err(|_| "in op new failed to parse value")?;
                    match op {
                        "*" => Ok(Rc::new(move |x: i128| x * val)),
                        "+" => Ok(Rc::new(move |x: i128| x + val)),
                        _ => Err("invalid operator"),
                    }
                }
            }
        } else {
            Err("expected Operation: new")
        }
    }

    fn parse_next_monkey(
        pred: &String,
        on_true: &String,
        on_false: &String,
    ) -> Result<(i128, Rc<dyn Fn(i128) -> usize>), &'static str> {
        let div_str = pred.split(" ").nth(5).ok_or("failed to get divisablor")?;
        let div_by = div_str
            .parse::<i128>()
            .map_err(|_| "failed to parse devisiblor")?;

        let on_true_str = on_true.split(" ").nth(9).ok_or("failed to get on true")?;
        let true_monkey = on_true_str
            .parse::<usize>()
            .map_err(|_| "failed to parse on true monkey")?;

        let on_false_str = on_false.split(" ").nth(9).ok_or("failed to get on false")?;
        let false_monkey = on_false_str
            .parse::<usize>()
            .map_err(|_| "failed to parse on false monkey")?;

        Ok((
            div_by,
            Rc::new(move |x: i128| {
                if x % div_by == 0 {
                    true_monkey
                } else {
                    false_monkey
                }
            }),
        ))
    }
}

impl TryFrom<&Vec<String>> for Monkey {
    type Error = &'static str;

    fn try_from(x: &Vec<String>) -> Result<Self, Self::Error> {
        let mut iter = x.iter();

        let next = iter.next().ok_or("monkey line missing")?;
        let id = MonkeyTryFromHelper::parse_id(next)?;

        let next = iter.next().ok_or("items line missing")?;
        let items = MonkeyTryFromHelper::parse_items(next)?;

        let next = iter.next().ok_or("op line missing")?;
        let op = MonkeyTryFromHelper::parse_op(next)?;

        let pred = iter.next().ok_or("pred line missing")?;
        let on_true = iter.next().ok_or("on_true line missing")?;
        let on_false = iter.next().ok_or("on_false line missing")?;
        let (div_by, next_monkey) =
            MonkeyTryFromHelper::parse_next_monkey(pred, on_true, on_false)?;

        Ok(Monkey {
            id,
            items,
            op,
            next_monkey,
            div_by,
            throw_count: 0,
        })
    }
}

fn parse_monkeys(loader: &DataLoader) -> Result<Vec<Monkey>, &str> {
    let strs = loader.iter().cloned().collect::<Vec<String>>();
    strs.into_iter()
        .chunks(7)
        .into_iter()
        .map(|iter| Monkey::try_from(&iter.collect::<Vec<String>>()))
        .collect::<Result<Vec<Monkey>, &str>>()
}

fn exec_one_monkey_round(monkeys: &mut Vec<Monkey>) {
    for i in 0..monkeys.len() {
        let mut current = monkeys[i].clone();
        monkeys[i].throw_count += monkeys[i].items.len();
        monkeys[i].items.clear();
        while let Some(item) = current.items.pop_front() {
            //println!("  - Monkey inspects an item with a worry level of {item}");
            let worry_lvl = current.op.as_ref()(item) / 3;
            //println!("  -- Monkey gets bored with item. Worry level chagnes to {worry_lvl}");
            let next = current.next_monkey.as_ref()(worry_lvl);
            //println!("  -- Item is thrown to monkey {next}");
            monkeys[next].items.push_back(worry_lvl);
        }
    }
}

fn exec_one_monkey_round_no_div(monkeys: &mut Vec<Monkey>, mod_factor: i128) {
    for i in 0..monkeys.len() {
        let mut current = monkeys[i].clone();
        monkeys[i].throw_count += monkeys[i].items.len();
        monkeys[i].items.clear();
        while let Some(item) = current.items.pop_front() {
            //println!("  - Monkey inspects an item with a worry level of {item}");
            let worry_lvl = current.op.as_ref()(item);
            //println!("  -- Monkey gets bored with item. Worry level chagnes to {worry_lvl}");
            let next = current.next_monkey.as_ref()(worry_lvl);
            //println!("  -- Item is thrown to monkey {next}");
            monkeys[next].items.push_back(worry_lvl % mod_factor);
        }
    }
}

pub fn solve_a(loader: &DataLoader) -> Result<String, &str> {
    let mut monkeys = parse_monkeys(loader)?;
    for round in 1..=20 {
        exec_one_monkey_round(&mut monkeys);
        //println!("-------- After round {} ------------", round);
        //monkeys.iter().for_each(|monkey| println!("{monkey}\n"));
    }

    Ok(monkeys
        .iter()
        .map(|m| m.throw_count)
        .max_x(2)
        .product::<usize>()
        .to_string())
}

pub fn solve_b(loader: &DataLoader) -> Result<String, &str> {
    let mut monkeys = parse_monkeys(loader)?;
    let mod_factor = monkeys.iter().map(|m| m.div_by).product();

    println!("mod_factor={mod_factor}");

    for round in 1..=10000 {
        exec_one_monkey_round_no_div(&mut monkeys, mod_factor);
        //println!("-------- After round {} ------------", round);
        //monkeys.iter().for_each(|monkey| println!("{monkey}\n"));
    }

    monkeys.iter().for_each(|monkey| println!("{monkey}\n"));

    Ok(monkeys
        .iter()
        .map(|m| m.throw_count)
        .max_x(2)
        .map(|x| x as i128)
        .product::<i128>()
        .to_string())
}
