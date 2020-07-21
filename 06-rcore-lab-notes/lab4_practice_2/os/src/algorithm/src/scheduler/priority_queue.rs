use alloc::vec::Vec;
use core::fmt::Debug;

// #[derive(Clone, PartialEq)]
#[derive(Clone, PartialEq, Debug)]
pub struct PriorityObject<T, U> {
    pub object: T,
    pub priority: U,
}

#[derive(Debug)]
pub struct PriorityQueue<T, U> {
    heap: Vec<PriorityObject<T, U>>,
}

// impl<T: Clone + Eq, U: Clone + Copy + Ord> PriorityQueue<T, U> {
impl<T: Clone + Eq, U: Clone + Copy + Ord> PriorityQueue<T, U> {
    pub fn new() -> Self {
        Self {
            heap: Vec::<PriorityObject<T, U>>::new(),
        }
    }

    fn swap(&mut self, a: usize, b: usize) {
        let tmp = self.heap[b].clone();
        self.heap[b] = self.heap[a].clone();
        self.heap[a] = tmp.clone();
    }

    pub fn push(&mut self, object: T, priority: U) {
        self.heap.push(PriorityObject {
            object,
            priority,
        });
        let mut pos = self.heap.len() - 1;
        if pos == 0 { return; }
        let mut parent = (pos + 1) / 2 - 1;
        while pos != 0 && priority > self.heap[parent].priority {
            self.swap(pos, parent);
            pos = parent;
            if pos == 0 { break; }
            parent = (pos + 1) / 2 - 1;
        }
    }

    pub fn empty(&self) -> bool {
        self.heap.is_empty()
    }

    pub fn top(&self) -> Option<PriorityObject<T, U>> {
        if self.empty() {
            None
        } else {
            Some(self.heap[0].clone())
        }
    }

    pub fn pop(&mut self) -> Option<PriorityObject<T, U>> {
        if self.empty() {
            return None;
        }
        let len = self.heap.len() - 1;
        self.swap(0, len);
        let mut pos = 0;
        let mut child = (pos + 1) * 2 - 1;
        while child < len {
            if self.heap[child].priority < self.heap[child+1].priority && child < len - 1 {
                child += 1;
            }
            if self.heap[pos].priority >= self.heap[child].priority {
                break;
            }
            self.swap(pos, child);
            pos = child;
            child = (pos + 1) * 2 - 1;
        }
        self.heap.pop()
    }

    pub fn remove(&mut self, obj: &T) {
        for i in 0..self.heap.len() {
            if &self.heap[i].object == obj {
                // self.heap.remove(i);
                self.swap(0, i);
                self.pop();
                break;
            }
        }
    }

    pub fn set_priority(&mut self, obj: T, priority: U) {
        let len = self.heap.len();
        let mut pos = len;
        for i in 0..len {
            if self.heap[i].object == obj {
                self.heap[i].priority = priority;
                pos = i;
                break;
            }
        }
        if pos == len { return; }
        self.swap(pos, len-1);
        pos = len - 1;
        if pos == 0 { return; }
        let mut parent = (pos + 1) / 2 - 1;
        while pos != 0 && priority > self.heap[parent].priority {
            parent = (pos + 1) / 2 - 1;
            self.swap(pos, parent);
            pos = parent;
        }
    }
}

