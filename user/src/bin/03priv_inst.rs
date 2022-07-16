#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use core::arch::asm;

#[no_mangle]
fn main() -> i32 {
    info!("Try to execute privileged instruction in U Mode");
    warn!("Kernel should kill this application!");
    unsafe {
        asm!("sret");
    }
    0
}
