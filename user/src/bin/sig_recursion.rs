#![no_std]
#![no_main]

extern crate user_lib;

use user_lib::*;

static mut COUNT: i32 = 0;
unsafe fn func() {
    info!("func: user_sig_test succsess count = {}", COUNT);
    COUNT += 1;
    if COUNT < 10 {
        debug!("func: kill signal=SIGUSR2");
        if kill(getpid() as usize, SIGUSR1) < 0 {
            error!("func: Kill failed!");
            exit(1);
        }
        debug!("func: kill return")
    }
    sigreturn();
}

#[no_mangle]
pub fn main() -> i32 {
    warn!("signal_recursion: signal");
    if signal(SIGUSR1, func as usize) < 0 {
        panic!("signal_recursion: signal failed!");
    }
    warn!("signal_recursion: kill signal=SIGUSR1");
    if kill(getpid() as usize, SIGUSR1) < 0 {
        error!("signal_recursion: Kill failed!");
        exit(1);
    }
    warn!("signal_recursion: Done");
    0
}
