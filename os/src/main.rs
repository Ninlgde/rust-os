#![no_std]
#![no_main]
#![feature(panic_info_message)]

#[macro_use]
mod console;
#[macro_use]
mod logging;
mod config;
mod timer;
mod stack_trace;
mod lang_items;
mod sbi;

#[cfg(feature = "board_qemu")]
#[path = "boards/qemu.rs"]
mod board;
mod loader;
mod sync;
mod trap;
mod syscall;
mod task;

use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));


#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    logging::init();
    println!("[kernel] Hello, world!");
    trap::init();
    loader::load_apps();
    task::run_first_task();
    panic!("Unreachable in rust_main!");
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}