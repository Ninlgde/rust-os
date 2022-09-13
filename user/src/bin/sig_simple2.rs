#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::{exit, fork, kill, signal, sigreturn, sleep, waitpid};

fn func() {
    println!("user_sig_test succsess");
    sigreturn();
}

#[no_mangle]
pub fn main() -> i32 {
    let pid = fork();
    if pid == 0 {
        println!("signal_simple2: child signal");
        if signal(10, func as usize) < 0 {
            panic!("signal failed!");
        }
        sleep(1000);
        println!("signal_simple2: child done");
        exit(0);
    } else if pid > 0 {
        println!("signal_simple2: parent kill child");
        sleep(500);
        if kill(pid as usize, 1 << 10) < 0 {
            println!("Kill failed!");
            exit(1);
        }
        println!("signal_simple2: parent wait child");
        let mut exit_code = 0;
        waitpid(pid as usize, &mut exit_code);
        println!("signal_simple2: parent Done");
        exit(0);
    }

    0
}
