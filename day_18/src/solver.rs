use aoc_helpers::data_loader::DataLoader;
use std::collections::HashSet;
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

fn is_connected_to_open_area(
    point: Point,
    grid: &HashSet<Point>,
    max_point: Point,
    depth: usize,
) -> bool {
    if point.0 == -1
        || point.1 == -1
        || point.2 == -1
        || point.0 == max_point.0
        || point.1 == max_point.1
        || point.2 == max_point.2
    {
        true
    } else if depth == 0 {
        false
    } else {
        get_3d_surrounding()
            .iter()
            .map(|p| point + *p)
            .filter(|p| !grid.contains(p))
            .map(|p| is_connected_to_open_area(p, grid, max_point, depth - 1))
            .any(|x| x)
    }
}

fn count_areas_connected_to_open_area(point: Point, grid: &HashSet<Point>) -> usize {
    let max_a = *grid.iter().map(|Point(a, _, _)| a).max().unwrap();
    let max_b = *grid.iter().map(|Point(_, b, _)| b).max().unwrap();
    let max_c = *grid.iter().map(|Point(_, _, c)| c).max().unwrap();
    let max_point = Point(max_a + 1, max_b + 1, max_c + 1);

    get_3d_surrounding()
        .iter()
        .map(|p| point + *p)
        .filter(|p| !grid.contains(p))
        .filter(|p| is_connected_to_open_area(*p, grid, max_point, max_point.max() as usize))
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
    let open_areas = grid
        .iter()
        .map(|p| count_areas_connected_to_open_area(*p, &grid))
        .sum::<usize>();
    Ok(open_areas.to_string())
}
