//! App management syscalls
use crate::task::{exit_current_and_run_next, suspend_current_and_run_next};
use crate::timer::get_time_ms;

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    trace!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// 打印任务信息
pub fn sys_taskinfo() -> isize {
    // print_current_app();
    0
}

/// yield的实现
pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}

/// 获取时间
pub fn sys_get_time() -> isize {
    get_time_ms() as isize
}