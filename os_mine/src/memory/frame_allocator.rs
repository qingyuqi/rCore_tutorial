use crate::consts::MAX_PHYSICAL_PAGES;
use spin::Mutex;

pub struct SegmentTreeAllocator {
    a: [u8; MAX_PHYSICAL_PAGES << 1],
    m: usize,
    n: usize,
    offset: usize
}

impl SegmentTreeAllocator {
    // 使用物理页号区间 [l, r) 进行初始化
    pub fn init(&mut self, l: usize, r: usize) {
        self.offset = l - 1;
        self.n = r - l;
        self.m = 1;
        while self.m < self.n + 2 {
            self.m = self.m << 1;
        }
        for i in (1..(self.m << 1)) { 
            self.a[i] = 1; 
        }
        for i in (1..self.n) { 
            self.a[self.m + i] = 0; 
        }
        for i in (1..self.m).rev() { 
            self.a[i] = self.a[i << 1] & self.a[(i << 1) | 1]; 
        }
    }
    // 分配一个物理页
    // 自上而下寻找可用最小物理页号
    // 注意，假定永远不会出现物理页耗尽的情况
    pub fn alloc(&mut self) -> usize {
        if self.a[1] == 1 {
            panic!("physical memory depleted!");
        }
        let mut p = 1;
        while p < self.m {
            if self.a[p << 1] == 0 {
                p = p << 1;
            } else {
                p = (p << 1) | 1;
            }
        }
        let result = p + self.offset - self.m;
        self.a[p] = 1;
        p >>= 1;
        while p > 0 {
            self.a[p] = self.a[p << 1] & self.a[(p << 1) | 1];
            p >>= 1;
        }
        result
    }
    // 回收物理页号为 n 的物理页
    // 自上而下进行更新
    pub fn dealloc(&mut self, n: usize) {
        let mut p = n + self.m - self.offset;
        assert!(self.a[p] == 1);
        self.a[p] = 0;
        p >>= 1;
        while p > 0 {
            self.a[p] = self.a[p << 1] & self.a[(p << 1) | 1];
            p >>= 1;
        }
    }
}

pub static SEGMENT_TREE_ALLOCATOR: Mutex<SegmentTreeAllocator> = Mutex::new(SegmentTreeAllocator {
    a: [0; MAX_PHYSICAL_PAGES << 1],
    m: 0,
    n: 0,
    offset: 0
});

pub struct FirstFitAllocator {
    a: [u8; MAX_PHYSICAL_PAGES],
    n: usize,
    offset: usize
}

impl FirstFitAllocator {
    pub fn init(&mut self, l: usize, r: usize) {
        self.offset = l - 1;
        self.n = r - l + 1;
        for i in (1..self.n) {
            self.a[i] = 0;
        }
    }
    pub fn alloc(&mut self, cnt: usize) -> Option<usize> {
        let mut result = 1;
        for i in (1..self.n) {
            if self.a[i] == 1 {
                result = i + 1;
            } else {
                if i - result + 1 >= cnt {
                    for j in (result..i + 1) {
                        assert!(self.a[j] == 0);
                        self.a[j] = 1;
                    }
                    return Some(result);
                }
            }
        }
        return None;
    }
    pub fn dealloc(&mut self, ppn: usize, cnt: usize) {
        let mut p = ppn - self.offset;
        for i in (p..(p + cnt)) {
            assert!(self.a[i] == 1);
            self.a[i] = 0;
        }
    }
}

pub static FIRST_FIT_ALLOCATOR: Mutex<FirstFitAllocator> = Mutex::new(FirstFitAllocator {
    a: [0; MAX_PHYSICAL_PAGES],
    n: 0,
    offset: 0
});