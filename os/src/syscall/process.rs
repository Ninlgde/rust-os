//! App management syscalls
use crate::batch::{print_current_app, run_next_app};

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    trace!("[kernel] Application exited with code {}", exit_code);
    run_next_app()
}

pub fn sys_taskinfo() -> isize {
    print_current_app();
    0
}