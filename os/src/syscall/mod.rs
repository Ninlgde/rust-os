//! Implementation of syscalls
//!
//! The single entry point to all system calls, [`syscall()`], is called
//! whenever userspace wishes to perform a system call using the `ecall`
//! instruction. In this case, the processor raises an 'Environment call from
//! U-mode' exception, which is handled as one of the cases in
//! [`crate::trap::trap_handler`].
//!
//! For clarity, each single syscall is implemented as its own function, named
//! `sys_` then the name of the syscall. You can find functions like this in
//! submodules, and you should also implement syscalls this way.

const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_TASKINFO: usize = 22;
const SYSCALL_YIELD: usize = 124;
const SYSCALL_GET_TIME: usize = 169;

mod fs;
mod process;

use fs::*;
use process::*;

/// 根据 `syscall_id` 处理系统调用
pub fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
    match syscall_id {
        // 写操作
        SYSCALL_WRITE => sys_write(args[0], args[1] as *const u8, args[2]),
        // 用户程序退出
        SYSCALL_EXIT => sys_exit(args[0] as i32),
        // 打印任务信息
        SYSCALL_TASKINFO => sys_taskinfo(),
        // yield
        SYSCALL_YIELD => sys_yield(),
        // 获取时间
        SYSCALL_GET_TIME => sys_get_time(),
        // 无法识别的id
        _ => panic!("Unsupported syscall_id: {}", syscall_id),
    }
}
