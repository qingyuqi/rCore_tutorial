use riscv::register::{
    sstatus::Sstatus,
    scause::Scause,
};

#[repr(C)]
// 中断帧结构体
pub struct TrapFrame {
    pub x: [usize; 32], // 普通寄存器
    pub sstatus: Sstatus, // Supervisor Status Register
    pub sepc: usize, // Supervisor exception program counter
    pub stval: usize, // Supervisor trap value
    pub scause: Scause, // Scause register 中断原因
}