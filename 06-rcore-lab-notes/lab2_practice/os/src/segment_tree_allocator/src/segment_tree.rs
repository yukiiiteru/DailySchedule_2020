#![allow(dead_code)]

use alloc::rc::Rc;
use core::cmp::max;
use core::cell::RefCell;

pub struct SegTreeNode {
    l_node: Option<Rc<RefCell<SegTreeNode>>>,
    r_node: Option<Rc<RefCell<SegTreeNode>>>,
    m_start: usize,
    m_end: usize,
    m_max: usize,
    m_available: usize,
    m_used: bool,
}

impl SegTreeNode {
    pub const fn new() -> SegTreeNode {
        SegTreeNode {
            l_node: None,
            r_node: None,
            m_start: 0,
            m_end: 0,
            m_max: 0,
            m_available: 0,
            m_used: false,
        }
    }

    pub fn build(m_start: usize, m_end: usize) -> SegTreeNode {
        if m_start == m_end {
            return SegTreeNode {
                l_node: None,
                r_node: None,
                m_start,
                m_end,
                m_max: 1,
                m_available: 1,
                m_used: false,
            }
        }
        let m_mid = (m_start + m_end) >> 1; 
        let l_node = Self::build(m_start, m_mid);
        let r_node = Self::build(m_mid+1, m_end);
        let m_available = m_end - m_start + 1;
        SegTreeNode {
            l_node: Some(Rc::new(RefCell::new(l_node))),
            r_node: Some(Rc::new(RefCell::new(r_node))),
            m_start,
            m_end,
            m_max: m_available,
            m_available,
            m_used: false,
        }
    }

    fn push_up(&mut self) {
        let left_max = if let Some(node) = &self.l_node { node.borrow().m_max } else { 0 };
        let left_available = if let Some(node) = &self.l_node {
            if node.borrow().m_used { 0 } else { node.borrow().m_available }
        } else { 0 };
        let right_max = if let Some(node) = &self.r_node { node.borrow().m_max } else { 0 };
        let right_available = if let Some(node) = &self.r_node {
            if node.borrow().m_used { 0 } else { node.borrow().m_available }
        } else { 0 };

        self.m_available = if left_max == left_available && right_max == right_available {
            self.m_max
        } else {
            max(left_available, right_available)
        }
    }

    pub fn alloc(&mut self, m_size: usize) -> Option<usize> {
        if m_size > self.m_available {
            return None;
        }
        if self.m_available == self.m_max && m_size > (self.m_max >> 1) {
            self.m_available = 0;
            self.m_used = true;
            return Some(self.m_start);
        }
        let mut result:Option<usize> = None;
        if let Some(node) = &self.l_node {
            result = node.borrow_mut().alloc(m_size);
        }
        if result == None {
            if let Some(node) = &self.r_node {
                result = node.borrow_mut().alloc(m_size);
            }
        }
        self.push_up();
        result
    }

    pub fn dealloc(&mut self, m_start: usize, m_size: usize) -> Option<usize> {
        assert!(m_size <= self.m_max);
        if self.m_start == m_start && self.m_used && m_size > (self.m_max >> 1) {
            self.m_available = self.m_max;
            self.m_used = false;
            return Some(self.m_max);
        }
        let mut result: Option<usize> = None;
        let m_mid = (self.m_start + self.m_end) >> 1;
        if m_start < m_mid {
            if let Some(node) = &self.l_node {
                result = node.borrow_mut().dealloc(m_start, m_size);
            }
        } else {
            if let Some(node) = &self.r_node {
                result = node.borrow_mut().dealloc(m_start, m_size);
            }
        }
        self.push_up();
        return result;
    }
}

