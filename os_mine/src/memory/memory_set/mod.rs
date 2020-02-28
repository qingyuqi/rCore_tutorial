pub mod attr;
pub mod handler;
pub mod area;

use area::MemoryArea;
use attr::MemoryAttr;
use crate::memory::paging::PageTableImpl;
use crate::consts::*;
use handler::{
    MemoryHandler,
    Linear
};
use alloc::{
    boxed::Box,
    vec::Vec
};
use crate::memory::access_pa_via_va;

pub struct MemorySet {
    areas: Vec<MemoryArea>,
    page_table: PageTableImpl
}

impl MemorySet {
    pub fn push(&mut self, start: usize, end: usize, attr: MemoryAttr, handler: impl MemoryHandler) {
        // 加入一个新的给定了 handler, attr 的 MemoryArea
        // 合法性测试
        assert!(start <= end, "invalid memory area!");
        // 整段地址空间均未被占据
        assert!(self.test_free_area(start, end), "memory area overlap!");
        // 构造 MemoryArea
        let area = MemoryArea::new(start, end, Box::new(handler), attr);
        // 更新本 MemorySet 的映射
        area.map(&mut self.page_table);
        // 更新本 MemorySet 的 MemoryArea 集合
        self.areas.push(area);
    }
    fn test_free_area(&self, start: usize, end: usize) -> bool {
        self.areas.iter().find(|area| area.is_overlap_with(start, end)).is_none()
    }
    pub unsafe fn activate(&self) {
        self.page_table.activate();
    }

    pub fn new() -> Self {
        let mut memory_set = MemorySet {
            areas: Vec::new(),
            page_table: PageTableImpl::new_bare(),
        };
        memory_set.map_kernel_and_physical_memory();
        memory_set
    }
    pub fn map_kernel_and_physical_memory(&mut self) {
        extern "C" {
            fn stext();
            fn etext();
            fn srodata();
            fn erodata();
            fn sdata();
            fn edata();
            fn sbss();
            fn ebss();
            fn end();
        }
        let offset = PHYSICAL_MEMORY_OFFSET;
        self.push(
            stext as usize,
            etext as usize,
            MemoryAttr::new().set_readonly().set_execute(),
            Linear::new(offset)
        );
        self.push(
            srodata as usize,
            erodata as usize,
            MemoryAttr::new().set_readonly(),
            Linear::new(offset)
        );
        self.push(
            sdata as usize,
            edata as usize,
            MemoryAttr::new(),
            Linear::new(offset)
        );
        self.push(
            sbss as usize,
            ebss as usize,
            MemoryAttr::new(),
            Linear::new(offset)
        );
        self.push(
            (end as usize / PAGE_SIZE + 1) * PAGE_SIZE,
            access_pa_via_va(PHYSICAL_MEMORY_END),
            MemoryAttr::new(),
            Linear::new(offset)
        );
    }
}
