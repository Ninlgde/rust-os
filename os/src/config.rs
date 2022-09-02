//! Constants used in rCore

pub const USER_STACK_SIZE: usize = 4096 * 2;
pub const KERNEL_STACK_SIZE: usize = 4096 * 2;
pub const KERNEL_HEAP_SIZE: usize = 0x20_0000;

/// 页大小 4096 bytes
pub const PAGE_SIZE: usize = 0x1000;
/// 页大小的bits 1 << PAGE_SIZE_BITS = PAGE_SIZE
pub const PAGE_SIZE_BITS: usize = 0xc;

/// 跳板的物理地址
pub const TRAMPOLINE: usize = usize::MAX - PAGE_SIZE + 1;
pub const TRAP_CONTEXT: usize = TRAMPOLINE - PAGE_SIZE;

/*
#[cfg(feature = "board_k210")]
pub const CLOCK_FREQ: usize = 403000000 / 62;

#[cfg(feature = "board_qemu")]
pub const CLOCK_FREQ: usize = 12500000;
*/
pub use crate::board::{CLOCK_FREQ, MEMORY_END, MMIO};
