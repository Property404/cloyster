use core::arch::global_asm;

#[cfg(target_arch = "x86_64")]
global_asm!(include_str!("x86_64.S"));
#[cfg(target_arch = "riscv64")]
global_asm!(include_str!("riscv64.S"));
