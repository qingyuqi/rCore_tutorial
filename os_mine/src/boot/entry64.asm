    .section .text.entry
    .globl _start
_start:
    # to := 三级页表虚拟地址
    lui t0, %hi(boot_page_table_sv39)
    # t1 := 0xffffffff40000000 即虚实映射偏移量
    li t1, 0xffffffffc0000000 - 0x80000000
    # to 减去虚实映射偏移量，变为三级页表的物理地址
    sub t0, t0, t1
    # t0 >>= 12, 变为三级页表的物理页号
    srli t0, t0, 12

    # t1 := 8 << 60, 设置 satp 的 MODE 字段为 Sv39
    li t1, 8 << 60
    # 将刚才计算出的预设三级页表物理页号附加到 satp 中
    or t0, t0, t1
    # 将计算出的 t0 覆盖到 satp 中
    csrw satp, t0
    # 使用 sfence.vma 刷新 TLB
    sfence.vma

    lui sp, %hi(bootstacktop)
    
    lui t0, %hi(rust_main)
    addi t0, t0, %lo(rust_main)
    jr t0

    .section .bss.stack
    .align 12
    .global bootstack
bootstack:
    .space 4096 * 4
    .global bootstacktop
bootstacktop:

    .section .data
    # 由于要把这个页表放到一个页里面，因此必须 12 位对齐
    .align 12
# 分配 4KiB 空间内存给预设的三级页表
boot_page_table_sv39:
    # 0xffffffff_c00000000 map to 0x80000000 (1G)
    # 前511个表项设置为 0，因此 V=0，意味着是空的
    .zero 8 * 511
    # 设置最后一个页表项，PPN=0x80000，标志位 VRWXAD 均为 1
    .quad (0x80000 << 10) | 0xcf
