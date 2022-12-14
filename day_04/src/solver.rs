use aoc_helpers::data_loader::DataLoader;

#[derive(Debug)]
struct Range(i32, i32);

impl Range {
    fn includes(&self, other: &Range) -> bool {
        self.0 <= other.0 && self.1 >= other.1
    }

    pub(self) fn in_between(&self, x: i32) -> bool {
        self.0 <= x && x <= self.1
    }
}

impl TryFrom<&str> for Range {
    type Error = &'static str;

    fn try_from(x: &str) -> Result<Self, Self::Error> {
        let (l_str, r_str) = x.split_once('-').ok_or("not a range")?;
        let l_int = l_str.parse::<i32>().map_err(|_| "left is not an int")?;
        let r_int = r_str.parse::<i32>().map_err(|_| "right is not an int")?;
        Ok(Range(l_int, r_int))
    }
}

struct RangePair(Range, Range);

impl TryFrom<&str> for RangePair {
    type Error = &'static str;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let (l_str, r_str) = line.split_once(',').ok_or("not a pair of ranges")?;
        let l = Range::try_from(l_str)?;
        let r = Range::try_from(r_str)?;
        Ok(RangePair(l, r))
    }
}

impl RangePair {
    fn fully_overlayed(&self) -> bool {
        self.0.includes(&self.1) || self.1.includes(&self.0)
    }

    fn partially_overlayed(&self) -> bool {
        self.fully_overlayed() || self.0.in_between(self.1 .0) || self.0.in_between(self.1 .1)
    }
}

trait RangePairs {
    fn range_pairs(&self) -> Result<Vec<RangePair>, &str>;
}

impl RangePairs for DataLoader {
    fn range_pairs(&self) -> Result<Vec<RangePair>, &str> {
        self.iter()
            .map(|line| RangePair::try_from(line.as_str()))
            .collect()
    }
}

pub fn solve_a(loader: &DataLoader) -> Result<String, &str> {
    let pairs = loader.range_pairs()?;

    let res = pairs
        .iter()
        .filter(|pair| pair.fully_overlayed())
        .count()
        .to_string();

    Ok(res)
}

pub fn solve_b(loader: &DataLoader) -> Result<String, &str> {
    let pairs = loader.range_pairs()?;

    let res = pairs
        .iter()
        .filter(|pair| pair.partially_overlayed())
        .count()
        .to_string();

    Ok(res)
}

#[cfg(test)]
mod test_solver {
    use super::*;

    #[test]
    fn test_overlayed() {
        assert!(!RangePair::try_from("2-4,6-8")
            .unwrap()
            .partially_overlayed());
        assert!(!RangePair::try_from("6-8,2-4")
            .unwrap()
            .partially_overlayed());
        assert!(!RangePair::try_from("2-3,4-5")
            .unwrap()
            .partially_overlayed());
        assert!(RangePair::try_from("5-7,7-9")
            .unwrap()
            .partially_overlayed());
        assert!(RangePair::try_from("7-9,5-7")
            .unwrap()
            .partially_overlayed());
        assert!(RangePair::try_from("2-8,3-7")
            .unwrap()
            .partially_overlayed());
        assert!(RangePair::try_from("3-7,2-8")
            .unwrap()
            .partially_overlayed());
        assert!(RangePair::try_from("6-6,4-6")
            .unwrap()
            .partially_overlayed());
        assert!(RangePair::try_from("4-6,6-6")
            .unwrap()
            .partially_overlayed());
        assert!(RangePair::try_from("2-6,4-8")
            .unwrap()
            .partially_overlayed());
    }
}
