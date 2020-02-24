global_asm!(include_str!("boot/entry64.asm"));

use crate::io;
use crate::sbi;
use crate::consts::*;

use crate::memory::{
    alloc_frame,
    dealloc_frame
};

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    extern "C" {
        fn end();
    }
    println!("kernel end vaddr = {:#x}", end as usize);
    println!(
        "free physical memory ppn = [{:#x}, {:#x})", 
        ((end as usize - KERNEL_BEGIN_VADDR + KERNEL_BEGIN_PADDR) >> 12) + 1,
        PHYSICAL_MEMORY_END >> 12
    );
    crate::interrupt::init();

    crate::memory::init(
        ((end as usize - KERNEL_BEGIN_VADDR + KERNEL_BEGIN_PADDR) >> 12) + 1,
        PHYSICAL_MEMORY_END >> 12
    );

    frame_allocating_test();
    dynamic_allocating_test();

    let grade = FirstFitAllocator_test();
    println!("grade for allocator test: {}", grade);

    crate::timer::init();

    unsafe {
        asm!("ebreak"::::"volatile");
        // asm!("mret"::::"volatile");
    }
    panic!("end of rust_main");
    loop {}
}

fn frame_allocating_test() {
    println!("alloc {:x?}", alloc_frame());
    let f = alloc_frame();
    println!("alloc {:x?}", f);
    println!("alloc {:x?}", alloc_frame());
    println!("dealloc {:x?}", f);
    dealloc_frame(f.unwrap());
    println!("alloc {:x?}", alloc_frame());
    println!("alloc {:x?}", alloc_frame());
}

fn dynamic_allocating_test() {
    use alloc::vec::Vec;
    use alloc::boxed::Box;

    extern "C" {
        fn sbss();
        fn ebss();
    }
    let lbss = sbss as usize;
    let rbss = ebss as usize;

    let heap_value = Box::new(5);
    assert!(*heap_value == 5);
    println!("heap_value assertion successfully!");
    println!("heap_value is at {:p}", heap_value);
    let heap_value_addr = &*heap_value as *const _ as usize;
    assert!(heap_value_addr >= lbss && heap_value_addr < rbss);
    println!("heap_value is in section .bss!");

    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    for i in 0..500 {
        assert!(vec[i] == i);
    }
    println!("vec assertion successfully!");
    println!("vec is at {:p}", vec.as_slice());
    let vec_addr = vec.as_ptr() as usize;
    assert!(vec_addr >= lbss && vec_addr < rbss);
    println!("vec is in section .bss!");
}

use riscv::addr::Frame;

fn alloc(cnt: usize) -> Option<usize> {
    if let Some(frames) = crate::memory::alloc_frames(cnt) {
        return Some(frames.number());
    }
    return None;
}

fn dealloc(pnn: usize, cnt: usize) {
    crate::memory::dealloc_frames(Frame::of_ppn(pnn), cnt)
}

fn FirstFitAllocator_test() -> usize {
    let mut grade: usize = 0;
    crate::memory::init_allocator(1, 6);
    let mut p0 = alloc(5);
    if p0.is_none() {
        return grade;
    }
    let mut p0 = p0.unwrap();
    if !alloc(1).is_none() {
        return grade;
    }
    dealloc(p0 + 2, 3);
    if !alloc(4).is_none() {
        return grade;
    } else {
        grade += 1 ;
    }
    let mut p1 = alloc(3);
    if p1.is_none() {
        return grade;
    } else {
        grade += 1 ;
    }
    let mut p1 = p1.unwrap();
    if !alloc(1).is_none() {
        return grade;
    } else {
        grade += 1 ;
    }
    if p0 + 2 != p1 {
        return grade;
    } else {
        grade += 1 ;
    }
    let mut p2 = p0 + 1;
    dealloc(p0, 1);
    dealloc(p1, 3);
    p0 = alloc(1).unwrap();
    if p0 != p2 - 1 {
        return grade;
    } else {
        grade += 1 ;
    }
    dealloc(p0, 1);
    p0 = alloc(2).unwrap();
    if p0 != p2 + 1 {
        return grade;
    } else {
        grade += 1 ;
    }
    dealloc(p0, 2);
    dealloc(p2, 1);
    let mut p0 = alloc(5);
    if p0.is_none() {
        return grade;
    } else {
        grade += 1 ;
    }
    if !alloc(1).is_none() {
        return grade;
    } else {
        grade += 1 ;
    }
    dealloc(p0.unwrap(), 5);
    return grade;
}