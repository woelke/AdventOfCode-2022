use aoc_helpers::data_loader::DataLoader;
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

#[derive(Debug, Clone)]
struct ToValves(Vec<(Valve, usize)>);

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
                .collect::<Vec<(Valve, usize)>>(),
        )
    }
}

#[derive(Debug, Clone, Copy)]
enum Action {
    GoTo(Valve, usize),
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

    fn get_current_time(&self) -> usize {
        self.actions
            .iter()
            .map(|a| match a {
                Action::GoTo(_, c) => *c,
                Action::Open(_) => 1,
            })
            .sum::<usize>()
            + 1
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
    let current_time = hist.get_current_time();

    // when all valves are open or we are out of time
    if hist.opened_at_min.len() == map.0.len() || current_time >= max_time {
        hist.update_current_flow(rates, max_time);
        return hist;
    }

    // open current valve
    if let None = hist.opened_at_min.get(current_pos) {
        if rates.0.get(current_pos).unwrap() > &0 {
            hist.opened_at_min.insert(*current_pos, current_time);
            hist.actions.push(Action::Open(*current_pos));
        }
    }

    //println!("current_pos={:?}", current_pos);
    // calc best result
    map.0
        .get(current_pos)
        .unwrap()
        .0
        .iter()
        .map(|(next_valve, cost)| {
            let mut tmp_hist = hist.clone();
            tmp_hist.actions.push(Action::GoTo(*next_valve, *cost));
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
    let shrinked_map = map.shrinked_map(&rates);
    println!("shrinked map={:?}\n", shrinked_map);
    let shrinked_map = shrinked_map.removed_loopes();
    println!("no loop map={:?}\n", shrinked_map);
    let start_valve = Valve::from("AA");
    //let history = calc_optimal_route(&map, &rates, 30, &start_valve);
    let history = calc_optimal_route(&shrinked_map, &rates, 30, &start_valve);
    println!("History={:?}\n", history);
    println!(
        "open order={:?}\n",
        history
            .actions
            .iter()
            .filter(|a| if let Action::Open(_) = a { true } else { false })
            .cloned()
            .collect::<Vec<Action>>()
    );

    Ok(history.current_flow.to_string())
}

#[cfg(test)]
mod test_main {
    use super::*;

    #[test]
    fn calc_origin() {
        let loader = DataLoader::from_file("data/test_input.txt");
        let rates = ValveRates::from(&loader);
        let mut hist = History::new();

        hist.opened_at_min.insert(Valve::from("DD"), 2);
        hist.opened_at_min.insert(Valve::from("BB"), 5);
        hist.opened_at_min.insert(Valve::from("JJ"), 9);
        hist.opened_at_min.insert(Valve::from("HH"), 17);
        hist.opened_at_min.insert(Valve::from("EE"), 21);
        hist.opened_at_min.insert(Valve::from("CC"), 24);

        hist.update_current_flow(&rates, 30);
        assert_eq!(hist.current_flow, 1651);
    }

    #[test]
    fn calc_first_try() {
        let loader = DataLoader::from_file("data/test_input.txt");
        let rates = ValveRates::from(&loader);
        let mut hist = History::new();

        hist.opened_at_min.insert(Valve::from("DD"), 2);
        hist.opened_at_min.insert(Valve::from("BB"), 5);
        hist.opened_at_min.insert(Valve::from("JJ"), 9);
        hist.opened_at_min.insert(Valve::from("EE"), 14);
        hist.opened_at_min.insert(Valve::from("HH"), 18);
        hist.opened_at_min.insert(Valve::from("CC"), 24);

        hist.update_current_flow(&rates, 30);
        assert_eq!(hist.current_flow, 1650);
    }
}
