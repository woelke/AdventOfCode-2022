use aoc_helpers::data_loader::DataLoader;
use cond_utils::Between;
use std::collections::{HashSet, VecDeque};
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
    S,
    E,
    W,
}

impl Direct {
    fn pos(&self) -> Pos {
        match self {
            Direct::N => Pos { x: 0, y: -1 },
            Direct::S => Pos { x: 0, y: 1 },
            Direct::E => Pos { x: 1, y: 0 },
            Direct::W => Pos { x: -1, y: 0 },
        }
    }

    fn opposite(&self) -> Self {
        match self {
            Direct::N => Direct::S,
            Direct::S => Direct::N,
            Direct::W => Direct::E,
            Direct::E => Direct::W,
        }
    }
}

fn get_all_poses() -> [Pos; 4] {
    [
        Direct::N.pos(),
        Direct::E.pos(),
        Direct::S.pos(),
        Direct::W.pos(),
    ]
}

#[derive(Debug, Clone)]
struct Grove {
    start: Pos,
    start_direction: Direct,
    goal: Pos,
    goal_direction: Direct,
    edge: Pos,
    h_blizz: Vec<Vec<(usize, Direct)>>,
    v_blizz: Vec<Vec<(usize, Direct)>>,
}

impl Grove {
    fn from_loader(loader: &DataLoader) -> Grove {
        let start: Pos = Pos { x: 1, y: 0 };
        let start_direction = Direct::S;
        let mut goal: Pos = Pos { x: -1, y: -1 };
        let goal_direction = Direct::S;
        let mut h_blizz: Vec<Vec<(usize, Direct)>> = vec![];
        let mut v_blizz: Vec<Vec<(usize, Direct)>> = vec![];

        h_blizz.push(vec![]);
        for (y, line) in loader.iter().enumerate().skip(1) {
            h_blizz.push(vec![]);

            if 3 == line.chars().filter(|c| c == &'#').take(3).count() {
                let x = line
                    .chars()
                    .enumerate()
                    .find_map(|(x, c)| if c == '.' { Some(x) } else { None })
                    .unwrap();
                goal = Pos {
                    x: x as i64,
                    y: y as i64,
                };
                break;
            }

            for (x, c) in line.chars().enumerate() {
                if v_blizz.len() <= x {
                    v_blizz.push(vec![]);
                }

                match c {
                    '>' => h_blizz[y].push((x, Direct::E)),
                    '<' => h_blizz[y].push((x, Direct::W)),
                    '^' => v_blizz[x].push((y, Direct::N)),
                    'v' => v_blizz[x].push((y, Direct::S)),
                    '#' | '.' => (),
                    _ => panic!("unkown char"),
                }
            }
        }

        Grove {
            start,
            start_direction,
            goal,
            goal_direction,
            edge: goal,
            h_blizz,
            v_blizz,
        }
    }

    fn is_walkable(&self, pos: Pos, round: usize) -> bool {
        for (x, direct) in self.h_blizz[pos.y as usize].iter() {
            let bliz_pos = match direct {
                Direct::E => 1 + (x - 1 + round as usize) % (self.edge.x as usize),
                Direct::W => 1 + (*x as i64 - 1 - round as i64).rem_euclid(self.edge.x) as usize,
                _ => {
                    println!("the impl derect={direct:?}");
                    panic!("impossile direction")
                }
            };
            if bliz_pos == (pos.x as usize) {
                return false;
            }
        }

        for (y, direct) in self.v_blizz[pos.x as usize].iter() {
            let bliz_pos = match direct {
                Direct::S => 1 + (y - 1 + round as usize) % (self.edge.y as usize - 1),
                Direct::N => {
                    1 + (*y as i64 - 1 - round as i64).rem_euclid(self.edge.y - 1) as usize
                }
                _ => {
                    println!("the impl derect={direct:?}");
                    panic!("impossile direction")
                }
            };
            if bliz_pos == (pos.y as usize) {
                return false;
            }
        }

        true
    }

    fn next_poses(&self, from_pos: Pos, round: usize) -> Vec<Pos> {
        if from_pos == self.start {
            let next_pos = self.start + self.start_direction.pos();
            if self.is_walkable(next_pos, round) {
                return vec![from_pos, next_pos];
            } else {
                return vec![from_pos];
            }
        }

        if from_pos == (self.goal + self.goal_direction.opposite().pos()) {
            return vec![self.goal];
        }

        let mut res = get_all_poses()
            .iter()
            .map(|p| *p + from_pos)
            .filter(|p| p.x.within(1, self.edge.x) && p.y.within(1, self.edge.y - 1))
            .filter(|p| self.is_walkable(*p, round))
            .collect::<Vec<Pos>>();

        if self.is_walkable(from_pos, round) {
            res.push(from_pos);
        }
        res
    }

    fn print(&self) {
        todo!();
    }
}

fn find_shortest_path(grove: &Grove, start_step: usize, max_rounds: usize) -> Option<usize> {
    let mut jobs: VecDeque<(Pos, usize)> = VecDeque::new();
    for next_pos in grove.next_poses(grove.start, start_step + 1) {
        jobs.push_front((next_pos, start_step + 1));
    }
    let mut processed: HashSet<(Pos, usize)> = HashSet::new();

    let mut best_res = None;

    while let Some((pos, steps)) = jobs.pop_front() {
        if pos == grove.goal && (best_res.is_none() || steps < best_res.unwrap()) {
            println!("found res steps={steps}");
            best_res = Some(steps);
            continue;
        }

        if steps - start_step >= max_rounds || (best_res.is_some() && steps >= best_res.unwrap()) {
            continue;
        }

        for next_pos in grove.next_poses(pos, steps + 1) {
            let job = (next_pos, steps + 1);
            if !processed.contains(&job) {
                jobs.push_front(job);
                processed.insert(job);
            }
        }
    }

    best_res
}

