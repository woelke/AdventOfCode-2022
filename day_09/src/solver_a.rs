use aoc_helpers::data_loader::DataLoader;
use simple_matrix::Matrix;
use std::iter;

use std::collections::HashSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Step {
    R,
    U,
    L,
    D,
    Nop,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn new() -> Pos {
        Pos { x: 0, y: 0 }
    }

    fn move_pos(&mut self, step: Step) {
        match step {
            Step::R => self.x += 1,
            Step::U => self.y += 1,
            Step::L => self.x -= 1,
            Step::D => self.y -= 1,
            Step::Nop => (),
        }
    }

    fn move_oposite(&mut self, step: Step) {
        match step {
            Step::R => self.move_pos(Step::L),
            Step::U => self.move_pos(Step::D),
            Step::L => self.move_pos(Step::R),
            Step::D => self.move_pos(Step::U),
            Step::Nop => (),
        }
    }

    fn distance(&self, pos: Pos) -> f64 {
        (((self.x - pos.x).abs().pow(2) + (self.y - pos.y).abs().pow(2)) as f64).sqrt()
    }
}

#[derive(Debug)]
struct World {
    head: Pos,
    tail: Pos,
    tail_track: HashSet<Pos>,
    head_move: Step,
}

impl TryFrom<&str> for Step {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.trim() {
            "R" => Ok(Step::R),
            "U" => Ok(Step::U),
            "L" => Ok(Step::L),
            "D" => Ok(Step::D),
            _ => Err("Failed to parse Step"),
        }
    }
}

impl World {
    fn new() -> World {
        let mut res = World {
            head: Pos::new(),
            tail: Pos::new(),
            tail_track: HashSet::new(),
            head_move: Step::Nop,
        };

        res.tail_track.insert(res.tail);
        res
    }

    fn move_head(&mut self, step: Step) {
        self.head.move_pos(step);
        self.head_move = step;
    }

    fn follow_with_tail(&mut self) {
        if self.head.distance(self.tail) < 2_f64.sqrt() + f64::EPSILON {
            return;
        }

        self.tail = self.head;
        self.tail.move_oposite(self.head_move);

        self.tail_track.insert(self.tail);
    }
}

fn print_map(max_row: usize, max_col: usize, world: &World) {
    //let max_row = world.tail_track.iter().map(|pos| pos.y).max().unwrap() as usize;
    //let max_col = world.tail_track.iter().map(|pos| pos.y).max().unwrap() as usize;

    let mut mat: Matrix<char> = Matrix::from_iter(max_row + 1, max_col + 1, iter::repeat('.'));
    for pos in world.tail_track.iter() {
        mat.set(pos.y as usize, pos.x as usize, '#');
    }

    mat.set(0, 0, 's');
    mat.set(world.tail.y as usize, world.tail.x as usize, 'T');
    mat.set(world.head.y as usize, world.head.x as usize, 'H');

    for row_idx in (0..max_row).rev() {
        let row = mat.get_row(row_idx).unwrap();
        for c in row {
            print!("{}", c);
        }
        println!("");
    }
    println!("");
}

fn get_cmds(loader: &DataLoader) -> Result<Vec<(Step, i32)>, &str> {
    let splited_input = loader
        .iter()
        .map(|line| line.split_once(' '))
        .collect::<Option<Vec<(&str, &str)>>>()
        .ok_or("Failed to split once")?;

    let parsed_input = splited_input
        .iter()
        .map(|(l, r)| (Step::try_from(*l).unwrap(), r.parse::<i32>().unwrap()))
        .collect::<Vec<(Step, i32)>>();

    Ok(parsed_input)
}

fn run_cmds(cmds: &Vec<(Step, i32)>, world: &mut World) {
    //println!("Start World: {:?}", world);
    for (cmd, count) in cmds.iter() {
        //println!("next cmd: {:?}; count: {}", cmd, count);
        for _ in 0..*count {
            world.move_head(*cmd);
            world.follow_with_tail();
        }
        //println!("World: {:?}", world);
        //print_map(10, 10, &world);
        //println!("");
    }
}

pub fn solve_a(loader: &DataLoader) -> Result<String, &str> {
    let mut world = World::new();
    let cmds = get_cmds(loader)?;
    run_cmds(&cmds, &mut world);

    Ok(world.tail_track.len().to_string())
}
