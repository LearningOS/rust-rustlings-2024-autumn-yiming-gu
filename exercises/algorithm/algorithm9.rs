/*
	heap
	This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default + Copy + std::fmt::Display + std::cmp::PartialOrd,
{
    count: usize,
    iter_count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default + Copy + std::fmt::Display + std::cmp::PartialOrd,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            iter_count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        //TODO
        if self.count == 0 {
            self.items[0] = value;
            self.count += 1;
            return;
        }
        let mut idx = self.count;
        let mut parent_idx = self.parent_idx(idx);
        self.items.push(value);
        while (self.comparator)(&value, &self.items[parent_idx]) {
            self.items.swap(idx, parent_idx);
            idx = parent_idx;
            let left = self.left_child_idx(idx);
            let right = self.right_child_idx(idx);
            if (left <= self.count && right <= self.count) {
                if !(self.comparator)(&self.items[left], &self.items[right]) {
                    self.items.swap(left, right);
                }
            }
            parent_idx = self.parent_idx(idx);
            self.iter_count = idx;
        }
        self.count += 1;
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        //TODO
        if self.items[self.left_child_idx(idx)] > self.items[self.right_child_idx(idx)] {
            self.right_child_idx(idx)
        }
        else {
            self.left_child_idx(idx)
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord + Copy + std::fmt::Display + std::cmp::PartialOrd,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default + Copy + std::fmt::Display + std::cmp::PartialOrd,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        //TODO
        if self.count == 0 {
            None
        }
        else {
            let iter_count = self.iter_count;
            self.iter_count += 1;
            Some(self.items[iter_count])
        }
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + Copy + std::fmt::Display + std::cmp::PartialOrd,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + Copy + std::fmt::Display + std::cmp::PartialOrd,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}