use aoc_helpers::data_loader::DataLoader;
use std::cmp::max;
use std::collections::HashMap;
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



#[derive(Debug)]
struct Valves(Vec<Valve>);

impl Valves {
    fn from_line(line: &String) -> Valves {
        Valves(
            line.split_once("; ")
                .unwrap()
                .1
                .replace("s", "")
                .replace("tunnel lead to valve ", "")
                .split(',')
                .map(|s| Valve::from(s.trim()))
                .collect::<Vec<Valve>>(),
        )
    }
}

#[derive(Debug, Clone, Copy)]
enum Action {
    GoTo(Valve),
    Open(Valve),
}

#[derive(Debug, Clone)]
struct History {
    actions: Vec<Action>,
    opened_at_min: HashMap<Valve, usize>,
    current_flow: u64,
}

impl History {
    fn new() -> History {
        History {
            actions: vec![],
            opened_at_min: HashMap::new(),
            current_flow: 0,
        }
    }

    fn update_current_flow(&mut self, rates: &ValveRates, current_time: usize) {
        self.current_flow = self
            .opened_at_min
            .iter()
            .filter(|(v, opened)| opened < &&current_time)
            .map(|(v, opened)| {
                let rate = rates.0.get(v).unwrap();
                ((current_time - opened) as u64) * rate
            })
            .sum::<u64>();
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
                            4 => Some({let res = s.split_once("=").unwrap().1; res.get(..res.len()-1).unwrap() }),
                            _ => None,
                        })
                        .collect::<Vec<&str>>()
                })
                .map(|v| (Valve::from(v[0]), v[1].parse::<u64>().unwrap()))
                .filter(|(_, r)| r > &0)
                .collect::<HashMap<Valve, u64>>(),
        )
    }
}

#[derive(Debug)]
struct CaveMap(HashMap<Valve, Valves>);

impl CaveMap {
    fn from(loader: &DataLoader) -> CaveMap {
        CaveMap(
            loader
                .iter()
                .map(|line| (Valve::from_line(line), Valves::from_line(line)))
                .collect::<HashMap<Valve, Valves>>(),
        )
    }
}

fn calc_optimal_route(
    map: &CaveMap,
    rates: &ValveRates,
    max_time: usize,
    start_valve: &Valve,
) -> History {
    calc_optimal_route_impl(map, rates, max_time, start_valve, &History::new())
}

fn calc_optimal_route_impl(
    map: &CaveMap,
    rates: &ValveRates,
    max_time: usize,
    current_pos: &Valve,
    last_history: &History,
) -> History {
    let mut hist = last_history.clone();
    let current_time = hist.actions.len() + 1;

    // when all valves are open
    if hist.opened_at_min.len() == rates.0.len() {
        hist.update_current_flow(rates, max_time);
        return hist;
    }

    if current_time >= max_time {
        hist.update_current_flow(rates, max(current_time, max_time));
        return hist;
    }

    // open current valve
    if let None = hist.opened_at_min.get(current_pos) {
        if rates.0.contains_key(current_pos) {
            hist.opened_at_min.insert(*current_pos, current_time);
            hist.actions.push(Action::Open(*current_pos));
        }
    }

    let current_time = hist.actions.len() + 1;
    if current_time + 1 >= max_time {
        hist.update_current_flow(rates, max(current_time, max_time));
        return hist;
    }

    // calc best result
    map.0
        .get(current_pos)
        .unwrap()
        .0
        .iter()
        .map(|next_valve| {
            let mut tmp_hist = hist.clone();
            tmp_hist.actions.push(Action::GoTo(*next_valve));
            calc_optimal_route_impl(map, rates, max_time, next_valve, &tmp_hist)
        })
        .max_by(|ha, hb| ha.current_flow.cmp(&hb.current_flow))
        .unwrap()
}

pub fn solve_a(loader: &DataLoader) -> Result<String, &str> {
    let map = CaveMap::from(loader);
    println!("Map={:?}\n", map);
    let rates = ValveRates::from(loader);
    println!("Map={:?}\n", rates);
    let start_valve = Valve::from("AA");
    //let history = calc_optimal_route(&map, &rates, 30, &start_valve);
    let history = calc_optimal_route(&map, &rates, 25, &start_valve);
    println!("History={:?}\n", history);
    Ok(history.current_flow.to_string())
}

pub fn solve_b(loader: &DataLoader) -> Result<String, &str> {
    Err("xx")
}
