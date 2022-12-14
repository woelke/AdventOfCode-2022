pub struct Range<T> {
    buf: Vec<T>,
}

impl<T> Iterator for Range<T>
where
    T: Clone,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.buf.pop()
    }
}

pub trait RangeIterator<T>: Iterator<Item = T> + Sized
where
    T: Ord + Clone,
{
    fn max_x(&mut self, count: usize) -> Range<T> {
        let mut buf: Vec<T> = Vec::new();
        while let Some(val) = self.next() {
            buf.push(val);
            buf.sort();
            if buf.len() > count {
                buf.remove(0);
            }
        }

        Range { buf }
    }
}

impl<T, I: Iterator<Item = T>> RangeIterator<T> for I
where
    I: Iterator<Item = T>,
    T: Ord + Clone,
{
}

#[cfg(test)]
mod max_x_tests {
    use super::*;

    #[test]
    fn test_max_x() {
        assert_eq!((1..=10).max_x(3).collect::<Vec<i32>>(), vec![10, 9, 8]);
        assert_eq!((1..=2).max_x(2).collect::<Vec<i32>>(), vec![2, 1]);
        assert_eq!((1..=2).max_x(3).collect::<Vec<i32>>(), vec![2, 1]);
        assert_eq!((1..=2).max_x(3).collect::<Vec<i32>>(), vec![2, 1]);
        assert_eq!(
            vec![1, 3, 5, 67, 4, 2, 23]
                .iter()
                .max_x(3)
                .collect::<Vec<&i32>>(),
            vec![&67, &23, &5]
        );
        assert_eq!(
            vec![3, 3, 3].iter().max_x(2).collect::<Vec<&i32>>(),
            vec![&3, &3]
        );
    }
}
