use aoc_helpers::data_loader::DataLoader;
use itertools::Itertools;

use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, Copy)]
enum Op {
    Add,
    Sub,
    Div,
    Mul,
}

impl Op {
    fn from_char(c: char) -> Op {
        match c {
            '+' => Op::Add,
            '-' => Op::Sub,
            '*' => Op::Mul,
            '/' => Op::Div,
            _ => panic!("unknown operation"),
        }
    }
}

type Monkey = String;

#[derive(Debug, Clone)]
struct Equation {
    res: Monkey,
    left: Monkey,
    op: Op,
    right: Monkey,
}

impl Equation {
    fn get_res(&self, left: i64, right: i64) -> i64 {
        match self.op {
            Op::Add => left + right,
            Op::Sub => left - right,
            Op::Mul => left * right,
            Op::Div => left / right,
        }
    }

    fn get_left(&self, res: i64, right: i64) -> i64 {
        match self.op {
            Op::Add => res - right,
            Op::Sub => res + right,
            Op::Mul => res / right,
            Op::Div => res * right,
        }
    }

    fn get_right(&self, res: i64, left: i64) -> i64 {
        match self.op {
            Op::Add => res - left,
            Op::Sub => left - res,
            Op::Mul => res / left,
            Op::Div => left / res,
        }
    }
}

#[derive(Debug)]
struct Context {
    lookup: HashMap<Monkey, i64>,
    equations: VecDeque<Equation>,
}

impl Context {
    fn new() -> Context {
        Context {
            lookup: HashMap::new(),
            equations: VecDeque::new(),
        }
    }

    fn from_loader(loader: &DataLoader) -> Context {
        let mut ctx = Context::new();

        for line in loader.iter() {
            let split = line.split(' ').collect_vec();

            if split.len() == 4 {
                let res = split[0].replace(":", "");
                let left = split[1].to_string();
                let op = Op::from_char(split[2].chars().next().unwrap());
                let right = split[3].to_string();
                ctx.equations.push_back(Equation {
                    res,
                    left,
                    op,
                    right,
                });
            } else if split.len() == 2 {
                let name = split[0].replace(":", "");
                let num = split[1].parse::<i64>().unwrap();
                ctx.lookup.insert(name, num);
            } else {
                panic!("line not parable");
            }
        }
        ctx
    }

    fn solve_upwards(&mut self) {
        let mut abort_counter = 0;

        while abort_counter < self.equations.len() && let Some(eq) = self.equations.pop_front() {
            if let Some(left) = self.lookup.get(&eq.left)
                && let Some(right) = self.lookup.get(&eq.right) {
                let res = eq.get_res(*left, *right);

                self.lookup.insert(eq.res.clone(), res);
                abort_counter = 0;
            } else {
                self.equations.push_back(eq);
                abort_counter += 1;
            }
        }
    }

    fn solve_downwards(&mut self) {
        let mut abort_counter = 0;

        while abort_counter < self.equations.len() && let Some(eq) = self.equations.pop_front() {
            if let Some(res) = self.lookup.get(&eq.res) {
                if let Some(left) = self.lookup.get(&eq.left) {
                    let right = eq.get_right(*res, *left);
                    self.lookup.insert(eq.right.clone(), right);
                    abort_counter = 0;
                    continue;
                } else if let Some(right) = self.lookup.get(&eq.right) {
                    let left = eq.get_left(*res, *right);
                    self.lookup.insert(eq.left.clone(), left);
                    abort_counter = 0;
                    continue;
                }
            }
            self.equations.push_back(eq);
            abort_counter += 1;
        }
    }
}

pub fn solve_a(loader: &DataLoader) -> Result<String, &str> {
    let mut ctx = Context::from_loader(loader);
    ctx.solve_upwards();
    let root = ctx.lookup.get("root").ok_or("root not found")?;
    Ok(root.to_string())
}

pub fn solve_b(loader: &DataLoader) -> Result<String, &str> {
    let mut ctx = Context::from_loader(loader);
    let root_eq;
    {
        let idx = ctx
            .equations
            .iter()
            .position(|eq| eq.res == "root")
            .unwrap();
        root_eq = ctx.equations[idx].clone();
        ctx.equations.remove(idx);
        ctx.lookup.remove("humn");
    }

    ctx.solve_upwards();

    {
        if let Some(left) = ctx.lookup.get(&root_eq.left) {
            ctx.lookup.insert(root_eq.right, *left);
        } else if let Some(right) = ctx.lookup.get(&root_eq.right) {
            ctx.lookup.insert(root_eq.left, *right);
        } else {
            return Err("root eq could not be resolv;ed");
        }
    }

    ctx.solve_downwards();

    let humn = ctx.lookup.get("humn").ok_or("humn not found")?;
    Ok(humn.to_string())
}
