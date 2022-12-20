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
        self.move_element(idx);
    }

    fn mixed_buf(&self) -> CircBuf {
        let mut res = self.clone();

        for origin_idx in 0..self.buf.len() {
            res.mix_idx(origin_idx);
            //println!("{:?}", res.view());
        }

        res
    }

    fn move_element(&mut self, idx: Idx) {
        let item = self.buf[idx];
        let (val, _) = item;

        let new_val = if val > 0 { val + 1 } else { val };
        let mut new_idx = self.idx_add(idx, new_val);

        if new_idx == 0 && val < 0 {
            new_idx = self.buf.len();
        }

        if new_idx > idx {
            self.buf.insert(new_idx, item);
            self.buf.remove(idx);
        } else if new_idx < idx {
            self.buf.remove(idx);
            self.buf.insert(new_idx, item);
        }
    }

    fn idx_add(&self, idx: Idx, val: Val) -> Idx {
        let circ = self.buf.len() as i64;
        let res = (idx as i64 + val) % circ;
        if res >= 0 {
            res as usize
        } else {
            (circ + res) as usize
        }
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

        let new_idx = self.idx_add(idx, nth as i64);
        self.buf[new_idx].0
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
    Ok("".to_string())
}

#[cfg(test)]
mod test_solver {
    use super::*;

    #[test]
    fn test_mixed_idx() {
        let loader = DataLoader::from_file("data/test_input.txt");
        let mut buf = CircBuf::from_loader(&loader);

        assert_eq!(buf.view(), [1, 2, -3, 3, -2, 0, 4], "Initial arrangement");

        buf.mix_idx(0);
        assert_eq!(buf.view(), [2, 1, -3, 3, -2, 0, 4], "idx=0; 1 moves between 2 and -3");

        buf.mix_idx(1);
        assert_eq!(buf.view(), [1, -3, 2, 3, -2, 0, 4], "idx=1; 2 moves between -3 and 3");

        buf.mix_idx(2);
        assert_eq!(buf.view(), [1, 2, 3, -2, -3, 0, 4], "idx=2; -3 moves between -2 and 0");

        buf.mix_idx(3);
        assert_eq!(buf.view(), [1, 2, -2, -3, 0, 3, 4], "idx=3; 3 moves between 0 and 4");

        buf.mix_idx(4);
        assert_eq!(buf.view(), [1, 2, -3, 0, 3, 4, -2], "idx=4; -2 moves between 4 and 1");

        buf.mix_idx(5);
        assert_eq!(buf.view(), [1, 2, -3, 0, 3, 4, -2], "idx=5; 0 does not move");

        buf.mix_idx(6);
        assert_eq!(buf.view(), [1, 2, -3, 4, 0, 3, -2], "idx=6; 4 moves between -3 and 0");
    }
}
