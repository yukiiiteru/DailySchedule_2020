use crate::Heap;
use core::mem::size_of;
use core::alloc::Layout;
use crate::segment_tree::SegTree;

#[test]
fn test_seg_tree() {
    let mut tree = SegTree::new();
    tree.build(0, 63);
    assert_eq!(tree.alloc(63), Some(0));
    assert_eq!(tree.alloc(63), None);
    assert_eq!(tree.alloc(1), None);
    assert_eq!(tree.dealloc(0, 63), Some(64));

    assert_eq!(tree.alloc(64), Some(0));
    assert_eq!(tree.dealloc(0, 63), Some(64));

    assert_eq!(tree.alloc(32), Some(0));
    assert_eq!(tree.alloc(32), Some(32));
    assert_eq!(tree.alloc(32), None);
    assert_eq!(tree.dealloc(0, 32), Some(32));
    assert_eq!(tree.dealloc(32, 32), Some(32));

    assert_eq!(tree.alloc(9), Some(0));
    assert_eq!(tree.alloc(9), Some(16));
    assert_eq!(tree.alloc(9), Some(32));
    assert_eq!(tree.alloc(9), Some(48));
    assert_eq!(tree.alloc(9), None);
    assert_eq!(tree.dealloc(32, 9), Some(16));
    assert_eq!(tree.dealloc(48, 9), Some(16));
    assert_eq!(tree.alloc(32), Some(32));
    assert_eq!(tree.dealloc(16, 9), Some(16));
    assert_eq!(tree.dealloc(0, 9), Some(16));
    assert_eq!(tree.alloc(32), Some(0));
    assert_eq!(tree.dealloc(32, 32), Some(32));
    assert_eq!(tree.dealloc(0, 32), Some(32));

    assert_eq!(tree.alloc(1), Some(0));
    assert_eq!(tree.dealloc(0, 1), Some(1));
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
    assert!(heap.alloc(Layout::from_size_align(1, 1).unwrap()).is_ok());
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

