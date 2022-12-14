use aoc_helpers::data_loader::DataLoader;
use itertools::Itertools;
use std::str::Chars;

use std::cmp::Ordering;

#[derive(Debug, Clone)]
enum Item {
    List(Vec<Item>),
    Num(i64),
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Item::Num(l), Item::Num(r)) => l.cmp(r),
            (Item::List(l), Item::List(r)) => {
                let mut iter = l.iter().zip(r.iter());
                let res = loop {
                    if let Some((a, b)) = iter.next() {
                        let cmp = a.cmp(b);
                        match cmp {
                            Ordering::Less => break cmp,
                            Ordering::Equal => continue,
                            Ordering::Greater => break cmp,
                        }
                    } else {
                        break l.len().cmp(&r.len());
                    };
                };
                res
            }
            (Item::List(_), Item::Num(_)) => self.cmp(&Item::List(vec![other.clone()])),
            (Item::Num(_), Item::List(_)) => Item::List(vec![self.clone()]).cmp(other),
        }
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl Eq for Item {}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct ItemTryFromHelper {}

impl ItemTryFromHelper {
    fn bootstraping<'a>(iter: &mut Chars<'a>) -> Result<Item, &'static str> {
        if let Some(c) = iter.next() {
            ItemTryFromHelper::try_from(iter)
        } else {
            Err("expected '[' as the first char")
        }
    }

    fn try_from<'a>(iter: &mut Chars<'a>) -> Result<Item, &'static str> {
        let mut res = vec![];
        let mut tmp_num = vec![];

        while let Some(c) = iter.next() {
            match c {
                '[' => {
                    let x = ItemTryFromHelper::try_from(iter)?;
                    res.push(x);
                }
                '0'..='9' => tmp_num.push(c),
                ',' => {
                    if tmp_num.is_empty() {
                        continue;
                    }

                    let num = tmp_num
                        .iter()
                        .collect::<String>()
                        .parse::<i64>()
                        .map_err(|_| "failed to parse to i64")?;

                    res.push(Item::Num(num));
                    tmp_num.clear();
                }
                ']' => {
                    if tmp_num.is_empty() {
                        break;
                    }

                    let num = tmp_num
                        .iter()
                        .collect::<String>()
                        .parse::<i64>()
                        .map_err(|_| "failed to parse to i64")?;

                    res.push(Item::Num(num));
                    break;
                }

                _ => return Err("unexpected char"),
            }
        }

        Ok(Item::List(res))
    }
}

impl<'a> TryFrom<&mut Chars<'a>> for Item {
    type Error = &'static str;

    fn try_from(iter: &mut Chars<'a>) -> Result<Self, Self::Error> {
        ItemTryFromHelper::bootstraping(iter)
    }
}

fn to_lrs(loader: &DataLoader) -> Result<Vec<(Item, Item)>, &str> {
    loader
        .iter()
        .chunks(3)
        .into_iter()
        .map(|mut iter| {
            if let Some(l_str) = iter.next() && let Some(r_str) = iter.next() {
                match (Item::try_from(&mut l_str.chars()), Item::try_from(&mut r_str.chars())) {
                    (Ok(left), Ok(right)) => Ok((left,right)),
                    (Err(err), _) | (_, Err(err)) => Err(err),
                }
            } else {
                Err("left or right is missing")
            }
        })
        .collect::<Result<Vec<(Item, Item)>, &str>>()
}

fn to_items(loader: &DataLoader) -> Result<Vec<Item>, &str> {
    loader
        .iter()
        .filter(|line| !line.is_empty())
        .map(|line| Item::try_from(&mut line.chars()))
        .collect::<Result<Vec<Item>, &str>>()
}

pub fn solve_a(loader: &DataLoader) -> Result<String, &str> {
    let lrs = to_lrs(loader)?;
    let res = lrs
        .iter()
        .enumerate()
        .map(|(i, (l, r))| (i + 1, l, r))
        .filter(|(_, l, r)| l < r)
        .map(|(i, _, _)| i)
        .sum::<usize>();
    Ok(res.to_string())
}

pub fn solve_b(loader: &DataLoader) -> Result<String, &str> {
    let mut items = to_items(loader)?;

    let divider_1 = Item::try_from(&mut "[[2]]".chars())?;
    let divider_2 = Item::try_from(&mut "[[6]]".chars())?;
    items.push(divider_1.clone());
    items.push(divider_2.clone());

    items.sort();

    let res = items
        .iter()
        .enumerate()
        .filter_map(|(i, item)| {
            if *item == divider_1 || *item == divider_2 {
                Some(i + 1)
            } else {
                None
            }
        })
        .product::<usize>();

    Ok(res.to_string())
}

#[cfg(test)]
mod solver_tests {
    use super::*;

    fn calc(l: &str, r: &str) -> Ordering {
        Item::try_from(&mut l.chars())
            .unwrap()
            .partial_cmp(&Item::try_from(&mut r.chars()).unwrap())
            .unwrap()
    }

    #[test]
    fn some_a_tests() {
        assert_eq!(calc("[]", "[]"), Ordering::Equal);
        assert_eq!(calc("[3]", "[3]"), Ordering::Equal);
        assert_eq!(calc("[[3]]", "[[3]]"), Ordering::Equal);
        assert_eq!(calc("[[3,999]]", "[[3,[999]]]"), Ordering::Equal);
        assert_eq!(calc("[[3,999]]]", "[[3,[999]]]"), Ordering::Equal);
        assert_eq!(calc("[[[[[[[[4]]]]]]]]", "[4]"), Ordering::Equal);
        assert_eq!(calc("[4,[[[[[[[3]]]]]]]]", "[4,3]"), Ordering::Equal);
        assert_eq!(calc("[1,1,3,1,1]", "[1,1,5,1,1]"), Ordering::Less);
        assert_eq!(calc("[[1],[2,3,4]]", "[[1],4]"), Ordering::Less);
        assert_eq!(calc("[9]", "[[8,7,6]]"), Ordering::Greater);
        assert_eq!(calc("[[4,4],4,4]", "[[4,4],4,4,4]"), Ordering::Less);
        assert_eq!(calc("[7,7,7,7]", "[7,7,7]"), Ordering::Greater);
        assert_eq!(calc("[]", "[3]"), Ordering::Less);
        assert_eq!(calc("[[[]]]", "[[]]"), Ordering::Greater);
        assert_eq!(
            calc("[1,[2,[3,[4,[5,6,7]]]],8,9]", "[1,[2,[3,[4,[5,6,0]]]],8,9]"),
            Ordering::Greater
        );
    }
}
