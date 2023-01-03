use aoc_helpers::data_loader::DataLoader;
use cond_utils::Between;
use itertools::Itertools;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point(i64, i64, i64);

impl Point {
    fn from_line(line: &String) -> Point {
        let mut r = line.split(",");
        let a = r.next().unwrap().parse::<i64>().unwrap();
        let b = r.next().unwrap().parse::<i64>().unwrap();
        let c = r.next().unwrap().parse::<i64>().unwrap();
        Point(a, b, c)
    }

    fn max(&self) -> i64 {
        vec![self.0, self.1, self.2].into_iter().max().unwrap()
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Point(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

fn count_free_areas(point: Point, grid: &HashSet<Point>) -> usize {
    get_3d_surrounding()
        .iter()
        .map(|p| point + *p)
        .filter(|p| !grid.contains(p))
        .count()
}

fn is_connected_to_open_area(point: Point, grid: &HashSet<Point>, max_point: Point) -> bool {
    if point.0 == -1
        || point.1 == -1
        || point.2 == -1
        || point.0 == max_point.0
        || point.1 == max_point.1
        || point.2 == max_point.2
    {
        true
    } else {
        get_3d_surrounding()
            .iter()
            .map(|p| point + *p)
            .filter(|p| !grid.contains(p))
            .map(|p| is_connected_to_open_area(p, grid, max_point))
            .any(|x| x)
    }
}

fn get_open_grid(grid: &HashSet<Point>) -> HashSet<Point> {
    let a = grid
        .iter()
        .map(|Point(a, _, _)| a)
        .minmax()
        .into_option()
        .unwrap();
    let b = grid
        .iter()
        .map(|Point(_, b, _)| b)
        .minmax()
        .into_option()
        .unwrap();
    let c = grid
        .iter()
        .map(|Point(_, _, c)| c)
        .minmax()
        .into_option()
        .unwrap();

    let min = Point(a.0 - 1, b.0 - 1, c.0 - 1);
    let max = Point(a.1 + 1, b.1 + 1, c.1 + 1);

    let mut res = HashSet::new();

    let mut jobs = VecDeque::new();
    jobs.push_back(min);
    res.insert(min);

    while !jobs.is_empty() {
        let job = jobs.pop_front().unwrap();
        for p in get_3d_surrounding().iter().map(|p| job + *p) {
            let Point(a, b, c) = p;
            let x = a.within(min.0, max.0);
            if a.within(min.0, max.0)
                && b.within(min.1, max.1)
                && c.within(min.2, max.2)
                && !grid.contains(&p)
                && !res.contains(&p)
            {
                jobs.push_back(p);
                res.insert(p);
            }
        }
    }

    res
}

fn count_open_areas(point: Point, open_areas: &HashSet<Point>) -> usize {
    get_3d_surrounding()
        .iter()
        .map(|p| point + *p)
        .filter(|p| open_areas.contains(p))
        .count()
}

fn get_3d_surrounding() -> Vec<Point> {
    vec![
        Point(1, 0, 0),
        Point(-1, 0, 0),
        Point(0, 1, 0),
        Point(0, -1, 0),
        Point(0, 0, 1),
        Point(0, 0, -1),
    ]
}

fn get_grid(loader: &DataLoader) -> HashSet<Point> {
    loader
        .iter()
        .map(|line| Point::from_line(line))
        .collect::<HashSet<Point>>()
}

pub fn solve_a(loader: &DataLoader, row: i64) -> Result<String, &str> {
    let grid = get_grid(loader);
    let free_areas = grid
        .iter()
        .map(|p| count_free_areas(*p, &grid))
        .sum::<usize>();
    Ok(free_areas.to_string())
}

pub fn solve_b(loader: &DataLoader, max_xy: i64) -> Result<String, &str> {
    let grid = get_grid(loader);
    let open_grid = get_open_grid(&grid);
    let count = grid
        .iter()
        .map(|p| count_open_areas(*p, &open_grid))
        .sum::<usize>();
    Ok(count.to_string())
}
