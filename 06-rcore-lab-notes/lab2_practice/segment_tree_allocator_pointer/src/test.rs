use crate::Heap;
use crate::segment_tree::SegTreeNode;
use core::alloc::Layout;
use core::mem::size_of;

#[test]
fn test_seg_tree() {
    let mut root = SegTreeNode::build(0, 63);
    assert_eq!(root.alloc(63), Some(0));
    assert_eq!(root.alloc(63), None);
    assert_eq!(root.alloc(1), None);
    assert_eq!(root.dealloc(0, 63), Some(64));

    assert_eq!(root.alloc(64), Some(0));
    assert_eq!(root.dealloc(0, 63), Some(64));

    assert_eq!(root.alloc(32), Some(0));
    assert_eq!(root.alloc(32), Some(32));
    assert_eq!(root.alloc(32), None);
    assert_eq!(root.dealloc(0, 32), Some(32));
    assert_eq!(root.dealloc(32, 32), Some(32));

    assert_eq!(root.alloc(9), Some(0));
    assert_eq!(root.alloc(9), Some(16));
    assert_eq!(root.alloc(9), Some(32));
    assert_eq!(root.alloc(9), Some(48));
    assert_eq!(root.alloc(9), None);
    assert_eq!(root.dealloc(32, 9), Some(16));
    assert_eq!(root.dealloc(48, 9), Some(16));
    assert_eq!(root.alloc(32), Some(32));
    assert_eq!(root.dealloc(16, 9), Some(16));
    assert_eq!(root.dealloc(0, 9), Some(16));
    assert_eq!(root.alloc(32), Some(0));
    assert_eq!(root.dealloc(32, 32), Some(32));
    assert_eq!(root.dealloc(0, 32), Some(32));
}

#[test]
fn test_empty_heap() {
    let mut heap = Heap::new();
    assert!(heap.alloc(Layout::from_size_align(1, 1).unwrap()).is_err());
}

#[test]
fn test_heap_add() {
    let mut heap = Heap::new();
    assert!(heap.alloc(Layout::from_size_align(1, 1).unwrap()).is_err());

    let space: [usize; 100] = [0; 100];
    unsafe {
        heap.add_to_heap(space.as_ptr() as usize, space.as_ptr().add(100) as usize);
    }
    let addr = heap.alloc(Layout::from_size_align(1, 1).unwrap());
    assert!(addr.is_ok());
}

#[test]
fn test_heap_oom() {
    let mut heap = Heap::new();
    let space: [usize; 100] = [0; 100];
    unsafe {
        heap.add_to_heap(space.as_ptr() as usize, space.as_ptr().add(100) as usize);
    }

    assert!(heap
        .alloc(Layout::from_size_align(101 * size_of::<usize>(), 1).unwrap())
        .is_err());
    assert!(heap
        .alloc(Layout::from_size_align(100 * size_of::<usize>(), 1).unwrap())
        .is_ok());
    assert!(heap.alloc(Layout::from_size_align(1, 1).unwrap()).is_err());
}

#[test]
fn test_heap_alloc_and_free() {
    let mut heap = Heap::new();
    assert!(heap.alloc(Layout::from_size_align(1, 1).unwrap()).is_err());

    let space: [usize; 100] = [0; 100];
    unsafe {
        heap.add_to_heap(space.as_ptr() as usize, space.as_ptr().add(100) as usize);
    }
    for _ in 0..100 {
        let addr = heap.alloc(Layout::from_size_align(1, 1).unwrap()).unwrap();
        heap.dealloc(addr, Layout::from_size_align(1, 1).unwrap());
    }
}
