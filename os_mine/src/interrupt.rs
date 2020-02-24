use riscv::register::{
    scause::{
        self,
        Trap,
        Exception,
        Interrupt
    },
    sepc,
    stvec,
    sscratch,
    sstatus
};
use crate::context::TrapFrame;
use crate::timer::{
    TICKS,
    clock_set_next_event
};

global_asm!(include_str!("trap/trap.asm"));

pub fn init() {
    unsafe {
        extern "C" {
            // 中断处理总入口
            fn __alltraps();
        };
        // 现在处于内核态， 要把 sscratch 初始化为0
        sscratch::write(0);
        // 仍使用 Direct 模式
        // 将中断处理总入口设置为 __alltraps
        stvec::write(__alltraps as usize, stvec::TrapMode::Direct);
        // 设置 sstatus 的 sie 位
        sstatus::set_sie();
    }
    println!("++++ setup interrupt! ++++");
}

// 中断分发及处理
#[no_mangle]
pub fn rust_trap(tf: &mut TrapFrame) {
    // 根据中断原因分类讨论
    match tf.scause.cause() {
        // 断点异常
        Trap::Exception(Exception::Breakpoint) => breakpoint(&mut tf.sepc),
        // S态时钟中断
        Trap::Interrupt(Interrupt::SupervisorTimer) => super_timer(),
        _ => {
            println!("{:?}", tf.scause.cause());
            panic!("undefined trap!")
        }
    }
}

// 断点中断处理，输出断点地址并改变中断返回地址
fn breakpoint(sepc: &mut usize) {
    println!("a breakpoint set @0x{:x}", sepc);
    *sepc += 2;
}

// S态时钟中断处理
fn super_timer() {
    // 设置下一次时钟中断发生时间
    clock_set_next_event();
    unsafe {
        // 更新时钟中断触发计数
        // 注意由于 TICKS 是 static mut 的
        // 后面会提到，多个线程都能访问这个变量
        // 如果同时进行 +1 操作，会造成计数错误或更多严重bug
        // 因此这是 unsafe 的，不过目前先不用管这个
        TICKS += 1;
        // 每触发 100 次时钟中断将计数清零并输出
        if (TICKS == 100) {
            TICKS = 0;
            println!("* 100 ticks *");
        }
    }
    // 由于一般都是在死循环内触发时钟中断
    // 因此我们同样的指令再执行一次也无妨
    // 因此不必修改 sepc
}