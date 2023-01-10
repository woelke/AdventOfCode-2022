use aoc_helpers::data_loader::DataLoader;
use aoc_helpers::matrix_helper::print_matrix;
use itertools::Itertools;
use simple_matrix::Matrix;
use std::cmp::max;
use std::collections::{HashSet, VecDeque};
use std::iter;
use std::ops::Add;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Pos(i64, i64);

impl Pos {
    fn new() -> Pos {
        Pos(0, 0)
    }

    fn x(&self) -> i64 {
        self.0
    }

    fn y(&self) -> i64 {
        self.1
    }

    fn x_mut(&mut self) -> &mut i64 {
        &mut self.0
    }

    fn y_mut(&mut self) -> &mut i64 {
        &mut self.1
    }
}

impl Add for Pos {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self(self.x() + other.x(), self.y() + other.y())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Shape {
    Bar(Pos),
    Cross(Pos),
    Angle(Pos),
    Stick(Pos),
    Block(Pos),
}

impl Shape {
    fn expansion(&self) -> Vec<Pos> {
        self.rel_expansion()
            .iter()
            .map(|p| *p + self.pos())
            .collect_vec()
    }

    fn rel_expansion(&self) -> Vec<Pos> {
        match self {
            Shape::Bar(_) => vec![Pos(0, 0), Pos(1, 0), Pos(2, 0), Pos(3, 0)],
            Shape::Cross(_) => vec![Pos(1, 0), Pos(0, -1), Pos(1, -1), Pos(2, -1), Pos(1, -2)],
            Shape::Angle(_) => vec![Pos(2, 0), Pos(2, -1), Pos(0, -2), Pos(1, -2), Pos(2, -2)],
            Shape::Stick(_) => vec![Pos(0, 0), Pos(0, -1), Pos(0, -2), Pos(0, -3)],
            Shape::Block(_) => vec![Pos(0, 0), Pos(1, 0), Pos(0, -1), Pos(1, -1)],
        }
    }

    fn height(&self) -> usize {
        let (min, max) = self
            .rel_expansion()
            .into_iter()
            .map(|p| p.y())
            .minmax()
            .into_option()
            .unwrap();
        1 + (max - min).abs() as usize
    }

    fn pos(&self) -> Pos {
        match self {
            Shape::Bar(pos) => *pos,
            Shape::Cross(pos) => *pos,
            Shape::Angle(pos) => *pos,
            Shape::Stick(pos) => *pos,
            Shape::Block(pos) => *pos,
        }
    }

