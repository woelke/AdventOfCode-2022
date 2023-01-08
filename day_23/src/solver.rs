use aoc_helpers::data_loader::DataLoader;
use aoc_helpers::matrix_helper::{get_flatten_matrix, print_matrix, MatrixLoader};
use aoc_helpers::slide_iter::SlideIterator;
use itertools::{unfold, Itertools};
use simple_matrix::Matrix;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::iter;
use std::ops::Add;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Pos {
    x: i64,
    y: i64,
}

impl Add for Pos {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direct {
    N,
    NW,
    NE,
    S,
    SW,
    SE,
    E,
    W,
}

impl Direct {
    fn pos(&self) -> Pos {
        match self {
            Direct::N => Pos { x: 0, y: -1 },
            Direct::NW => Pos { x: -1, y: -1 },
            Direct::NE => Pos { x: 1, y: -1 },
            Direct::S => Pos { x: 0, y: 1 },
            Direct::SW => Pos { x: -1, y: 1 },
            Direct::SE => Pos { x: 1, y: 1 },
            Direct::E => Pos { x: 1, y: 0 },
            Direct::W => Pos { x: -1, y: 0 },
        }
    }
}

type Grid = HashSet<Pos>;
type Directs = VecDeque<Direct>;

fn get_grid(loader: &DataLoader) -> Grid {
    let mut res = HashSet::new();

    get_flatten_matrix(&loader.to_matrix::<char>().unwrap())
        .iter()
        .filter(|(_, _, c)| **c == '#')
        .for_each(|(y, x, _)| {
            res.insert(Pos {
                x: (*x as i64),
                y: (*y as i64),
            });
        });

    res
}

fn print_grid(grid: &Grid) {
    let (min_x, max_x) = grid.iter().map(|pos| pos.x).minmax().into_option().unwrap();
    let (min_y, max_y) = grid.iter().map(|pos| pos.y).minmax().into_option().unwrap();
    let x_offset = min_x;
    let y_offset = min_y;

    let mut mat = Matrix::from_iter(
        (1 + max_y - min_y) as usize,
        (1 + max_x - min_x) as usize,
        iter::repeat('.'),
    );

    for pos in grid.iter() {
        mat.set(
            (pos.y - y_offset) as usize,
            (pos.x - x_offset) as usize,
            '#',
        );
    }

    print_matrix(&mat);
    println!("");
}

fn is_clear(pos: &Pos, to_check: &[Pos], grid: &Grid) -> bool {
    to_check
        .iter()
        .map(|p| *pos + *p)
        .all(|p| !grid.contains(&p))
}

fn calc_next_round(grid: &Grid, directs: &Directs) -> Grid {
    let mut res = Grid::new();
    let mut proposed_moves: Vec<(Pos, Pos)> = Vec::new();

    for pos in grid.iter() {
        if is_clear(pos, &get_all_pos(), grid) {
            proposed_moves.push((*pos, *pos));
        } else {
            if let Some(direct) = directs.iter().find_map(|d| {
                if is_clear(pos, &get_search_poses(*d), grid) {
                    Some(d)
                } else {
                    None
                }
            }) {
                proposed_moves.push((*pos, (*pos + direct.pos())));
            } else {
                proposed_moves.push((*pos, *pos));
            }
        }
    }

    for (pos, new_pos) in proposed_moves.iter() {
        if 1 < proposed_moves
            .iter()
            .filter(|(_, n)| n == new_pos)
            .take(2)
            .count()
        {
            res.insert(*pos);
        } else {
            res.insert(*new_pos);
        }
    }

    res
}

fn get_all_pos() -> [Pos; 8] {
    [
        Direct::N.pos(),
        Direct::NE.pos(),
        Direct::E.pos(),
        Direct::SE.pos(),
        Direct::S.pos(),
        Direct::SW.pos(),
        Direct::W.pos(),
        Direct::NW.pos(),
    ]
}

fn get_search_poses(direct: Direct) -> [Pos; 3] {
    match direct {
        Direct::N => [Direct::N.pos(), Direct::NW.pos(), Direct::NE.pos()],
        Direct::S => [Direct::S.pos(), Direct::SE.pos(), Direct::SW.pos()],
        Direct::W => [Direct::W.pos(), Direct::NW.pos(), Direct::SW.pos()],
        Direct::E => [Direct::E.pos(), Direct::NE.pos(), Direct::SE.pos()],
        _ => panic!("should not happen"),
    }
}

fn calc_result(grid: &Grid) -> usize {
    let (min_x, max_x) = grid.iter().map(|pos| pos.x).minmax().into_option().unwrap();
    let (min_y, max_y) = grid.iter().map(|pos| pos.y).minmax().into_option().unwrap();

    let grid_size = (1 + max_x - min_x) * (1 + max_y - min_y);

    (grid_size as usize) - grid.len()
}

pub fn solve_a(loader: &DataLoader) -> Result<String, &str> {
    let grid = get_grid(loader);
    let directs: Directs = [Direct::N, Direct::S, Direct::W, Direct::E].into();

    println!("loader");
    print_grid(&grid);

    let rounds = unfold((grid, directs), |(g, d)| {
        *g = calc_next_round(g, d);
        let tmp = d.pop_front().unwrap();
        d.push_back(tmp);
        Some(g.clone())
    });
    let res = calc_result(&rounds.take(10).last().unwrap());
    Ok(res.to_string())
}

pub fn solve_b(loader: &DataLoader) -> Result<String, &str> {
    let grid = get_grid(loader);
    let directs: Directs = [Direct::N, Direct::S, Direct::W, Direct::E].into();

    println!("loader");
    print_grid(&grid);

    let rounds = unfold((grid, directs), |(g, d)| {
        *g = calc_next_round(g, d);
        let tmp = d.pop_front().unwrap();
        d.push_back(tmp);
        Some(g.clone())
    });
    let last_round = rounds.enumerate().slide(2).find_map(|v| {
        println!("round={}", v[1].0);
        if v[0].1 == v[1].1 {
            Some(v[1].0 + 1)
        } else {
            None
        }
    }).unwrap();
    Ok(last_round.to_string())
}
