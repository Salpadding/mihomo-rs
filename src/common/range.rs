#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Range<T> {
    pub start: T,
    pub end: T,
}

impl<T: PartialOrd> Range<T> {
    pub fn new(start: T, end: T) -> Self {
        if start > end {
            Self { start: end, end: start }
        } else {
            Self { start, end }
        }
    }

    /// [start, end]
    pub fn contains(&self, t: &T) -> bool {
        t >= &self.start && t <= &self.end
    }

    /// [start, end)
    pub fn left_contains(&self, t: &T) -> bool {
        t >= &self.start && t < &self.end
    }

    /// (start, end]
    pub fn right_contains(&self, t: &T) -> bool {
        t > &self.start && t <= &self.end
    }

    pub fn start(&self) -> &T {
        &self.start
    }

    pub fn end(&self) -> &T {
        &self.end
    }
}