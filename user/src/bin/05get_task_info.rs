#![no_std]
#![no_main]

extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    user_lib::taskinfo();
    0
}
