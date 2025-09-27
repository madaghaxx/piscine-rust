use std::ops::Add;
pub struct StepIterator<T> {
    beg: T,
    end: T,
    step: T,
}

impl<T> StepIterator<T> {
    pub fn new(beg: T, end: T, step: T) -> StepIterator<T> {
        StepIterator {
            beg,
            end,
            step,
        }
    }
}
impl<T: Copy + PartialOrd + Add<Output = T>> std::iter::Iterator for StepIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.beg > self.end {
            return None;
        }
        let current = self.beg;
        self.beg = self.beg + self.step;
        Some(current)
    }
}
