use crate::sbi::set_timer;
use riscv::register::{
    time,
    sie
};

// 当前已经触发多少次时钟中断
pub static mut TICKS: usize = 0;
// 时钟中断触发间隔
// 数值一般约为CPU频率的1%，防止过多占用CPU
static TIMEBASE: u64 = 100000;
pub fn init() {
    unsafe {
        // 初始化时钟中断触发次数
        TICKS = 0;
        sie::set_stimer();
    }
    // 硬件机制问题我们不能直接设置时钟中断触发间隔
    // 只能当每一次时钟中断触发时
    // 设置下一次时钟中断的触发时间
    // 设置为当前时间加上 TIMEBASE
    // 这次调用用来预处理
    clock_set_next_event();
    println!("++++ setup timer! ++++");
}

pub fn clock_set_next_event() {
    // 调用OpenSBI提供的接口设置下次时钟中断时间
    set_timer(get_cycle() + TIMEBASE);
}

fn get_cycle() -> u64 {
    time::read() as u64
}