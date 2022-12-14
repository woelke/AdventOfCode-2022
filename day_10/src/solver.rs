use aoc_helpers::data_loader::DataLoader;

use itertools::Itertools;
use std::collections::VecDeque;

#[derive(Debug)]
enum Instr {
    Noop,
    AddX(i32),
}

impl Instr {
    fn cycle_count(&self) -> usize {
        match self {
            Instr::Noop => 1,
            Instr::AddX(_) => 2,
        }
    }
}

struct Instrs(VecDeque<Instr>);

impl TryFrom<&String> for Instr {
    type Error = &'static str;

    fn try_from(x: &String) -> Result<Self, Self::Error> {
        if x == "noop" {
            return Ok(Instr::Noop);
        } else if x.starts_with("addx") {
            let str_val = x.split_once(' ').ok_or("addx wihtout a a value")?;
            let val = str_val
                .1
                .parse::<i32>()
                .map_err(|_| "the value of addx is not an integer")?;
            return Ok(Instr::AddX(val));
        }

        Err("failed to parse Instr")
    }
}

struct Cpu {
    reg_x: i32,
    cycle: usize,
    current_instr: Instr,
    remaining_cycles: usize,
    instrs: Instrs,
}

impl Cpu {
    fn new(instrs: Instrs) -> Cpu {
        Cpu {
            reg_x: 1,
            cycle: 0,
            current_instr: Instr::Noop,
            remaining_cycles: 0,
            instrs,
        }
    }

    fn fetch_next_instr(&mut self) {
        if let Some(instr) = self.instrs.0.pop_front() {
            self.current_instr = instr;
            self.remaining_cycles = self.current_instr.cycle_count();
        }
    }

    fn execute_instr(&mut self) {
        match self.current_instr {
            Instr::Noop => (),
            Instr::AddX(val) => self.reg_x += val,
        }
    }

    fn next_cycle(&mut self) {
        if self.remaining_cycles == 0 {
            self.execute_instr();
            self.fetch_next_instr();
        }

        self.cycle += 1;
        self.remaining_cycles -= 1;
    }
}

fn get_instr(loader: &DataLoader) -> Result<Instrs, &str> {
    let res = loader
        .iter()
        .map(|line| Instr::try_from(line))
        .collect::<Result<VecDeque<Instr>, &str>>()?;
    Ok(Instrs(res))
}

pub fn solve_a(loader: &DataLoader) -> Result<String, &str> {
    let instrs = get_instr(loader)?;
    let mut cpu = Cpu::new(instrs);

    let calcs = (1..=220)
        .map(|cycle| {
            cpu.next_cycle();
            let res = (cpu.cycle, cpu.reg_x);
            res
        })
        .filter(|item| (item.0 + 20) % 40 == 0)
        .collect::<Vec<(usize, i32)>>();

    let res = calcs
        .iter()
        .map(|(cycle, reg_x)| (*cycle as i32) * reg_x)
        .sum::<i32>();

    //println!("{:?}", filtered_calcs);

    Ok(res.to_string())
}

pub fn solve_b(loader: &DataLoader) -> Result<String, &str> {
    let instrs = get_instr(loader)?;
    let mut cpu = Cpu::new(instrs);

    let calcs = (1..=240)
        .map(|cycle| {
            cpu.next_cycle();
            let res = (cpu.cycle, cpu.reg_x);
            res
        })
        .collect::<Vec<(usize, i32)>>();

    let lines = calcs
        .clone()
        .into_iter()
        .chunks(40)
        .into_iter()
        .map(|iter| {
            iter.map(|(cycle, pos)| (((cycle % 40) as i32) - 1, pos))
                .map(|(screen_pos, sprite_pos)| {
                    sprite_pos - 1 <= screen_pos && screen_pos <= sprite_pos + 1
                })
                .map(|lit| if lit { '#' } else { '-' })
                .collect::<String>()
        })
        .collect::<Vec<String>>();

    lines.iter().for_each(|line| println!("{}", line));

    //println!("{:?}", calcs);

    //let res = calcs
    //.iter()
    //.map(|(cycle, reg_x)| (*cycle as i32) * reg_x)
    //.sum::<i32>();

    //println!("{:?}", filtered_calcs);

    Ok(String::new())
}
