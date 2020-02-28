mod frame_allocator;
pub mod paging;
pub mod memory_set;

use frame_allocator::SEGMENT_TREE_ALLOCATOR as FRAME_ALLOCATOR;
use frame_allocator::FIRST_FIT_ALLOCATOR as FRAME_ALLOCATOR_TEST;

use riscv::addr::{
    // 分别为虚拟地址、物理地址、虚拟页、物理页帧
    VirtAddr,
    PhysAddr,
    Page,
    Frame
};
use crate::consts::*;
use buddy_system_allocator::LockedHeap;
use memory_set::{
    MemorySet,
    attr::MemoryAttr,
    handler::Linear
};

pub fn init(l: usize, r: usize) {
    FRAME_ALLOCATOR.lock().init(l, r);
    init_heap();
    kernel_remap();
    println!("++++ setup memory! ++++");
}

pub fn alloc_frame() -> Option<Frame> {
    // 将物理页号转为物理页帧
    Some(Frame::of_ppn(FRAME_ALLOCATOR.lock().alloc()))
}
pub fn dealloc_frame(f: Frame) {
    FRAME_ALLOCATOR.lock().dealloc(f.number())
}

pub fn init_allocator(l: usize, r: usize) {
    FRAME_ALLOCATOR_TEST.lock().init(l, r);
}

pub fn alloc_frames(cnt: usize) -> Option<Frame> {
    // 将物理页号转为物理页帧
    let p = FRAME_ALLOCATOR_TEST.lock().alloc(cnt);
    if !p.is_none() {
        return Some(Frame::of_ppn(p.unwrap()));
    }
    return None;
}
pub fn dealloc_frames(f: Frame, cnt: usize) {
    FRAME_ALLOCATOR_TEST.lock().dealloc(f.number(), cnt);
}

fn init_heap() {
    // 同样是在内核中开一块静态内存供 buddy system allocator 使用
    static mut HEAP: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE];
    unsafe {
        // 这里需要先开锁
        DYNAMIC_ALLOCATOR
            .lock()
            .init(HEAP.as_ptr() as usize, KERNEL_HEAP_SIZE);
    }
}

pub fn access_pa_via_va(pa: usize) -> usize {
    pa + PHYSICAL_MEMORY_OFFSET
}

pub fn kernel_remap() {
    let mut memory_set = MemorySet::new();

    extern "C" {
        fn bootstack();
        fn bootstacktop();
    }
    memory_set.push(
        bootstack as usize,
        bootstacktop as usize,
        MemoryAttr::new(),
        Linear::new(PHYSICAL_MEMORY_OFFSET)
    );
    
    unsafe {
        memory_set.activate();
    }
}

#[global_allocator]
static DYNAMIC_ALLOCATOR: LockedHeap = LockedHeap::empty();

#[alloc_error_handler]
fn alloc_error_handler(_: core::alloc::Layout) -> ! {
    panic!("alloc_error_handler do nothing but panic!");
}
