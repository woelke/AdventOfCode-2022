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
        }
    }

    fn move_oposite(&mut self, step: Step) {
        match step {
            Step::R => self.move_pos(Step::L),
            Step::U => self.move_pos(Step::D),
            Step::L => self.move_pos(Step::R),
            Step::D => self.move_pos(Step::U),
        }
    }

    fn distance(&self, pos: Pos) -> f64 {
        (((self.x - pos.x).abs().pow(2) + (self.y - pos.y).abs().pow(2)) as f64).sqrt()
    }

    fn dist_lt_diagonal(&self, pos: Pos) -> bool {
        self.distance(pos) < 2_f64.sqrt() + f64::EPSILON
    }

    fn from_pos(&mut self, pos: Pos) {
        self.x = pos.x;
        self.y = pos.y;
    }

    fn add(&self, pos: Pos) -> Pos {
        let mut res = self.clone();
        res.x += pos.x;
        res.y += pos.y;
        res
    }

    fn sub(&self, pos: Pos) -> Pos {
        let mut res = self.clone();
        res.x -= pos.x;
        res.y -= pos.y;
        res
    }

    fn is_straight(&self) -> bool {
        self.x * self.y == 0
    }

    fn makes_horizontal_line(&self, pos: Pos) -> bool {
        self.y == pos.y
    }

    fn makes_vertical_line(&self, pos: Pos) -> bool {
        self.x == pos.x
    }

    fn taxi_distance(&self, pos: Pos) -> i32 {
        let tmp = self.sub(pos);
        tmp.x.abs() + tmp.y.abs()
    }

    fn top_right_of(&self, pos: Pos) -> bool {
        self.y > pos.y && self.x > pos.x
    }

    fn top_left_of(&self, pos: Pos) -> bool {
        self.y > pos.y && self.x < pos.x
    }

    fn beneath_left_of(&self, pos: Pos) -> bool {
        self.y < pos.y && self.x < pos.x
    }

    fn beneath_right_of(&self, pos: Pos) -> bool {
        self.y < pos.y && self.x > pos.x
    }

    fn is_top_of(&self, pos: Pos) -> bool {
        self.y > pos.y
    }

    fn is_beneath_of(&self, pos: Pos) -> bool {
        self.y < pos.y
    }

    fn is_left_of(&self, pos: Pos) -> bool {
        self.x < pos.x
    }

    fn is_right_of(&self, pos: Pos) -> bool {
        self.x > pos.x
    }
}

#[derive(Debug)]
struct World {
    knots: Vec<Pos>, //idx=0 is head, idx=9 is tail
    tail_track: HashSet<Pos>,
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
        let knots: Vec<Pos> = Vec::from_iter(iter::repeat(Pos::new()).take(10));
        let mut res = World {
            knots,
            tail_track: HashSet::new(),
        };

        res.tail_track.insert(res.tail());

        res
    }

    fn head(&self) -> Pos {
        self.knots.first().unwrap().clone()
    }

    fn head_mut<'a>(&'a mut self) -> &'a mut Pos {
        self.knots.first_mut().unwrap()
    }

    fn snd<'a>(&'a self) -> &'a Pos {
        &self.knots[1]
    }

    fn snd_mut<'a>(&'a mut self) -> &'a mut Pos {
        &mut self.knots[1]
    }

    fn tail(&self) -> Pos {
        self.knots.last().unwrap().clone()
    }

    fn move_knots(&mut self, step: Step) {
        self.head_mut().move_pos(step);

        let head = self.head();
        if head.dist_lt_diagonal(self.knots[1]) {
            return;
        }

        self.knots[1].from_pos(head);
        self.knots[1].move_oposite(step);

        let mut last_relative_move = Pos::new();
        let mut now_its_vertical_time = false;
        let mut vertical_time_is_over_forever = false;

        for i in 2..self.knots.len() {
            let ahead_pos = self.knots[i - 1];
            let pos = self.knots[i];

            if ahead_pos.dist_lt_diagonal(pos) {
                return;
            }

            if pos.makes_vertical_line(ahead_pos) || pos.makes_horizontal_line(ahead_pos) {
                if pos.is_left_of(ahead_pos) {
                    self.knots[i].move_pos(Step::R);
                } else if pos.is_right_of(ahead_pos) {
                    self.knots[i].move_pos(Step::L);
                } else if pos.is_top_of(ahead_pos) {
                    self.knots[i].move_pos(Step::D);
                } else if pos.is_beneath_of(ahead_pos) {
                    self.knots[i].move_pos(Step::U);
                }

                if now_its_vertical_time {
                    vertical_time_is_over_forever = true;
                }
            } else {
                if !vertical_time_is_over_forever && now_its_vertical_time {
                    self.knots[i] = pos.add(last_relative_move);
                    continue;
                }

                if ahead_pos.top_right_of(pos) {
                    last_relative_move = Pos { x: 1, y: 1 };
                } else if ahead_pos.top_left_of(pos) {
                    last_relative_move = Pos { x: -1, y: 1 };
                } else if ahead_pos.beneath_right_of(pos) {
                    last_relative_move = Pos { x: 1, y: -1 };
                } else if ahead_pos.beneath_left_of(pos) {
                    last_relative_move = Pos { x: -1, y: -1 };
                }
                self.knots[i] = pos.add(last_relative_move);

                now_its_vertical_time = true;
            }
        }

        self.tail_track.insert(self.tail());
    }
}

fn print_map(max_row: usize, max_col: usize, world: &World) {
    //let max_row = world.tail_track.iter().map(|pos| pos.y).max().unwrap() as usize;
    //let max_col = world.tail_track.iter().map(|pos| pos.y).max().unwrap() as usize;

    let col_offset : i32 = 12;
    let row_offset :i32= 6;

    let mut mat: Matrix<char> = Matrix::from_iter(max_row + 1, max_col + 1, iter::repeat('.'));
    for pos in world.tail_track.iter() {
        mat.set(
            (pos.y + row_offset) as usize,
            (pos.x + col_offset) as usize,
            '#',
        );
    }

    mat.set((0 + row_offset) as usize, (0 + col_offset) as usize, 's');
    for (i, pos) in world.knots.iter().enumerate().rev() {
        let val;
        match i {
            0 => val = 'H',
            9 => val = 'T',
            _ => val = i.to_string().chars().next().unwrap(),
        }
        mat.set(
            (pos.y + row_offset) as usize,
            (pos.x + col_offset) as usize,
            val,
        );
    }

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
    for (cmd, count) in cmds.iter() {
        for _ in 0..*count {
            world.move_knots(*cmd);
        }
    }
}

fn run_cmds_debug(cmds: &Vec<(Step, i32)>, world: &mut World) {
    println!("Start World: {:?}", world);
    for (cmd, count) in cmds.iter() {
        println!("next cmd: {:?}; count: {}", cmd, count);
        for i in 0..*count {
            //println!("next cmd: {:?}; count: {}/{}", cmd, i, count);
            world.move_knots(*cmd);
            //print_map(21, 26, &world);
        }
        print_map(22, 27, &world);
        //println!("World: {:?}", world);
        println!("");
    }
}

pub fn solve_b(loader: &DataLoader) -> Result<String, &str> {
    let mut world = World::new();
    let cmds = get_cmds(loader)?;
    run_cmds(&cmds, &mut world);
    //run_cmds_debug(&cmds, &mut world);

    //Ok(String::new())
    Ok(world.tail_track.len().to_string())
}
