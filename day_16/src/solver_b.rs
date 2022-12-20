use aoc_helpers::data_loader::DataLoader;
use itertools::Itertools;
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

type Cost = usize;

#[derive(Debug, Clone, Copy)]
enum Action {
    GoTo(Valve, Cost),
    Open(Valve),
}

#[derive(Debug, Clone)]
struct History {
    actions_a: Vec<Action>,
    actions_b: Vec<Action>,
    opened_at_min: HashMap<Valve, usize>,
    current_flow: u64,
    current_time: usize,
    pos_a: Valve,
    pos_b: Valve,
}

impl History {
    fn new() -> History {
        History {
            actions_a: vec![],
            actions_b: vec![],
            opened_at_min: HashMap::new(),
            current_flow: 0,
            current_time: 0,
            pos_a: Valve::from("AA"),
            pos_b: Valve::from("AA"),
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

struct CalcContext {
    map: CaveMap,
    rates: ValveRates,
    max_time: usize,
}

fn calc_optimal_route(ctx: &CalcContext) -> History {
    calc_optimal_route_impl(ctx, &History::new())
}

fn get_gotos(pos: &Valve, map: &CaveMap, actions: &Vec<Action>) -> Vec<Action> {
    let last_goto: Option<&Action> = actions.last();
    map.0
        .get(pos)
        .unwrap()
        .0
        .iter()
        .filter_map(|(v, c)| {
            if let Some(Action::GoTo(last_v, _)) = last_goto && last_v == v {
                    None
            } else {
                Some(Action::GoTo(*v, *c))
            }
        })
        .collect::<Vec<Action>>()
}

fn try_waiting(actions: &Vec<Action>) -> Option<Action> {
    if let Some(Action::GoTo(v, c)) = actions.last() {
        if c > &1 {
            return Some(Action::GoTo(*v, c - 1));
        }
    }
    None
}

fn try_open(pos: &Valve, rates: &ValveRates, hist: &History) -> Option<Action> {
    if let None = hist.opened_at_min.get(pos) {
        if rates.0.get(pos).unwrap() > &0 {
            return Some(Action::Open(*pos));
        }
    }
    None
}

fn get_possible_actions(ctx: &CalcContext, hist: &History) -> Vec<(Action, Action)> {
    let mut res = vec![];

    let waiting_a = try_waiting(&hist.actions_a);
    let waiting_b = try_waiting(&hist.actions_b);

    if waiting_a.is_some() && waiting_b.is_some() {
        res.push((waiting_a.unwrap(), waiting_b.unwrap()));

        return res;
    } else if waiting_a.is_some() && waiting_b.is_none() {
        if let Some(open_b) = try_open(&hist.pos_b, &ctx.rates, hist) {
            res.push((waiting_a.unwrap(), open_b));
        }

        for goto_b in get_gotos(&hist.pos_b, &ctx.map, &hist.actions_b) {
            res.push((waiting_a.unwrap(), goto_b));
        }

        return res;
    } else if waiting_a.is_none() && waiting_b.is_some() {
        if let Some(open_a) = try_open(&hist.pos_a, &ctx.rates, hist) {
            res.push((open_a, waiting_b.unwrap()));
        }

        for goto_a in get_gotos(&hist.pos_a, &ctx.map, &hist.actions_a) {
            res.push((goto_a, waiting_b.unwrap()));
        }

        return res;
    }

    // else is waiting_a none && is waiting_b none
    if hist.pos_a == hist.pos_b {
        let mut actions_a = get_gotos(&hist.pos_a, &ctx.map, &hist.actions_a);
        if let Some(open_a) = try_open(&hist.pos_a, &ctx.rates, hist) {
            actions_a.push(open_a);
        }
        let actions_b = get_gotos(&hist.pos_b, &ctx.map, &hist.actions_b);

        return actions_a
            .into_iter()
            .cartesian_product(actions_b.into_iter())
            .collect::<Vec<(Action, Action)>>();
    } else {
        let mut actions_a = get_gotos(&hist.pos_a, &ctx.map, &hist.actions_a);
        if let Some(open_a) = try_open(&hist.pos_a, &ctx.rates, hist) {
            actions_a.push(open_a);
        }

        let mut actions_b = get_gotos(&hist.pos_b, &ctx.map, &hist.actions_b);
        if let Some(open_b) = try_open(&hist.pos_b, &ctx.rates, hist) {
            actions_b.push(open_b);
        }

        return actions_a
            .into_iter()
            .cartesian_product(actions_b.into_iter())
            .collect::<Vec<(Action, Action)>>();
    }
}

fn calc_optimal_route_impl(ctx: &CalcContext, last_history: &History) -> History {
    let mut hist = last_history.clone();
    hist.current_time += 1;

    // when all valves are open or we are out of time
    if hist.opened_at_min.len() == ctx.map.0.len() || hist.current_time >= ctx.max_time {
        hist.update_current_flow(&ctx.rates, ctx.max_time);
        return hist;
    }

    let mut res = vec![];
    for action in get_possible_actions(ctx, &hist) {
        let mut tmp_hist = hist.clone();
        tmp_hist.actions_a.push(action.0);
        tmp_hist.actions_b.push(action.1);

        match action {
            (Action::Open(v_a), Action::Open(v_b)) => {
                tmp_hist.opened_at_min.insert(v_a, hist.current_time);
                tmp_hist.opened_at_min.insert(v_b, hist.current_time);
            }
            (Action::Open(v_a), Action::GoTo(v_b, _)) => {
                tmp_hist.opened_at_min.insert(v_a, hist.current_time);
                tmp_hist.pos_b = v_b;
            }
            (Action::GoTo(v_a, _), Action::Open(v_b)) => {
                tmp_hist.opened_at_min.insert(v_b, hist.current_time);
                tmp_hist.pos_a = v_a;
            }
            (Action::GoTo(v_a, _), Action::GoTo(v_b, _)) => {
                tmp_hist.pos_a = v_a;
                tmp_hist.pos_b = v_b;
            }
        }

        res.push(calc_optimal_route_impl(ctx, &tmp_hist));
    }

    res.into_iter()
        .max_by(|ha, hb| ha.current_flow.cmp(&hb.current_flow))
        .unwrap()
}

pub fn solve_b(loader: &DataLoader) -> Result<String, &str> {
    let rates = ValveRates::from(loader);
    let map = CaveMap::from(loader).shrinked_map(&rates).removed_loopes();

    let ctx = CalcContext {
        map,
        rates,
        max_time: 13,
    };
    println!("map={:?}\n", ctx.map);
    println!("rates={:?}\n", ctx.rates);
    let rates_count = ctx.rates.0.iter().filter(|(v, c)| c > &&0).count();
    let rates_x = ctx.rates.0.iter().filter_map(|(v, c)| if c > &&0 {Some(c.clone())} else {None}).collect::<Vec<u64>>();
    println!("rates_count={}", rates_count);
    println!("rates_x={:?}", rates_x);

    let history = calc_optimal_route(&ctx);
    println!("History={:?}\n", history);

    Ok(history.current_flow.to_string())
}
