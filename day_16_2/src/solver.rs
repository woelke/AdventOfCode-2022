use aoc_helpers::data_loader::DataLoader;
//use itertools::Itertools;
use std::collections::HashMap;
use comparator::collections::BinaryHeap;
use std::fmt;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Valve(u8, u8);

impl Valve {
    fn from(raw_valve: &str) -> Valve {
        Valve(
            raw_valve.chars().nth(0).unwrap() as u8,
            raw_valve.chars().nth(1).unwrap() as u8,
        )
    }

    fn from_line(line: &str) -> Valve {
        line.split(" ")
            .enumerate()
            .filter_map(|(i, s)| match i {
                1 => Some(Valve::from(s)),
                _ => None,
            })
            .nth(0)
            .unwrap()
    }
}

impl fmt::Debug for Valve {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.0 as char, self.1 as char)
    }
}

type Cost = usize;

#[derive(Debug, Clone)]
struct ToValves(Vec<(Valve, Cost)>);

impl ToValves {
    fn from_line(line: &String) -> ToValves {
        ToValves(
            line.split_once("; ")
                .unwrap()
                .1
                .replace("s", "")
                .replace("tunnel lead to valve ", "")
                .split(',')
                .map(|s| (Valve::from(s.trim()), 1))
                .collect::<Vec<(Valve, Cost)>>(),
        )
    }
}

#[derive(Debug)]
struct ValveRates(HashMap<Valve, u64>);

impl ValveRates {
    fn from(loader: &DataLoader) -> ValveRates {
        ValveRates(
            loader
                .iter()
                .map(|line| {
                    line.split(" ")
                        .enumerate()
                        .filter_map(|(i, s)| match i {
                            1 => Some(s),
                            4 => Some({
                                let res = s.split_once("=").unwrap().1;
                                res.get(..res.len() - 1).unwrap()
                            }),
                            _ => None,
                        })
                        .collect::<Vec<&str>>()
                })
                .map(|v| (Valve::from(v[0]), v[1].parse::<u64>().unwrap()))
                .collect::<HashMap<Valve, u64>>(),
        )
    }
}

#[derive(Debug, Clone)]
struct CaveMap(HashMap<Valve, ToValves>);

impl CaveMap {
    fn from(loader: &DataLoader) -> CaveMap {
        CaveMap(
            loader
                .iter()
                .map(|line| (Valve::from_line(line), ToValves::from_line(line)))
                .collect::<HashMap<Valve, ToValves>>(),
        )
    }

    fn shrinked_map(&self, rates: &ValveRates) -> CaveMap {
        let mut res = self.clone();
        for useless_valve in rates.0.iter().filter_map(|(v, r)| {
            if r == &0 && *v != Valve::from("AA") {
                Some(v)
            } else {
                None
            }
        }) {
            CaveMap::remove_useless_valve(&mut res, useless_valve);
        }

        res
    }

    fn remove_useless_valve(map: &mut CaveMap, valve: &Valve) {
        let replacements = map
            .0
            .remove(valve)
            .unwrap()
            .0
            .iter()
            .filter(|(v, _)| v != valve)
            .cloned()
            .collect::<Vec<(Valve, usize)>>();

        for (_, vs) in map.0.iter_mut() {
            if let Some((idx, cost)) =
                vs.0.iter()
                    .enumerate()
                    .find_map(|(i, (v, c))| if v == valve { Some((i, *c)) } else { None })
            {
                vs.0.remove(idx);
                vs.0.extend(replacements.iter().map(|(v, c)| (*v, c + cost)));
            }
        }
    }

    fn removed_loopes(&self) -> CaveMap {
        let mut res = self.clone();
        for (v, vs) in res.0.iter_mut() {
            vs.0.retain(|(w, _)| v != w);
        }
        res
    }
}

//fn calc_shortest_path(start: Pos, mat: &Matrix<char>) -> Matrix<u64> {
    //let mut res: Matrix<u64> = Matrix::from_iter(mat.rows(), mat.cols(), iter::repeat(u64::MAX));
    //let mut jobs = BinaryHeap::with_comparator(comparing(|job: &Job| u64::MAX - job.steps()));

    //res.set(start.row(), start.col(), 0);
    //jobs.push(Job(0, start));

    //while let Some(job) = jobs.pop() {
        //let next_step_count = job.steps() + 1;

        //let next_poses = get_sorounding_poses(job.pos())
            //.into_iter()
            //.filter(|p| p.in_boundary_of(mat))
            //.filter(|p| res.get(p.row(), p.col()).unwrap() > &next_step_count)
            //.filter(|p| {
                //(mat.get(p.row(), p.col()).unwrap().clone() as u8)
                    //<= (mat.get(job.pos().row(), job.pos().col()).unwrap().clone() as u8) + 1
            //})
            //.collect::<Vec<Pos>>();

        //for next_pos in next_poses {
            //res.set(next_pos.row(), next_pos.col(), next_step_count);
            //jobs.push(Job(next_step_count, next_pos));
        //}
    //}

    //res
//}

fn get_shortest_pahts(from: &Valve, map: &CaveMap) -> HashMap<(Valve, Valve), Cost>{
    let mut res: HashMap<(Valve, Valve), Cost> = HashMap::new();
    //let mut jobs = BinaryHeap::with_comparator(comparing(|job: &Job| u64::MAX - job.steps()));

    while true {

    }
    todo!();
}

pub fn solve_a(loader: &DataLoader) -> Result<String, &str> {
    let rates = ValveRates::from(loader);
    let map = CaveMap::from(loader).shrinked_map(&rates).removed_loopes();

    println!("map={:?}\n", map);
    println!("rates={:?}\n", rates);

    Ok(String::new())
}

pub fn solve_b(loader: &DataLoader) -> Result<String, &str> {
    Ok(String::new())
}
