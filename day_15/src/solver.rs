use aoc_helpers::data_loader::DataLoader;
use aoc_helpers::slide_iter::SlideIterator;
use itertools::{Itertools, Position};
use std::cmp::{max, min};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Range(i64, i64);

impl Range {
    fn new(a: i64, b: i64) -> Range {
        Range(min(a, b), max(a, b))
    }

    fn length(&self) -> usize {
        (self.1 - self.0).abs() as usize
    }

    fn merge(&mut self, x: &Range) -> bool {
        if self.0 - 1 <= x.0 && x.0 <= self.1 + 1 {
            self.1 = max(self.1, x.1);
            true
        } else if self.0 - 1 <= x.1 && x.1 <= self.1 + 1 {
            self.0 = min(self.0, x.0);
            true
        } else if x.0 < self.0 && self.1 < x.1 {
            self.0 = x.0;
            self.1 = x.1;
            true
        } else {
            false
        }
    }

    fn is_valid(&self) -> bool {
        self.0 <= self.1
    }
}

#[derive(Debug)]
struct SuperRange(Vec<Range>);

impl SuperRange {
    fn new() -> SuperRange {
        SuperRange(vec![])
    }

    fn merge(&mut self, x: Range) {
        self.0.insert(0, x);
        self.0 = SuperRange::squash(&self.0);
    }

    fn squash(sr: &Vec<Range>) -> Vec<Range> {
        let mut res = vec![];
        let mut merged_happend = false;

        'outer: for r_to_merge in sr.iter() {
            if res.is_empty() {
                res.push(*r_to_merge);
                continue 'outer;
            }

            for r in res.iter_mut() {
                if r.merge(r_to_merge) {
                    merged_happend = true;
                    continue 'outer;
                }
            }

            res.push(*r_to_merge);
        }

        if merged_happend {
            SuperRange::squash(&res)
        } else {
            res.sort();
            res
        }
    }

    fn length_sum(&self) -> usize {
        self.0.iter().map(|r| r.length()).sum::<usize>()
    }

    fn trim(&mut self, x: Range) {
        for r in self.0.iter_mut() {
            if r.0 < x.0 {
                r.0 = x.0
            }
            if r.1 > x.1 {
                r.1 = x.1
            }
        }

        self.0 = self
            .0
            .iter()
            .filter(|r| r.is_valid())
            .cloned()
            .collect::<Vec<Range>>();
    }

    fn range_counts(&self) -> usize {
        self.0.len()
    }

    fn get_inverted_super_range(&self) -> Option<SuperRange> {
        if self.range_counts() < 2 {
            return None;
        }

        Some(SuperRange(
            self.0
                .iter()
                .slide(2)
                .map(|v| Range::new(v[0].1 + 1, v[1].0 - 1))
                .collect::<Vec<Range>>(),
        ))
    }
}

type Point = (i64, i64);
type Points = (Point, Point);

fn get_points(loader: &DataLoader) -> Vec<Points> {
    loader
        .iter()
        .map(|line| {
            line.split(' ')
                .filter(|s| s.contains('='))
                .map(|v| v.split_once('=').unwrap().1)
                .with_position()
                .map(|pos_val| match pos_val {
                    Position::First(val) | Position::Middle(val) => &val[..val.len() - 1],
                    Position::Last(val) => val,
                    Position::Only(_) => panic!("should never be the only"),
                })
                .map(|val| val.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .map(|vals| ((vals[0], vals[1]), (vals[2], vals[3])))
        .collect::<Vec<((i64, i64), (i64, i64))>>()
}

fn get_taxi_distance(((x1, y1), (x2, y2)): &Points) -> usize {
    ((x1 - x2).abs() + (y1 - y2).abs()) as usize
}

fn get_range_on_row(ps: &Points, row: i64) -> Option<Range> {
    let ((x_sender, y_sender), (x2, y2)) = ps;
    let ps_dist = get_taxi_distance(ps);
    let row_to_sender_dist = (row - y_sender).abs() as usize;

    if row_to_sender_dist <= ps_dist {
        let remaining = ps_dist - row_to_sender_dist;
        Some(Range::new(
            x_sender - remaining as i64,
            x_sender + remaining as i64,
        ))
    } else {
        None
    }
}

pub fn solve_a(loader: &DataLoader, row: i64) -> Result<String, &str> {
    let super_range = get_points(loader)
        .iter()
        .map(|ps| get_range_on_row(ps, row))
        .flatten()
        .fold(SuperRange::new(), |mut acc, range| {
            acc.merge(range);
            acc
        });

    Ok(super_range.length_sum().to_string())
}

pub fn solve_b(loader: &DataLoader, max_xy: i64) -> Result<String, &str> {
    let points = get_points(loader);
    let mut inverted_super_ranges = vec![];
    for row in 0..=max_xy {
        let mut super_range = points
            .iter()
            .map(|ps| get_range_on_row(ps, row))
            .flatten()
            .fold(SuperRange::new(), |mut acc, range| {
                acc.merge(range);
                acc
            });
        super_range.trim(Range::new(0, max_xy));
        if super_range.range_counts() > 1 {
            inverted_super_ranges.push((row, super_range.get_inverted_super_range().unwrap()));
        }
    }
    let (y, sr)= inverted_super_ranges.first().unwrap();
    let x = sr.0.first().unwrap().0;
    Ok((x * 4000000 + y).to_string())
}

#[cfg(test)]
mod test_main {
    use super::*;

    #[test]
    fn super1() {
        let mut sr = SuperRange::new();
        for r in [
            Range(-1, 5),
            Range(8, 16),
            Range(12, 16),
            Range(6, 10),
            Range(0, 0),
            Range(12, 28),
            Range(17, 17),
        ] {
            sr.merge(r);
        }
        assert_eq!(sr.range_counts(), 1);
    }

    #[test]
    fn super2() {
        let mut sr = SuperRange::new();
        for r in [Range(-1, 5), Range(8, 16), Range(12, 16), Range(6, 10)] {
            sr.merge(r);
        }
        assert_eq!(sr.range_counts(), 1);
    }

    #[test]
    fn super3() {
        let mut sr = SuperRange::new();
        for r in [Range(6, 16), Range(-1, 5), Range(0, 0)] {
            sr.merge(r);
        }
        assert_eq!(sr.range_counts(), 1);
    }

    #[test]
    fn range_merge1() {
        let mut r = Range::new(0, 0);
        assert!(r.merge(&Range::new(-1, 5)));
    }
    #[test]
    fn range_merge2() {
        let mut r = Range::new(-1, 5);
        assert!(r.merge(&Range::new(6, 16)));
    }
    #[test]
    fn range_merge3() {
        let mut r = Range::new(6, 16);
        assert!(r.merge(&Range::new(-1, 5)));
    }
}
