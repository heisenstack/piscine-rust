pub struct StepIterator<T> {
    current: T,    
    start: T,      
    end: T,       
    step: T,      
    started: bool,
}

use std::ops::Add;

impl<T: Copy + Add<Output = T> + PartialOrd> StepIterator<T> {
    pub fn new(start: T, end: T, step: T) -> Self {
        Self {
            current: start,
            start,
            end,
            step,
            started: false,
        }
    }
}

impl<T: Copy + Add<Output = T> + PartialOrd> Iterator for StepIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.started {
            self.started = true;
            return Some(self.start);
        }

        self.current = self.current + self.step;

        if self.current > self.end {
            return None;
        }

        Some(self.current)
    }
}
