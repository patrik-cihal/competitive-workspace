use std::ops::Range;

pub struct SegmentTree<T, F> {
    n: usize,
    data: Vec<T>,
    neutral: T,
    f: F,
}

impl<T, F> SegmentTree<T, F>
where
    T: Clone,
    F: Fn(&T, &T) -> T,
{
    pub fn new(n: usize, neutral: T, f: F) -> Self {
        let mut size = 1;
        while size < n {
            size *= 2;
        }
        Self {
            n: size,
            data: vec![neutral.clone(); 2 * size],
            neutral,
            f,
        }
    }

    pub fn new_from_iter<I>(iter: I, neutral: T, f: F) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let v: Vec<_> = iter.into_iter().collect();
        let mut size = 1;
        while size < v.len() {
            size *= 2;
        }
        let mut data = vec![neutral.clone(); 2 * size];
        for i in 0..v.len() {
            data[size + i] = v[i].clone();
        }
        for i in (1..size).rev() {
            data[i] = (f)(&data[2 * i], &data[2 * i + 1]);
        }
        Self {
            n: size,
            data,
            neutral,
            f,
        }
    }

    pub fn update(&mut self, i: usize, value: T) {
        let mut i = i + self.n;
        self.data[i] = value;
        while i > 1 {
            i /= 2;
            self.data[i] = (self.f)(&self.data[2 * i], &self.data[2 * i + 1]);
        }
    }

    pub fn get(&self, i: usize) -> &T {
        &self.data[i + self.n]
    }

    pub fn query(&self, range: Range<usize>) -> T {
        self.query_inner(range, 1, 0..self.n)
    }

    fn query_inner(&self, range: Range<usize>, i: usize, inner_range: Range<usize>) -> T {
        if range.start >= range.end {
            return self.neutral.clone();
        }
        if range.start == inner_range.start && range.end == inner_range.end {
            return self.data[i].clone();
        }
        let mid = (inner_range.start + inner_range.end) / 2;
        let left = self.query_inner(range.start..mid.min(range.end), 2 * i, inner_range.start..mid);
        let right = self.query_inner(mid.max(range.start)..range.end, 2 * i + 1, mid..inner_range.end);
        (self.f)(&left, &right) 
    }

    pub fn data(&self) -> &[T] {
        &self.data[self.n..]
    }
}
