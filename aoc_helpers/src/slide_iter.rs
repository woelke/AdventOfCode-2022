use std::collections::VecDeque;

pub struct Slide<I, T> {
    iter: I,
    size: usize,
    buf: VecDeque<T>,
    is_initalized: bool,
    is_drained: bool,
}

impl<I, T> Iterator for Slide<I, T>
where
    I: Iterator<Item = T>,
    T: Clone,
{
    type Item = Vec<T>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.is_drained {
            return None;
        }

        self.update_buf();

        if self.buf.is_empty() {
            return None;
        }

        Some(self.buf.iter().cloned().collect())
    }
}

impl<I, T> Slide<I, T>
where
    I: Iterator<Item = T>,
    T: Clone,
{
    pub fn new(iter: I, size: usize) -> Self {
        let mut buf: VecDeque<T> = VecDeque::new();
        buf.reserve(size);

        Slide {
            iter,
            size,
            buf,
            is_initalized: false,
            is_drained: false,
        }
    }

    fn initialize_buf(&mut self) {
        for _ in 0..self.size {
            if let Some(item) = self.iter.next() {
                self.buf.push_back(item);
            } else {
                self.is_drained = true;
                break;
            }
        }
    }

    fn update_buf(&mut self) {
        if !self.is_initalized {
            self.initialize_buf();
            self.is_initalized = true;
            return;
        }

        self.buf.pop_front();

        if let Some(item) = self.iter.next() {
            self.buf.push_back(item);
        } else {
            self.buf.clear();
            self.is_drained = true;
        }
    }
}

pub trait SlideIterator<T>: Iterator<Item = T> + Sized
where
    T: Clone,
{
    fn slide(self, size: usize) -> Slide<Self, T> {
        Slide::new(self, size)
    }
}

impl<T, I: Iterator<Item = T>> SlideIterator<T> for I
where
    I: Iterator<Item = T>,
    T: Clone,
{
}

#[cfg(test)]
mod slide_tests {
    use super::*;

    #[test]
    fn test_slider() {
        assert_eq!(
            (1..=4).slide(3).collect::<Vec<Vec<i32>>>(),
            vec![vec![1, 2, 3], vec![2, 3, 4]]
        );

        assert_eq!(
            (1..=5).slide(3).collect::<Vec<Vec<i32>>>(),
            vec![vec![1, 2, 3], vec![2, 3, 4], vec![3, 4, 5]]
        );

        assert_eq!(
            (1..=4).slide(2).collect::<Vec<Vec<i32>>>(),
            vec![vec![1, 2], vec![2, 3], vec![3, 4]]
        );

        assert_eq!(
            (1..=5).slide(2).collect::<Vec<Vec<i32>>>(),
            vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]]
        );

        assert_eq!(
            (1..=5).slide(1).collect::<Vec<Vec<i32>>>(),
            vec![vec![1], vec![2], vec![3], vec![4], vec![5]]
        );

        assert_eq!(
            (1..=4).slide(4).collect::<Vec<Vec<i32>>>(),
            vec![vec![1, 2, 3, 4]]
        );

        assert_eq!(
            (1..=2).slide(4).collect::<Vec<Vec<i32>>>(),
            vec![vec![1, 2]]
        );

        assert_eq!((1..=1).slide(3).collect::<Vec<Vec<i32>>>(), vec![vec![1]]);
    }
}