pub fn solve_a(loader: &DataLoader) -> Result<String, &str> {
    let grove = Grove::from_loader(loader);
    println!("Grove={grove:?}");
    let max_rounds = (grove.goal.x + grove.goal.y) * 2;
    let steps = find_shortest_path(&grove, 0, max_rounds as usize).ok_or("no path found")?;
    Ok(steps.to_string())
}

pub fn solve_b(loader: &DataLoader) -> Result<String, &str> {
    let grove = Grove::from_loader(loader);
    println!("grove={grove:?}");

    let mut inverted_grove = grove.clone();
    inverted_grove.start = grove.goal;
    inverted_grove.goal = grove.start;
    inverted_grove.start_direction = grove.start_direction.opposite();
    inverted_grove.goal_direction = grove.goal_direction.opposite();

    println!("inverted_grove={inverted_grove:?}");

    let max_rounds = (grove.goal.x + grove.goal.y) * 3;
    let to_steps_1 =
        find_shortest_path(&grove, 0, max_rounds as usize).ok_or("to_steps_1 not found")?;
    let back_steps = find_shortest_path(&inverted_grove, to_steps_1, max_rounds as usize)
        .ok_or("back_steps not found")?;
    let to_steps_2 = find_shortest_path(&grove, back_steps, max_rounds as usize)
        .ok_or("to_steps_2 not found")?;

    Ok(to_steps_2.to_string())
}

#[cfg(test)]
mod solver_test {
    use super::*;

    #[test]
    fn test_h_blizz_direct_east() {
        let h_blizz = vec![vec![], vec![(1, Direct::E)]];
        let grove = Grove {
            start: Pos { x: 0, y: 0 },
            start_direction: Direct::N,
            goal: Pos { x: 5, y: 6 },
            goal_direction: Direct::N,
            edge: Pos { x: 5, y: 6 },
            v_blizz: vec![vec![], vec![], vec![], vec![], vec![], vec![], vec![]],
            h_blizz,
        };
        assert!(!grove.is_walkable(Pos { x: 1, y: 1 }, 0));
        assert!(grove.is_walkable(Pos { x: 1, y: 1 }, 1));
        assert!(grove.is_walkable(Pos { x: 1, y: 1 }, 2));

        assert!(!grove.is_walkable(Pos { x: 2, y: 1 }, 1));
        assert!(!grove.is_walkable(Pos { x: 5, y: 1 }, 4));
        assert!(!grove.is_walkable(Pos { x: 1, y: 1 }, 5));
        assert!(!grove.is_walkable(Pos { x: 2, y: 1 }, 6));

        assert!(!grove.is_walkable(Pos { x: 3, y: 1 }, 7));
        assert!(grove.is_walkable(Pos { x: 4, y: 1 }, 7));
    }

    #[test]
    fn test_h_blizz_direct_west() {
        let h_blizz = vec![vec![], vec![(1, Direct::W)]];
        let grove = Grove {
            start: Pos { x: 0, y: 0 },
            start_direction: Direct::N,
            goal: Pos { x: 5, y: 6 },
            goal_direction: Direct::N,
            edge: Pos { x: 5, y: 6 },
            v_blizz: vec![vec![], vec![], vec![], vec![], vec![], vec![], vec![]],
            h_blizz,
        };
        assert!(!grove.is_walkable(Pos { x: 1, y: 1 }, 0));
        assert!(grove.is_walkable(Pos { x: 1, y: 1 }, 1));

        assert!(!grove.is_walkable(Pos { x: 5, y: 1 }, 1));
        assert!(!grove.is_walkable(Pos { x: 4, y: 1 }, 2));
    }

    #[test]
    fn test_v_blizz_direct_south() {
        let v_blizz = vec![vec![], vec![(1, Direct::S)]];
        let grove = Grove {
            start: Pos { x: 0, y: 0 },
            start_direction: Direct::N,
            goal: Pos { x: 5, y: 6 },
            goal_direction: Direct::N,
            edge: Pos { x: 5, y: 6 },
            h_blizz: vec![vec![], vec![], vec![], vec![], vec![], vec![], vec![]],
            v_blizz,
        };
        assert!(!grove.is_walkable(Pos { x: 1, y: 1 }, 0));
        assert!(grove.is_walkable(Pos { x: 1, y: 1 }, 1));

        assert!(!grove.is_walkable(Pos { x: 1, y: 2 }, 1));
        assert!(!grove.is_walkable(Pos { x: 1, y: 5 }, 4));
        assert!(!grove.is_walkable(Pos { x: 1, y: 1 }, 5));
    }

    #[test]
    fn test_v_blizz_direct_north() {
        let v_blizz = vec![vec![], vec![(1, Direct::N)]];
        let grove = Grove {
            start: Pos { x: 0, y: 0 },
            start_direction: Direct::N,
            goal: Pos { x: 5, y: 6 },
            goal_direction: Direct::N,
            edge: Pos { x: 5, y: 6 },
            h_blizz: vec![vec![], vec![], vec![], vec![], vec![], vec![], vec![]],
            v_blizz,
        };
        assert!(!grove.is_walkable(Pos { x: 1, y: 1 }, 0));
        assert!(grove.is_walkable(Pos { x: 1, y: 1 }, 1));

        assert!(!grove.is_walkable(Pos { x: 1, y: 5 }, 1));
    }
}
