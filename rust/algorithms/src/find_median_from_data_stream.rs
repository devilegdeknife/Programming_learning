use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct MedianFinder {
    max_heap: BinaryHeap<i32>,
    min_heap: BinaryHeap<Reverse<i32>>,
}

impl MedianFinder {
    fn new() -> Self {
        MedianFinder {
            max_heap: BinaryHeap::new(),
            min_heap: BinaryHeap::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        self.max_heap.push(num);

        if self.max_heap.len() > self.min_heap.len() + 1 {
            let max_top = self.max_heap.pop().unwrap();
            self.min_heap.push(Reverse(max_top));
        }

        let max_top = self.max_heap.peek().unwrap();
        let min_top = self.min_heap.peek().map(|r| &r.0).unwrap_or(&i32::MAX);

        if max_top > min_top {
            let max_top = self.max_heap.pop().unwrap();
            let min_top = self.min_heap.pop().map(|r| r.0).unwrap_or(i32::MAX);
            self.max_heap.push(min_top);
            self.min_heap.push(Reverse(max_top));
        }
    }

    fn find_median(&self) -> f64 {
        if self.max_heap.len() == self.min_heap.len() {
            let max_top = *self.max_heap.peek().unwrap();
            let min_top = self.min_heap.peek().map(|r| r.0).unwrap_or(i32::MAX);
            (max_top as f64 + min_top as f64) / 2.0
        } else {
            *self.max_heap.peek().unwrap() as f64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MedianFinder;

    #[test]
    fn test_median_finder() {
        let mut finder = MedianFinder::new();

        finder.add_num(1);
        assert_eq!(finder.find_median(), 1.0);

        finder.add_num(2);
        assert_eq!(finder.find_median(), 1.5);

        finder.add_num(3);
        assert_eq!(finder.find_median(), 2.0);

        finder.add_num(4);
        assert_eq!(finder.find_median(), 2.5);

        finder.add_num(5);
        assert_eq!(finder.find_median(), 3.0);
    }
}