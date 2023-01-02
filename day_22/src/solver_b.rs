use aoc_helpers::data_loader::DataLoader;
use aoc_helpers::matrix_helper::print_matrix;
use simple_matrix::Matrix;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::iter;

#[derive(Debug, Clone, Copy)]
enum Orient {
    Right,
    Down,
    Left,
    Top,
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

    fn to_char(&self) -> char {
        match self {
            Orient::Top => '^',
            Orient::Right => '>',
            Orient::Left => '<',
            Orient::Down => 'v',
        }
    }
}

type Edge = Orient;

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

    fn to_tuple(&self) -> (i64, i64) {
        (self.x, self.y)
    }
}

type Quadrant = Pos;
type RelativePos = Pos;

#[derive(Debug)]
struct World {
    map: HashMap<Pos, Elem>,
    pos: Pos,
    orient: Orient,
    quadrant_size: i64,
}

impl World {
    fn from_loader(loader: &DataLoader, quadrant_size: i64) -> World {
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
            quadrant_size,
        }
    }

    fn go(&mut self, steps: usize) {
        if steps == 0 {
            return;
        }

        let next_pos = self.pos.next(self.orient);
        match self.map.get(&next_pos) {
            None => {
                self.print_map();
                let (wrap_pos, wrap_orient) = self.next_pos_after_wrap_around();
                match self.map.get(&wrap_pos) {
                    None => panic!("should never happen"),
                    Some(Elem::Wall) => {
                        println!("----Wall----");
                        return;
                    }
                    Some(Elem::Plain) => {
                        self.pos = wrap_pos;
                        self.orient = wrap_orient;
                        self.print_map();
                    }
                }
            }
            Some(Elem::Wall) => return,
            Some(Elem::Plain) => self.pos = next_pos,
        }

        self.go(steps - 1);
    }

    fn rotate(&mut self, direction: Orient) {
        self.orient = self.orient.rotate(direction);
    }

    fn next_pos_after_wrap_around(&self) -> (Pos, Orient) {
        let (quad, rel_pos) = self.get_current_quadrant_and_relative_pos();
        let edge = self.get_current_quadrant_edge(rel_pos).unwrap();

        //match (quad.to_tuple(), edge) {
        //((2, 1), Edge::Right) => (
        //Pos {
        //x: (4 * self.quadrant_size) - (rel_pos.y + 1),
        //y: (2 * self.quadrant_size),
        //},
        //Orient::Down,
        //),
        //((2, 2), Edge::Down) => (
        //Pos {
        //x: (1 * self.quadrant_size) - (rel_pos.x + 1),
        //y: (2 * self.quadrant_size) - 1,
        //},
        //Orient::Top,
        //),
        //((1, 1), Edge::Top) => (
        //Pos {
        //x: (2 * self.quadrant_size),
        //y: rel_pos.x,
        //},
        //Orient::Right,
        //),
        //_ => {
        //println!("quad={:?}; rel_pos={:?}; edge={:?}", quad, rel_pos, edge);
        //unimplemented!()
        //}
        //}

        println!("quad={:?}; rel_pos={:?}; edge={:?}", quad, rel_pos, edge);
        match (quad.to_tuple(), edge) {
            ((1, 0), Edge::Top) => (
                Pos {
                    x: 0,
                    y: (3 * self.quadrant_size) + rel_pos.x,
                },
                Orient::Right,
            ),
            ((0, 2), Edge::Left) => (
                Pos {
                    x: (1 * self.quadrant_size),
                    y: (1 * self.quadrant_size) - (rel_pos.y + 1),
                },
                Orient::Right,
            ),
            ////
            ((0, 2), Edge::Top) => (
                Pos {
                    x: (1 * self.quadrant_size),
                    y: (1 * self.quadrant_size) + rel_pos.x,
                },
                Orient::Right,
            ),
            ((0, 3), Edge::Left) => (
                Pos {
                    x: (1 * self.quadrant_size) + rel_pos.y,
                    y: 0,
                },
                Orient::Down,
            ),
            ((1, 2), Edge::Down) => (
                Pos {
                    x: (1 * self.quadrant_size) - 1,
                    y: (3 * self.quadrant_size) + rel_pos.x,
                },
                Orient::Left,
            ),
            ((0, 3), Edge::Right) => (
                Pos {
                    x: (1 * self.quadrant_size) + rel_pos.y,
                    y: (3 * self.quadrant_size) - 1,
                },
                Orient::Top,
            ),
            ((0, 3), Edge::Down) => (
                Pos {
                    x: (2 * self.quadrant_size) + rel_pos.x,
                    y: 0,
                },
                Orient::Down,
            ),
            ((2, 0), Edge::Top) => (
                Pos {
                    x: rel_pos.x,
                    y: (4 * self.quadrant_size) - 1,
                },
                Orient::Top,
            ),
            ((1, 2), Edge::Right) => (
                Pos {
                    x: (3 * self.quadrant_size) - 1,
                    y: (1 * self.quadrant_size) - (rel_pos.y + 1),
                },
                Orient::Left,
            ),
            ((2, 0), Edge::Right) => (
                Pos {
                    x: (2 * self.quadrant_size) - 1,
                    y: (3 * self.quadrant_size) - (rel_pos.y + 1),
                },
                Orient::Left,
            ),
            ((2, 0), Edge::Down) => (
                Pos {
                    x: (2 * self.quadrant_size) - 1,
                    y: (1 * self.quadrant_size) + rel_pos.x,
                },
                Orient::Left,
            ),
            ((1, 1), Edge::Right) => (
                Pos {
                    x: (2 * self.quadrant_size) + rel_pos.y,
                    y: (1 * self.quadrant_size) - 1,
                },
                Orient::Top,
            ),
            ((1, 1), Edge::Left) => (
                Pos {
                    x: rel_pos.y,
                    y: (2 * self.quadrant_size),
                },
                Orient::Down,
            ),
            ((1, 0), Edge::Left) => (
                Pos {
                    x: 0,
                    y: (3 * self.quadrant_size) - (rel_pos.y + 1),
                },
                Orient::Right,
            ),
            _ => {
                //println!("quad={:?}; rel_pos={:?}; edge={:?}", quad, rel_pos, edge);
                unimplemented!()
            }
        }
    }

    fn get_current_quadrant_and_relative_pos(&self) -> (Quadrant, RelativePos) {
        let quad = Quadrant {
            y: self.pos.y / self.quadrant_size as i64,
            x: self.pos.x / self.quadrant_size as i64,
        };
        let rel_pos = RelativePos {
            y: self.pos.y % self.quadrant_size as i64,
            x: self.pos.x % self.quadrant_size as i64,
        };
        (quad, rel_pos)
    }

    fn get_current_quadrant_edge(&self, pos: RelativePos) -> Option<Edge> {
        match self.orient {
            Orient::Top if pos.y == 0 => Some(self.orient),
            Orient::Right if pos.x == self.quadrant_size - 1 => Some(self.orient),
            Orient::Down if pos.y == self.quadrant_size - 1 => Some(self.orient),
            Orient::Left if pos.x == 0 => Some(self.orient),
            _ => None,
        }
    }

    fn print_map(&self) {
        let x_max = self.map.keys().map(|pos| pos.x).max().unwrap() as usize;
        let y_max = self.map.keys().map(|pos| pos.y).max().unwrap() as usize;
        let mut m = Matrix::from_iter(y_max + 1, x_max + 1, iter::repeat(' '));
        for (pos, elem) in self.map.iter() {
            match elem {
                Elem::Wall => m.set(pos.y as usize, pos.x as usize, '#'),
                Elem::Plain => m.set(pos.y as usize, pos.x as usize, '.'),
            };
        }
        m.set(
            self.pos.y as usize,
            self.pos.x as usize,
            self.orient.to_char(),
        );

        print_matrix(&m);
        println!();
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

pub fn solve_b(loader: &DataLoader, quadrant_size: i64) -> Result<String, &str> {
    let mut world = World::from_loader(loader, quadrant_size);
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
