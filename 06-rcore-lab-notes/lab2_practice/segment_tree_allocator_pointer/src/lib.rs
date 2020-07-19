#![no_std]
#![feature(const_fn)]
#![feature(allocator_api)]

extern crate spin;
extern crate alloc;

use spin::Mutex;
use core::cmp::max;
use core::ops::Deref;
use core::marker::Send;
use core::mem::size_of;
use core::ptr::NonNull;
use core::alloc::GlobalAlloc;
use alloc::alloc::{AllocErr, Layout};

pub mod segment_tree;

#[cfg(test)]
mod test;

pub struct Heap {
    root: segment_tree::SegTreeNode,
    user: usize,
    allocated: usize,
    total: usize,
}

impl Heap {
    pub const fn new() -> Self {
        Heap {
            root: segment_tree::SegTreeNode::new(),
            user: 0,
            allocated: 0,
            total: 0,
        }
    }

    pub const fn empty() -> Self {
        Self::new()
    }

    pub fn add_to_heap(&mut self, start: usize, end: usize) {
        self.root = segment_tree::SegTreeNode::build(start, end);
        self.total = end - start;
    }

    pub fn init(&mut self, start: usize, size: usize) {
        self.add_to_heap(start, start + size);
    }

    pub fn alloc(&mut self, layout: Layout) -> Result<NonNull<u8>, AllocErr> {
        let size = max(
            layout.size().next_power_of_two(),
            max(layout.align(), size_of::<usize>()),
        );
        if let Some(start) = self.root.alloc(layout.size()) {
            let result = NonNull::new(start as *mut u8);
            if let Some(result) = result {
                self.user += layout.size();
                self.allocated += size;
                return Ok(result);
            } else {
                return Err(AllocErr {});
            }
        }
        Err(AllocErr {})
    }

    pub fn dealloc(&mut self, ptr: NonNull<u8>, layout: Layout) {
        let start = ptr.as_ptr() as usize;
        if let Some(size) = self.root.dealloc(start, layout.size()) {
            self.user -= layout.size();
            self.allocated -= size;
        }
    }

    pub fn stats_alloc_user(&self) -> usize {
        self.user
    }

    pub fn stats_alloc_actual(&self) -> usize {
        self.allocated
    }

    pub fn stats_total_bytes(&self) -> usize {
        self.total
    }
}

unsafe impl Send for Heap {}

pub struct LockedHeap(Mutex<Heap>);

impl LockedHeap {
    pub const fn new() -> LockedHeap {
        LockedHeap(Mutex::new(Heap::new()))
    }

    pub const fn empty() -> LockedHeap {
        LockedHeap(Mutex::new(Heap::new()))
    }
}

impl Deref for LockedHeap {
    type Target = Mutex<Heap>;

    fn deref(&self) -> &Mutex<Heap> {
        &self.0 }
}

unsafe impl GlobalAlloc for LockedHeap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.0
            .lock()
            .alloc(layout)
            .ok()
            .map_or(0 as *mut u8, |allocation| allocation.as_ptr())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.0.lock().dealloc(NonNull::new_unchecked(ptr), layout);
    }
}

