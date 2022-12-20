use aoc_helpers::data_loader::DataLoader;

type Val = i64;
type Idx = usize;
type OriginIdx = usize;

#[derive(Debug, Clone)]
struct CircBuf {
    buf: Vec<(Val, OriginIdx)>,
}

impl CircBuf {
    fn new(values: Vec<Val>) -> CircBuf {
        CircBuf {
            buf: Vec::from_iter(values.into_iter().enumerate().map(|(i, v)| (v, i))),
        }
    }

    fn from_loader(loader: &DataLoader) -> CircBuf {
        CircBuf::new(Vec::from_iter(
            loader.iter().map(|line| line.parse::<Val>().unwrap()),
        ))
    }

    fn mix_idx(&mut self, origin_idx: OriginIdx) {
        let idx = self.get_idx(origin_idx);
        self.move_elem(idx, self.buf[idx].0);
    }

    fn mixed_buf(&self) -> CircBuf {
        let mut res = self.clone();

        for origin_idx in 0..self.buf.len() {
            res.mix_idx(origin_idx);
        }

        res
    }

    fn move_elem(&mut self, idx: Idx, by: Val) {
        let circ = (self.buf.len() * 2) as i64;

        if by > 0 {
            self.bubble_elem_forward(idx, by % circ);
        } else if by < 0 {
            self.buf.reverse();
            self.bubble_elem_forward(self.buf.len() - idx - 1, by.abs() % circ);
            self.buf.reverse();
        }
    }

    fn bubble_elem_forward(&mut self, idx: Idx, by: Val) {
        if by == 0 {
            return;
        }

        let c_idx = idx % self.buf.len();
        let cnext_idx = (idx + 1) % self.buf.len();

        self.buf.swap(c_idx, cnext_idx);
        self.bubble_elem_forward(idx + 1, by - 1);
    }

    fn get_idx(&self, origin_idx: OriginIdx) -> Idx {
        self.buf.iter().position(|(_, i)| i == &origin_idx).unwrap()
    }

    fn view(&self) -> Vec<i64> {
        self.buf
            .iter()
            .map(|(v, _)| v)
            .cloned()
            .collect::<Vec<i64>>()
    }

    fn nth_val_after_0(&self, nth: usize) -> Val {
        let idx = self.buf.iter().position(|(v, _)| v == &0).unwrap();
        self.buf[(idx + nth) % self.buf.len()].0
    }
}

pub fn solve_a(loader: &DataLoader) -> Result<String, &str> {
    let buf = CircBuf::from_loader(loader);
    let mixed_buf = buf.mixed_buf();

    let mut res = 0;
    for nth in [1000, 2000, 3000] {
        res += mixed_buf.nth_val_after_0(nth);
    }

    Ok(res.to_string())
}

pub fn solve_b(loader: &DataLoader) -> Result<String, &str> {
    let input_buf = CircBuf::from_loader(loader);
    let salted_buf = CircBuf::new(
        input_buf
            .view()
            .into_iter()
            .map(|v| v * 811589153)
            .collect::<Vec<i64>>(),
    );

    let mut buf = salted_buf;
    for i in 1..=10 {
        let tmp_buf = buf.mixed_buf();
        buf = tmp_buf;
    }

    let mut res = 0;
    for nth in [1000, 2000, 3000] {
        let tmp = buf.nth_val_after_0(nth);
        res += tmp
    }
    Ok(res.to_string())
}
