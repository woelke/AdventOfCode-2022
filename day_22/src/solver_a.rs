use aoc_helpers::data_loader::DataLoader;
use itertools::unfold;
use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy)]
enum Orient {
    Right = 0,
    Down = 1,
    Left = 2,
    Top = 3,
}

impl Orient {
    fn try_from(c: char) -> Option<Orient> {
        match c {
            'L' => Some(Orient::Left),
            'R' => Some(Orient::Right),
            _ => None,
        }
    }

    fn opposite(&self) -> Orient {
        match self {
            Orient::Top => Orient::Down,
            Orient::Down => Orient::Top,
            Orient::Left => Orient::Right,
            Orient::Right => Orient::Left,
        }
    }

    fn rotate(&self, orient: Orient) -> Orient {
        match orient {
            Orient::Right => match self {
                Orient::Top => Orient::Right,
                Orient::Right => Orient::Down,
                Orient::Down => Orient::Left,
                Orient::Left => Orient::Top,
            },
            Orient::Left => self
                .rotate(Orient::Right)
                .rotate(Orient::Right)
                .rotate(Orient::Right),
            _ => panic!("rotation to top or down not possible"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Elem {
    Wall,
    Plain,
}

impl Elem {
    fn try_from(c: char) -> Option<Elem> {
        match c {
            '.' => Some(Elem::Plain),
            '#' => Some(Elem::Wall),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Pos {
    y: i64,
    x: i64,
}

impl Pos {
    fn next(&self, orient: Orient) -> Pos {
        let mut res = *self;
        match orient {
            Orient::Right => res.x += 1,
            Orient::Down => res.y += 1,
            Orient::Left => res.x -= 1,
            Orient::Top => res.y -= 1,
        }
        res
    }
}

#[derive(Debug)]
struct World {
    map: HashMap<Pos, Elem>,
    pos: Pos,
    orient: Orient,
}

impl World {
    fn from_loader(loader: &DataLoader) -> World {
        let mut map = HashMap::new();
        loader
            .iter()
            .take_while(|line| !line.is_empty())
            .enumerate()
            .for_each(|(row, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(|(col, c)| {
                        if let Some(elem) = Elem::try_from(c) {
                            Some((col as i64, elem))
                        } else {
                            None
                        }
                    })
                    .for_each(|(col, elem)| {
                        map.insert(
                            Pos {
                                x: col,
                                y: row as i64,
                            },
                            elem,
                        );
                    })
            });
        let pos = map.iter().map(|(pos, _)| *pos).min().unwrap();

        World {
            map,
            pos,
            orient: Orient::Right,
        }
    }

    fn go(&mut self, steps: usize) {
        if steps == 0 {
            return;
        }

        let next_pos = self.pos.next(self.orient);
        match self.map.get(&next_pos) {
            None => {
                let wrap_pos = self.next_pos_after_wrap_around();
                match self.map.get(&wrap_pos) {
                    None => panic!("should never happen"),
                    Some(Elem::Wall) => return,
                    Some(Elem::Plain) => self.pos = wrap_pos,
                }
            }
            Some(Elem::Wall) => return,
            Some(Elem::Plain) => self.pos = next_pos,
        }

        self.go(steps - 1);
    }

    fn next_pos_after_wrap_around(&self) -> Pos {
        unfold(self.pos, |p| {
            *p = p.next(self.orient.opposite());
            Some(*p)
        })
        .take_while(|p| self.map.get(p).is_some())
        .last()
        .unwrap()
    }

    fn rotate(&mut self, direction: Orient) {
        self.orient = self.orient.rotate(direction);
    }
}

#[derive(Debug)]
enum Instr {
    Steps(usize),
    Rotate(Orient),
}

type Instrs = VecDeque<Instr>;

fn get_instrs(loader: &DataLoader) -> Instrs {
    let mut res = VecDeque::new();
    let mut data = vec![];

    for c in loader.iter().last().unwrap().chars() {
        if c.is_digit(10) {
            data.push(c);
        } else {
            res.push_back(Instr::Steps(
                String::from_iter(data.iter()).parse::<usize>().unwrap(),
            ));
            data.clear();

            res.push_back(Instr::Rotate(Orient::try_from(c).unwrap()));
        }
    }

    res.push_back(Instr::Steps(
        String::from_iter(data.iter()).parse::<usize>().unwrap(),
    ));

    res
}

pub fn solve_a(loader: &DataLoader) -> Result<String, &str> {
    let mut world = World::from_loader(loader);
    let instrs = get_instrs(loader);

    for (i, instr) in instrs.iter().enumerate() {
        match instr {
            Instr::Steps(steps) => world.go(*steps),
            Instr::Rotate(to) => world.rotate(*to),
        }
    }

    let res = (world.pos.y + 1) * 1000 + (world.pos.x + 1) * 4 + world.orient as i64;
    Ok(res.to_string())
}
