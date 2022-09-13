#![no_std]
#![no_main]

extern crate alloc;

extern crate user_lib;
use user_lib::console::getchar;
use user_lib::*;

const LF: u8 = 0x0au8;
const CR: u8 = 0x0du8;

fn func() {
    println!("signal_handler: caught signal SIGINT, and exit(1)");
    exit(1);
}

#[no_mangle]
#[allow(unreachable_code)]
pub fn main() -> i32 {
    println!("sig_ctrlc starting....  Press 'ctrl-c' or 'ENTER'  will quit.");

    println!("sig_ctrlc: signal");
    if signal(SIGINT, func as usize) < 0 {
        panic!("signal failed!");
    }
    println!("sig_ctrlc: getchar....");
    loop {
        let c = getchar();

        println!("Got Char  {}", c);
        if c == LF || c == CR {
            return 0;
        }
    }
    println!("sig_ctrlc: Done");
    0
}
