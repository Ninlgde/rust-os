#![no_std]
#![no_main]

extern crate user_lib;

use user_lib::*;

fn func() {
    println!("user_sig_test succsess");
    sigreturn();
}

#[no_mangle]
pub fn main() -> i32 {
    println!("signal_simple: signal");
    if signal(SIGUSR1, func as usize) < 0 {
        panic!("signal failed!");
    }
    println!("signal_simple: kill");
    if kill(getpid() as usize, SIGUSR1) < 0 {
        println!("Kill failed!");
        exit(1);
    }
    println!("signal_simple: Done");
    0
}
