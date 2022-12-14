use aoc_helpers::data_loader::DataLoader;
use aoc_helpers::matrix_helper::get_flatten_matrix;
use aoc_helpers::slide_iter::SlideIterator;
use itertools::{Itertools, MinMaxResult};
use simple_matrix::Matrix;
use std::cmp::{max, min};
use std::fmt;

#[derive(Default, PartialEq)]
enum Obj {
    #[default]
    Air,
    Wall,
    Sand,
    SandEntry,
}

#[derive(Debug, Clone, Copy)]
struct Point(usize, usize);

#[derive(Debug, Clone, Copy)]
struct Wall(Point, Point);

#[derive(Debug, Clone)]
struct Walls(Vec<Wall>);

struct Cave {
    sand_entry: Point,
    map: Matrix<Obj>,
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point(x, y)
    }

    fn x(&self) -> usize {
        self.0
    }

    fn y(&self) -> usize {
        self.1
    }
}

impl fmt::Display for Obj {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Obj::Air => write!(f, "."),
            Obj::Wall => write!(f, "#"),
            Obj::Sand => write!(f, "o"),
            Obj::SandEntry => write!(f, "+"),
        }
    }
}

impl fmt::Display for Cave {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.map.rows() {
            for x in 0..self.map.cols() {
                write!(f, "{}", self.map.get(y, x).unwrap())?;
            }
            writeln!(f, "")?;
        }
        writeln!(f, "")
    }
}

impl<'a> TryFrom<&str> for Point {
    type Error = &'static str;

    fn try_from(x: &str) -> Result<Self, Self::Error> {
        let str_pair = x.split_once(',').ok_or("expected x,y pair)")?;
        let x = str_pair
            .0
            .parse::<usize>()
            .map_err(|_| "failed to parse x")?;
        let y = str_pair
            .1
            .parse::<usize>()
            .map_err(|_| "failed to parse y")?;

        Ok(Point(x, y))
    }
}

impl<'a> TryFrom<&Walls> for Cave {
    type Error = &'static str;

    fn try_from(walls: &Walls) -> Result<Self, Self::Error> {
        let (x_min, x_max) = if let MinMaxResult::MinMax(a, b) = walls
            .0
            .iter()
            .map(|wall| vec![wall.0.x(), wall.1.x()])
            .flatten()
            .minmax()
        {
            Ok((a, b))
        } else {
            Err("failed to calc x_min, x_max")
        }?;

        let y_max = if let Some(a) = walls
            .0
            .iter()
            .map(|wall| vec![wall.0.y(), wall.1.y()])
            .flatten()
            .max()
        {
            Ok(a)
        } else {
            Err("failed to calc y_max")
        }?;

        let mut map: Matrix<Obj> = Matrix::new(1 + y_max, 1 + x_max - x_min);

        for Wall(Point(x1, y1), Point(x2, y2)) in walls.0.iter() {
            if y1 == y2 {
                for x in *min(x1, x2)..=*max(x1, x2) {
                    map.set(*y1, x - x_min, Obj::Wall);
                }
            } else if x1 == x2 {
                for y in *min(y1, y2)..=*max(y1, y2) {
                    map.set(y, *x1 - x_min, Obj::Wall);
                }
            } else {
                return Err("Wall is neither horizontal nor vertical");
            }
        }

        let sand_entry = Point::new(500 - x_min, 0);
        map.set(sand_entry.y(), sand_entry.x(), Obj::SandEntry);

        Ok(Cave { sand_entry, map })
    }
}

fn to_cave(loader: &DataLoader) -> Result<Cave, &str> {
    let raw_walls = loader
        .iter()
        .map(|line| {
            line.split(" -> ")
                .map(|str_point| Point::try_from(str_point))
                .collect::<Result<Vec<Point>, &str>>()
        })
        .collect::<Result<Vec<Vec<Point>>, &str>>()?;

    let walls = Walls(
        raw_walls
            .iter()
            .map(|line| line.iter().slide(2).map(|p| Wall(*p[0], *p[1])))
            .flatten()
            .collect::<Vec<Wall>>(),
    );

    Cave::try_from(&walls)
}

impl Cave {
    fn is_on_map(&self, Point(x, y): Point) -> bool {
        y < self.map.rows() && x < self.map.cols()
    }

    fn fall_down(&self, point: Point) -> Option<Point> {
        let Point(x, y) = point;
        let p_down = Point(x, y + 1);
        if !self.is_on_map(p_down) {
            return None;
        }

        match self.map.get(p_down.y(), p_down.x()) {
            None => None,
            Some(Obj::Air) => self.fall_down(p_down),
            Some(Obj::Wall) | Some(Obj::Sand) => self.fall_down_left(point),
            _ => panic!("should not happen,"),
        }
    }

    fn fall_down_left(&self, point: Point) -> Option<Point> {
        let Point(x, y) = point;
        if x == 0 {
            return None;
        }

        let p_down_left = Point(x - 1, y + 1);
        if !self.is_on_map(p_down_left) {
            return None;
        }

        match self.map.get(p_down_left.y(), p_down_left.x()) {
            None => None,
            Some(Obj::Air) => self.fall_down(p_down_left),
            Some(Obj::Wall) | Some(Obj::Sand) => self.fall_down_right(point),
            _ => panic!("should not happen,"),
        }
    }

    fn fall_down_right(&self, point: Point) -> Option<Point> {
        let Point(x, y) = point;
        let p_down_right = Point(x + 1, y + 1);

        if !self.is_on_map(p_down_right) {
            return None;
        }

        match self.map.get(p_down_right.y(), p_down_right.x()) {
            Some(Obj::Air) => self.fall_down(p_down_right),
            None | Some(Obj::Wall) | Some(Obj::Sand) => Some(point),
            _ => panic!("should not happen,"),
        }
    }

    fn drop_sand(&mut self) -> bool {
        match self.fall_down(self.sand_entry) {
            None => false,
            Some(Point(x, y)) => self.map.set(y, x, Obj::Sand),
        }
    }

    fn count(&self, obj: Obj) -> usize {
        get_flatten_matrix(&self.map)
            .iter()
            .filter(|(_, _, o)| **o == obj)
            .count()
    }
}

pub fn solve_a(loader: &DataLoader) -> Result<String, &str> {
    let mut cave = to_cave(loader)?;
    while cave.drop_sand() {}
    Ok(cave.count(Obj::Sand).to_string())
}
