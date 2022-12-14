use aoc_helpers::data_loader::DataLoader;

#[derive(Debug)]
struct Stacks(Vec<Vec<char>>);

impl TryFrom<&DataLoader> for Stacks {
    type Error = &'static str;

    fn try_from(loader: &DataLoader) -> Result<Self, Self::Error> {
        let lines = loader
            .iter()
            .take_while(|l| !l.starts_with(" 1"))
            .map(|l| {
                l.chars()
                    .skip(1)
                    .enumerate()
                    .filter(|(i, c)| i % 4 == 0)
                    .map(|(i, c)| c)
                    .enumerate()
                    .collect::<Vec<(usize, char)>>()
            })
            .collect::<Vec<Vec<(usize, char)>>>();

        let mut res: Vec<Vec<char>> = vec![vec![]; lines[0].len()];

        for (i, c) in lines.into_iter().flatten() {
            if c != ' ' {
                res[i].insert(0, c);
            }
        }

        Ok(Stacks(res))
    }
}

impl Stacks {
    fn top_rank(&self) -> Vec<char> {
        self.0
            .iter()
            .filter(|v| !v.is_empty())
            .map(|v| v.last().unwrap().clone())
            .collect::<Vec<char>>()
    }
}

#[derive(Debug)]
struct Ops(Vec<(usize, usize, usize)>);

impl TryFrom<&DataLoader> for Ops {
    type Error = &'static str;

    fn try_from(loader: &DataLoader) -> Result<Self, Self::Error> {
        let raw_ops = loader
            .iter()
            .filter(|line| line.starts_with('m'))
            .map(|line| {
                line.split(' ')
                    .enumerate()
                    .filter(|(i, _)| i % 2 == 1)
                    .map(|(_, c)| c.parse::<usize>().ok())
                    .collect::<Option<Vec<usize>>>()
            })
            .collect::<Option<Vec<Vec<usize>>>>()
            .ok_or("failed to parse Ops")?;

        if raw_ops.iter().any(|v| v.len() != 3) {
            return Err("incomplete op");
        }

        Ok(Ops(raw_ops
            .into_iter()
            .map(|l| (l[0], l[1], l[2]))
            .collect()))
    }
}

fn exec_operation_crate_mover_9000((count, from, to): (usize, usize, usize), stacks: &mut Stacks) {
    if from > stacks.0.len() || to > stacks.0.len() {
        panic!("invalid op; count={count}; from={from}; to={to}");
    }

    for i in 1..=count.clone() {
        if let Some(item) = stacks.0[from - 1].pop() {
            stacks.0[to - 1].push(item);
        } else {
            panic!("pop of an empty stack");
        }
    }
}

fn exec_operation_crate_mover_9001((count, from, to): (usize, usize, usize), stacks: &mut Stacks) {
    if from > stacks.0.len() || to > stacks.0.len() {
        panic!("invalid op; count={count}; from={from}; to={to}");
    }

    let from_stack_len = stacks.0[from - 1].len();
    let items_to_move = stacks.0[from - 1]
        .iter()
        .skip(from_stack_len - count)
        .map(|c| c.clone())
        .collect::<Vec<char>>();
    stacks.0[to - 1].extend(items_to_move);
    stacks.0[from - 1].truncate(from_stack_len - count);
}

fn exec_operations(
    ops: &Ops,
    stacks: &mut Stacks,
    fun: &dyn Fn((usize, usize, usize), &mut Stacks) -> (),
) {
    for op in ops.0.iter() {
        fun(op.clone(), stacks);
    }
}

pub fn solve_a(loader: &DataLoader) -> Result<String, &str> {
    let ops = Ops::try_from(loader)?;
    let mut stacks = Stacks::try_from(loader)?;

    exec_operations(&ops, &mut stacks, &exec_operation_crate_mover_9000);

    Ok(stacks.top_rank().iter().collect::<String>())
}

pub fn solve_b(loader: &DataLoader) -> Result<String, &str> {
    let ops = Ops::try_from(loader)?;
    let mut stacks = Stacks::try_from(loader)?;

    exec_operations(&ops, &mut stacks, &exec_operation_crate_mover_9001);

    Ok(stacks.top_rank().iter().collect::<String>())
}