    fn update_pos(self, pos: Pos) -> Shape {
        match self {
            Shape::Bar(_) => Shape::Bar(pos),
            Shape::Cross(_) => Shape::Cross(pos),
            Shape::Angle(_) => Shape::Angle(pos),
            Shape::Stick(_) => Shape::Stick(pos),
            Shape::Block(_) => Shape::Block(pos),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direct {
    Left,
    Right,
    Down,
}

impl Direct {
    fn pos(&self) -> Pos {
        match self {
            Direct::Left => Pos(-1, 0),
            Direct::Right => Pos(1, 0),
            Direct::Down => Pos(0, -1),
        }
    }
}

type GasJets = VecDeque<Direct>;

fn get_gas_jets(loader: &DataLoader) -> GasJets {
    loader
        .iter()
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            '<' => Direct::Left,
            '>' => Direct::Right,
            _ => panic!("unknown diretion"),
        })
        .collect::<VecDeque<Direct>>()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Board {
    width: usize,
    taken_spaces: HashSet<Pos>,
}

impl Board {
    pub(self) fn set_floor(&mut self) {
        for x in 0..self.width {
            self.taken_spaces.insert(Pos(x as i64, -1));
        }
    }

    fn new() -> Self {
        let mut board = Board {
            width: 7,
            taken_spaces: HashSet::new(),
        };

        board.set_floor();
        board
    }

    fn drop_shape<'a, I>(&mut self, shape: Shape, gas_jets: &mut I)
    where
        I: Iterator<Item = &'a Direct>,
    {
        let start_pos = self.get_shape_start_pos(shape);
        self.drop_shape_impl(shape.update_pos(start_pos), gas_jets);
    }

    pub(self) fn drop_shape_impl<'a, I>(&mut self, shape: Shape, gas_jets: &mut I)
    where
        I: Iterator<Item = &'a Direct>,
    {
        let to_direct = *gas_jets.next().unwrap();
        let (_, shape_1) = self.try_move_shape(shape, to_direct);
        let (moved_down, shape_2) = self.try_move_shape(shape_1, Direct::Down);

        if moved_down {
            self.drop_shape_impl(shape_2, gas_jets);
        } else {
            for pos in shape_2.expansion() {
                if !self.taken_spaces.insert(pos) {
                    panic!("space allready occupied");
                }
            }
        }
    }

    pub(self) fn try_move_shape(&mut self, shape: Shape, direct: Direct) -> (bool, Shape) {
        let next_shape = shape.update_pos(shape.pos() + direct.pos());
        match direct {
            Direct::Left => {
                if next_shape
                    .expansion()
                    .into_iter()
                    .all(|p| p.x() >= 0 && !self.taken_spaces.contains(&p))
                {
                    (true, next_shape)
                } else {
                    (false, shape)
                }
            }
            Direct::Right => {
                if next_shape
                    .expansion()
                    .into_iter()
                    .all(|p| p.x() < self.width as i64 && !self.taken_spaces.contains(&p))
                {
                    (true, next_shape)
                } else {
                    (false, shape)
                }
            }
            Direct::Down => {
                if next_shape
                    .expansion()
                    .into_iter()
                    .all(|p| !self.taken_spaces.contains(&p))
                {
                    (true, next_shape)
                } else {
                    (false, shape)
                }
            }
        }
    }

    pub(self) fn get_shape_start_pos(&self, shape: Shape) -> Pos {
        Pos(2, shape.height() as i64 + self.board_height() as i64 + 2)
    }

    fn board_height(&self) -> usize {
        if let Some(res) = self
            .taken_spaces
            .iter()
            .filter(|p| p.y() >= 0)
            .max_by(|l, r| l.y().cmp(&r.y()))
        {
            res.y() as usize + 1
        } else {
            0
        }
    }

    fn print(&self, shape: Option<Shape>) {
        let height = if let Some(s) = shape {
            max(
                self.board_height(),
                1 + s.expansion().into_iter().map(|p| p.y()).max().unwrap() as usize,
            )
        } else {
            self.board_height()
        };

        if height == 0 {
            println!("Boad is empty");
            return;
        }

        let mut mat = Matrix::from_iter(height, self.width, iter::repeat('.'));

        let shape_vec = if let Some(s) = shape {
            s.expansion()
        } else {
            vec![]
        };

        for pos in self.taken_spaces.iter().chain(shape_vec.iter()) {
            if pos.y() < 0 {
                continue;
            }
            mat.set(height - 1 - pos.y() as usize, pos.x() as usize, '#');
        }

        print_matrix(&mat);
        println!("");
    }

    fn trimmed(&self, max_height: usize) -> Board {
        let height = self.board_height();
        if height <= max_height {
            return self.clone();
        }

        let min_height = height as i64 - max_height as i64;

        let mut res = self.clone();
        res.taken_spaces = res
            .taken_spaces
            .into_iter()
            .filter_map(|p| {
                if p.y() >= min_height {
                    Some(p + Pos(0, -min_height))
                } else {
                    None
                }
            })
            .collect::<HashSet<Pos>>();

        res
    }
}

fn stack_it<'a, 'b, JET, SHAPE>(
    count: usize,
    board: Board,
    jets: &mut JET,
    shapes: &mut SHAPE,
) -> Board
where
    JET: Iterator<Item = &'a Direct>,
    SHAPE: Iterator<Item = &'b Shape>,
{
    let mut res = board.clone();

    for _ in 0..count {
        res.drop_shape(*shapes.next().unwrap(), jets);
    }

    res
}

pub fn solve_a(loader: &DataLoader) -> Result<String, &str> {
    let jets = get_gas_jets(loader);
    let board = Board::new();

    let shape_order = [
        Shape::Bar(Pos::new()),
        Shape::Cross(Pos::new()),
        Shape::Angle(Pos::new()),
        Shape::Stick(Pos::new()),
        Shape::Block(Pos::new()),
    ];
    let mut shapes_iter = shape_order.iter().cycle();
    let mut jets_iter = jets.iter().cycle();

    let board = stack_it(2022, board, &mut jets_iter, &mut shapes_iter);

    Ok(board.board_height().to_string())
}

pub fn solve_b(loader: &DataLoader) -> Result<String, &str> {
    let jets = get_gas_jets(loader);

    let shapes_order = [
        Shape::Bar(Pos::new()),
        Shape::Cross(Pos::new()),
        Shape::Angle(Pos::new()),
        Shape::Stick(Pos::new()),
        Shape::Block(Pos::new()),
    ];
    let mut shapes_iter = shapes_order.iter().cycle();
    let mut jets_iter = jets.iter().cycle();

    let cycles = jets.len() * shapes_order.len();

    let mut boards: Vec<Board> = vec![];
    boards.push(stack_it(
        cycles,
        Board::new(),
        &mut jets_iter,
        &mut shapes_iter,
    ));

    let starting_boards;
    let repeating_boards;

    loop {
        let last_board = boards.last().unwrap();
        println!("boards.len={}", boards.len());
        let board = stack_it(cycles, last_board.clone(), &mut jets_iter, &mut shapes_iter)
            .trimmed(last_board.board_height());

        if let Some(idx) = boards.iter().position(|b| b == &board) {
            starting_boards = boards.iter().take(idx).cloned().collect_vec();
            repeating_boards = boards.iter().skip(idx).cloned().collect_vec();
            break;
        } else {
            boards.push(board);
        }
    }

    println!("boards.len={}", boards.len());
    println!("starting boards.len={}", starting_boards.len());
    println!("repeating boards.len={}", repeating_boards.len());

    //board_1.trimmed(board_0.board_height()).print(None);

    Ok(String::new())
}
