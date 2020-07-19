use core::cmp::max;

const MAX_NODE: usize = 1024 * 4;

pub struct SegTree {
    node: [SegTreeNode; MAX_NODE],
}

#[derive(Copy, Clone, Debug)]
pub struct SegTreeNode {
    m_start: usize,
    m_end: usize,
    m_max: usize,
    m_available: usize,
    m_used: bool,
}

impl SegTree {
    pub const fn new() -> SegTree {
        SegTree {
            node: [SegTreeNode::new(); MAX_NODE],
        }
    }

    pub fn build(&mut self, m_start: usize, m_end: usize) {
        self._build(1, m_start, m_end);
    }

    fn _build(&mut self, pos: usize, m_start: usize, m_end: usize) {
        assert!(m_start <= m_end);
        let m_available = m_end - m_start + 1;
        self.node[pos] = SegTreeNode {
            m_start,
            m_end,
            m_max: m_available,
            m_available,
            m_used: false,
        };
        if m_start == m_end {
            return;
        }
        let m_mid = (m_start + m_end) >> 1;
        self._build(pos << 1, m_start, m_mid);
        self._build(pos << 1 | 1, m_mid + 1, m_end);
    }

    fn push_up(&mut self, pos: usize) {
        if self.node[pos].m_max == 1 {
            return;
        }
        let left_max = self.node[pos << 1].m_max;
        let left_available = self.node[pos << 1].m_available;
        let right_max = self.node[pos << 1 | 1].m_max;
        let right_available = self.node[pos << 1 | 1].m_available;

        self.node[pos].m_available = if left_max == left_available && right_max == right_available {
            self.node[pos].m_max
        } else {
            max(left_available, right_available)
        }
    }

    pub fn alloc(&mut self, m_size: usize) -> Option<usize> {
        self._alloc(1, m_size)
    }

    fn _alloc(&mut self, pos: usize, m_size: usize) -> Option<usize> {
        if m_size > self.node[pos].m_available {
            return None;
        }
        if self.node[pos].m_available == self.node[pos].m_max
            && m_size > (self.node[pos].m_max >> 1) {
                self.node[pos].m_available = 0;
                self.node[pos].m_used = true;
                return Some(self.node[pos].m_start);
        }
        if self.node[pos].m_max == 1 {
            return None;
        }
        let mut result = self._alloc(pos << 1, m_size);
        if result == None {
            result = self._alloc(pos << 1 | 1, m_size);
        }
        self.push_up(pos);
        result
    }

    pub fn dealloc(&mut self, m_start: usize, m_size: usize) -> Option<usize> {
        self._dealloc(1, m_start, m_size)
    }

    fn _dealloc(&mut self, pos: usize, m_start: usize, m_size: usize) -> Option<usize> {
        assert!(m_size <= self.node[pos].m_max);
        if m_start == self.node[pos].m_start && self.node[pos].m_used
            && m_size > (self.node[pos].m_max >> 1) {
                self.node[pos].m_available = self.node[pos].m_max;
                self.node[pos].m_used = false;
                return Some(self.node[pos].m_max);
        }
        let m_mid = (self.node[pos].m_start + self.node[pos].m_end) >> 1;
        let result = if m_start <= m_mid {
            self._dealloc(pos << 1, m_start, m_size)
        } else {
            self._dealloc(pos << 1 | 1, m_start, m_size)
        };
        self.push_up(pos);
        result
    }

    /*
    pub fn print(&self, pos: usize) {
        println!("{}: {:?}", pos, self.node[pos]);
    }

    pub fn pre_traversal(&self) {
        self._pre_traversal(1);
    }

    fn _pre_traversal(&self, pos: usize) {
        self.print(pos);
        if self.node[pos].m_max == 1 {
            return;
        }
        self._pre_traversal(pos << 1);
        self._pre_traversal(pos << 1 | 1);
    }
    */
}

impl SegTreeNode {
    pub const fn new() -> SegTreeNode {
        SegTreeNode {
            m_start: 0,
            m_end: 0,
            m_max: 0,
            m_available: 0,
            m_used: true,
        }
    }
}

